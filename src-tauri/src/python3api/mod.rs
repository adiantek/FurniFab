use core::ffi;
use std::{
    ffi::{CStr, CString},
    os::raw::c_void,
    process,
    ptr::null_mut,
};

use crate::Error;

use self::bindings::*;

mod bindings;

static mut THREAD_STATE: ffi::c_ulonglong = 0;

pub fn run_python_init(app: tauri::AppHandle) {
    let resource_path: std::path::PathBuf = app
        .path_resolver()
        .resolve_resource("resources/python.zip.enc")
        .expect("failed to resolve resource");
    let path_str = CString::new(resource_path.to_str().unwrap()).unwrap();
    let status: bindings::PyStatus = unsafe { python3api_init(path_str.as_ptr(), 0, null_mut()) };
    if status._type != PyStatus__PyStatus_TYPE_OK {
        let err_msg = unsafe { CStr::from_ptr(status.err_msg).to_str().unwrap() };
        println!("Python3 API init err_msg: {:?}", err_msg);
        if status._type == PyStatus__PyStatus_TYPE_EXIT {
            println!("Python3 API init exitcode: {:?}", status.exitcode);
            process::exit(status.exitcode as i32);
        }
    }
    unsafe { THREAD_STATE = python3api_save_thread() as ffi::c_ulonglong };
}

pub fn run_python_finalize() {
    unsafe {
        let thread_state = THREAD_STATE as *mut c_void;
        python3api_restore_thread(thread_state);
    }
    let status: i32 = unsafe { bindings::python3api_finalize() };
    if status != 0 {
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
        None
    }
}

fn python_eval(code: &str, start: i32) -> Result<String, Error> {
    unsafe {
        let module_name = CString::new("__api__").unwrap();
        let code_raw = CString::new(code).unwrap();
        let res = python3api_eval(module_name.as_ptr(), code_raw.as_ptr(), start);
        if res.is_null() {
            Err(Error::Python("failed".to_owned()))
        } else {
            let result = CStr::from_ptr(res).to_str().unwrap().to_string();
            python3api_free(res);
            Ok(result)
        }
    }
}

fn eval_in_python_thread(code: &str, function: &str, input_data: &str) -> Result<String, Error> {
    let result = python_clear();
    if result.is_some() {
        return Err(result.unwrap());
    }
    let result = python_eval("import json", Py_file_input)?;
    if result != "None" {
        return Err(Error::Python("Failed to load json".to_string()));
    }
    let result = python_eval(code, Py_file_input)?;
    if result != "None" {
        return Err(Error::Python("Failed to load code".to_string()));
    }
    let query = format!("json.dumps({}({}))", function, input_data);
    python_eval(&query, Py_eval_input)
}

pub fn eval_python(code: &str, function: &str, input_data: &str) -> Result<String, Error> {
    let state = unsafe { python3api_ensure_gil() };
    let result = eval_in_python_thread(code, function, input_data);
    unsafe { python3api_release_gil(state) };
    return result;
}
