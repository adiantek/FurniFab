use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::AppHandle;

use crate::Error;

use self::bindings::*;

mod bindings;

static API_STR: &CStr = match CStr::from_bytes_with_nul(b"__api__\0") {
    Ok(value) => value,
    Err(_) => panic!("invalid CStr"),
};
static PYTHON: OnceLock<Python> = OnceLock::new();

/// Represents global python state.
pub struct Python {
    callback: *mut c_void,
    finalized: AtomicBool,
}

/// It's safe to assume that Python is Send, because we manage
/// its initialization and finalization with thread safety in mind.
unsafe impl Send for Python {}

/// It's safe to assume that Python is Sync, because we manage
/// its initialization and finalization with thread safety in mind.
unsafe impl Sync for Python {}

impl Python {
    /// Initializes the global python state.
    /// This function should be called only once,
    /// but it's safe to call it from multiple threads multiple times.
    ///
    /// Panics if python initialization fails.
    pub fn initialize(handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        if PYTHON.get().is_some() {
            return Ok(());
        }

        let path = handle
            .path_resolver()
            .resolve_resource("resources/python.zip.enc")
            .ok_or("resource not found")?;
        let path_str = CString::new(path.to_str().ok_or("path is not valid UTF-8 string")?)?;

        PYTHON.get_or_init(|| {
            let status = unsafe { python3api_init(path_str.as_ptr(), 0, std::ptr::null_mut()) };

            #[allow(non_upper_case_globals)]
            match status._type {
                PyStatus__PyStatus_TYPE_EXIT => {
                    panic!("Python3API init exit code: {}", status.exitcode)
                }
                PyStatus__PyStatus_TYPE_OK => Self {
                    callback: unsafe { python3api_save_thread() },
                    finalized: AtomicBool::new(false),
                },
                _ => panic!("Python3API error: {}", unsafe {
                    CStr::from_ptr(status.err_msg)
                        .to_str()
                        .expect("Python3API: invalid error message")
                }),
            }
        });

        Ok(())
    }

    /// Finalizes the global python state.
    /// This function should be called only once,
    /// but it's safe to call it from multiple threads multiple times.
    /// If python wasn't initialized, this function does nothing.
    ///
    /// Returns an error if python finalization fails.
    pub fn finalize() -> Result<(), i32> {
        if let Some(python) = PYTHON.get() {
            if !python.finalized.swap(true, Ordering::Relaxed) {
                let status = unsafe {
                    python3api_restore_thread(python.callback);
                    python3api_finalize()
                };

                if status != 0 {
                    return Err(status);
                }
            }
        }

        Ok(())
    }

    /// Acquires the python global interpreter lock.
    ///
    /// Panics if python wasn't initialized or if it was finalized.
    pub fn acquire_gil() -> PythonGIL {
        if PYTHON.get().filter(|state| !state.is_finalized()).is_none() {
            panic!("python not available")
        }

        PythonGIL::acquire()
    }

    /// Evaluates python code and calls a function with the given data.
    /// This function acquires the global interpreter lock for the duration of the call.
    pub fn eval_with_gil<I, O>(code: &str, function: &str, data: I) -> Result<O, Error>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        Self::acquire_gil().eval(code, function, data)
    }

    fn is_finalized(&self) -> bool {
        self.finalized.load(Ordering::Relaxed)
    }
}

/// Represents a global interpreter lock.
pub struct PythonGIL(*mut c_void);

impl PythonGIL {
    fn acquire() -> Self {
        Self(unsafe { python3api_ensure_gil() })
    }

    /// Evaluates python code and calls a function with the given data.
    pub fn eval<I, O>(&self, code: &str, function: &str, data: I) -> Result<O, Error>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        unsafe { python3api_clear(API_STR.as_ptr()) }
            .then_some(())
            .ok_or(Error::Python("failed to clear python module".into()))?;

        if Self::eval_impl("import json", Py_file_input)? != "None" {
            return Err(Error::Python("failed to load json".into()));
        }

        if Self::eval_impl(code, Py_file_input)? != "None" {
            return Err(Error::Python("failed to load code".into()));
        }

        let code = format!("json.dumps({function}({}))", serde_json::to_string(&data)?);
        let result = Self::eval_impl(&code, Py_eval_input)?;
        Ok(serde_json::from_str(&result)?)
    }

    fn eval_impl(code: &str, start: i32) -> Result<String, Error> {
        let code = CString::new(code).map_err(|_| Error::Python("invalid code".into()))?;
        let result_ref = unsafe { python3api_eval(API_STR.as_ptr(), code.as_ptr(), start) };

        (!result_ref.is_null())
            .then_some(())
            .ok_or(Error::Python("running python failed".into()))?;

        let result = unsafe { CStr::from_ptr(result_ref) }
            .to_str()
            .map_err(|_| Error::Python("invalid result string".into()))?
            .to_string();

        unsafe { python3api_free(result_ref) };

        Ok(result)
    }
}

impl Drop for PythonGIL {
    fn drop(&mut self) {
        unsafe { python3api_release_gil(self.0) }
    }
}
