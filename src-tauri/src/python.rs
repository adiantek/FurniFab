use parking_lot::Mutex;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyCFunction, PyDict, PyTuple};
use pyo3::{pyclass, pymethods, Py, PyAny, PyErr, PyRef, PyResult, Python};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

#[pyclass(name = "__HijackedFile")]
#[derive(Clone, Debug, Default)]
struct HijackedFile {
    input_lines: Arc<Mutex<VecDeque<String>>>,
    output: Arc<Mutex<String>>,
}

#[pymethods]
impl HijackedFile {
    fn __iter__(py_self: PyRef<Self>) -> PyResult<Py<HijackedFile>> {
        Ok(py_self.into())
    }

    fn __next__(py_self: PyRef<Self>) -> PyResult<Option<String>> {
        Ok(py_self.input_lines.lock().pop_front())
    }

    fn __enter__(py_self: PyRef<Self>) -> PyResult<Py<HijackedFile>> {
        Ok(py_self.into())
    }

    #[pyo3(signature = (*_args))]
    fn __exit__(_py_self: PyRef<Self>, _args: &PyTuple) -> PyResult<()> {
        Ok(())
    }

    fn write(py_self: PyRef<Self>, input: &PyAny) -> PyResult<()> {
        let input: String = input.extract()?;
        *py_self.output.lock() += &input;
        Ok(())
    }
}

/// This struct manages hijacked python files.
/// Provides read and write access to files read and written from python.
#[derive(Clone, Debug, Default)]
pub struct FileHijacker {
    stdin_lines: Arc<Mutex<VecDeque<String>>>,
    files: Arc<Mutex<HashMap<String, HijackedFile>>>,
}

impl FileHijacker {
    /// Get output of file with given filename.
    pub fn _get_file_output(&self, filename: &str) -> PyResult<String> {
        let file_map = self.files.lock();
        let file = file_map
            .get(filename)
            .ok_or(PyValueError::new_err("file not found"))?;
        let mut value = file.output.lock();
        let mut output = String::new();
        std::mem::swap(&mut *value, &mut output);
        Ok(output)
    }

    /// Adds input to file with given filename.
    pub fn write_to_file(&self, name: String, input: &str) -> PyResult<()> {
        let mut map = self.files.lock();
        let values = match map.entry(name) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(HijackedFile::default()),
        };
        let mut x = values.input_lines.lock();
        Self::write(&mut x, input)
    }

    /// Adds input to stdin.
    /// Remember to add new line character at the end if needed.
    pub fn write_to_stdin(&self, input: &str) -> PyResult<()> {
        let mut stdin = self.stdin_lines.lock();
        Self::write(&mut stdin, input)
    }

    fn write(file: &mut VecDeque<String>, input: &str) -> PyResult<()> {
        input.split('\n').map(|s| s.to_string()).for_each(|s| {
            file.push_back(s);
        });
        Ok(())
    }
}

/// Access to hijacked stdout from python.
#[derive(Clone, Debug, Default)]
#[pyclass(name = "__HijackedPrint")]
pub struct HijackedStdout {
    output: Arc<Mutex<String>>,
}

impl HijackedStdout {
    /// Gets buffered stdout from python.
    /// Returns empty string when nothing new has been printed.
    pub fn get_stdout(&self) -> PyResult<String> {
        let mut output = String::new();
        std::mem::swap(&mut *self.output.lock(), &mut output);
        Ok(output)
    }
}

#[pymethods]
impl HijackedStdout {
    #[pyo3(signature = (*args, **dict))]
    fn __call__(py_self: PyRef<Self>, args: &PyTuple, dict: Option<&PyDict>) -> PyResult<()> {
        let mut end: Option<String> = None;
        let mut sep: Option<String> = None;
        let mut file: Option<PyRef<HijackedFile>> = None;
        if let Some(dict) = dict {
            for (key, value) in dict {
                let key: String = key.extract()?;
                if key == "end" {
                    end = Some(value.extract()?);
                } else if key == "sep" {
                    sep = Some(value.extract()?);
                } else if key == "file" {
                    file = Some(value.extract()?);
                }
            }
        }
        let end = end.unwrap_or('\n'.to_string());
        let sep = sep.unwrap_or(' '.to_string());
        let mut output = if let Some(file) = &file {
            file.output.lock()
        } else {
            py_self.output.lock()
        };

        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                *output += &sep;
            }
            *output += &arg.to_string();
        }
        *output += &end;
        Ok(())
    }
}

fn create_open(
    file_injector: FileHijacker,
) -> impl Fn(&PyTuple, Option<&PyDict>) -> PyResult<HijackedFile> + Send + 'static {
    move |args: &PyTuple, _: Option<&PyDict>| {
        let name: String = args.get_item(0)?.extract()?;
        let mut map = file_injector.files.lock();
        let file = match map.entry(name) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(HijackedFile::default()),
        };
        Ok(file.clone())
    }
}

fn create_input(
    stdin: Arc<Mutex<VecDeque<String>>>,
) -> impl Fn(&PyTuple, Option<&PyDict>) -> PyResult<String> + Send + 'static {
    move |_: &PyTuple, _: Option<&PyDict>| {
        let mut stdin = stdin.lock();
        stdin.pop_front().ok_or(PyValueError::new_err("EOF"))
    }
}

pub fn with_enhanced_gil<F, R>(f: F) -> Result<R, PyErr>
where
    F: for<'py> FnOnce(Python<'py>, HijackedStdout, FileHijacker) -> Result<R, PyErr>,
{
    Python::with_gil(move |py| {
        let builtins = py.import("builtins")?;
        let print = HijackedStdout::default();
        let file = FileHijacker::default();

        builtins.setattr("print", Py::new(py, print.clone())?)?;
        builtins.setattr(
            "open",
            PyCFunction::new_closure(py, None, None, create_open(file.clone()))?,
        )?;
        builtins.setattr(
            "input",
            PyCFunction::new_closure(py, None, None, create_input(file.stdin_lines.clone()))?,
        )?;

        f(py, print, file)
    })
}
