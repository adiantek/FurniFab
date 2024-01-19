use crate::{python, Error};
use pyo3::types::{PyDict, PyModule};
use serde::Deserialize;

const PA_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/pa.py");
const JOHNSON_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/Johnson.py");
const FILENAME: &str = "file.txt";
const MODULE_NAME: &str = "flow";
const MODULE_FILENAME: &str = "flow.py";

#[derive(Debug, Deserialize)]
pub enum Script {
    Pa,
    Johnson,
}

impl Script {
    fn get_script(&self) -> &'static str {
        match self {
            Script::Pa => PA_SCRIPT,
            Script::Johnson => JOHNSON_SCRIPT,
        }
    }
}

#[tauri::command]
pub fn run_flow() -> Result<String, Error> {
    let script = Script::Pa.get_script();

    Ok(python::with_enhanced_gil(|py, _, file| {
        file.write_to_stdin(FILENAME)?;
        file.write_to_stdin("\n")?;
        file.write_to_file(
            FILENAME.to_string(),
            include_str!("../../algo/F2,rj,pmtn,Cmax/file_pa.txt"),
        )?;
        PyModule::from_code(py, script, MODULE_FILENAME, MODULE_NAME)?;
        let pa = py.import(MODULE_NAME)?;
        let x: &PyDict = pa.getattr("time")?.extract()?;
        let x2: &PyDict = pa.getattr("time2")?.extract()?;
        Ok(format!("time: {x}\ntime2: {x2}"))
    })?)
}
