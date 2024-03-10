use std::{
    ffi::{CStr, CString},
    process,
    ptr::null_mut,
};

use crate::Error;

use self::bindings::*;

mod bindings;

fn run_python_init(app: tauri::AppHandle) {
    let resource_path: std::path::PathBuf = app
        .path_resolver()
        .resolve_resource("resources/python.zip.enc")
        .expect("failed to resolve resource");
    let path_str = CString::new(resource_path.to_str().unwrap()).unwrap();
    let status: bindings::PyStatus = unsafe { python3api_init(path_str.as_ptr(), 0, null_mut()) };
    if status._type == PyStatus__PyStatus_TYPE_OK {
        println!("Python3 API init success");
    } else {
        let err_msg = unsafe { CStr::from_ptr(status.err_msg).to_str().unwrap() };
        println!("Python3 API init err_msg: {:?}", err_msg);
        if status._type == PyStatus__PyStatus_TYPE_EXIT {
            println!("Python3 API init exitcode: {:?}", status.exitcode);
            process::exit(status.exitcode as i32);
        }
    }
}

pub fn run_python_finalize() {
    let status: i32 = unsafe { bindings::python3api_finalize() };
    if status == 0 {
        println!("Python3 API finalize success");
    } else {
        println!("Python3 API finalize status: {:?}", status);
    }
}

fn python_clear() -> Option<Error> {
    unsafe {
        let module_name = CString::new("__api__").unwrap();
        if !python3api_clear(module_name.as_ptr()) {
            println!("Failed to clear python module");
            return Some(Error::Python("Failed to clear python module".to_string()));
        }
        return None;
    }
}

fn python_eval(code: &str, start: i32) -> Result<String, Error> {
    unsafe {
        let module_name = CString::new("__api__").unwrap();
        let code_raw = CString::new(code).unwrap();
        let res = python3api_eval(module_name.as_ptr(), code_raw.as_ptr(), start);
        if res.is_null() {
            return Err(Error::Python("failed".to_owned()));
        } else {
            let result = CStr::from_ptr(res).to_str().unwrap().to_string();
            python3api_free(res);
            return Ok(result);
        }
    }
}

pub fn eval_python(
    app_handle: tauri::AppHandle,
    code: &str,
    function: &str,
    input_data: &str,
) -> Result<String, Error> {
    run_python_init(app_handle);
    let result = python_clear();
    if result.is_some() {
        return Err(result.unwrap());
    }
    let result = python_eval("import json", Py_file_input);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let result = result.unwrap();
    if result != "None" {
        return Err(Error::Python("Failed to load json".to_string()));
    }
    let result = python_eval(code, Py_file_input);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let result = result.unwrap();
    if result != "None" {
        return Err(Error::Python("Failed to load code".to_string()));
    }
    let query = format!("json.dumps({}({}))", function, input_data);
    let result = python_eval(&query, Py_eval_input);
    run_python_finalize();
    return result;
}
