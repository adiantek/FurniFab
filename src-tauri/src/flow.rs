use crate::{python, Error};
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyDict, PyList, PyModule};
use pyo3::PyErr;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Deserialize)]
pub struct Task {
    start_time: u64,
    grinding_time: u64,
    lacquering_time: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScheduleInfo {
    start_time: u64,
    end_time: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Schedule {
    grinding: Vec<Vec<ScheduleInfo>>,
    lacquering: Vec<Vec<ScheduleInfo>>,
}

fn parse_schedule_infos(dict: &PyDict) -> Result<Vec<Vec<ScheduleInfo>>, PyErr> {
    let mut infos: Vec<Vec<ScheduleInfo>> = vec![Vec::new(); dict.len()];

    for (key, value) in dict {
        let task_schedules = &mut infos[key.extract::<usize>()? - 1];
        for timings in value.extract::<&PyList>()? {
            let mut timings = timings.extract::<&PyList>()?.into_iter();
            task_schedules.push(ScheduleInfo {
                start_time: timings
                    .next()
                    .ok_or(PyValueError::new_err("missing start time"))?
                    .extract()?,
                end_time: timings
                    .next()
                    .ok_or(PyValueError::new_err("missing end time"))?
                    .extract()?,
            })
        }
    }

    Ok(infos)
}

#[tauri::command]
pub async fn run_flow(tasks: Vec<Task>) -> Result<Schedule, Error> {
    let script = Script::Pa.get_script();

    Ok(python::with_enhanced_gil(|py, _, file| {
        file.write_to_stdin(FILENAME)?;
        file.write_to_stdin("\n")?;

        for task in &tasks {
            file.write_to_file(
                FILENAME.to_string(),
                &format!(
                    "{} {} {}",
                    task.start_time, task.grinding_time, task.lacquering_time
                ),
            )?;
        }

        PyModule::from_code(py, script, MODULE_FILENAME, MODULE_NAME)?;

        let module = py.import(MODULE_NAME)?;

        let grinding = parse_schedule_infos(module.getattr("time")?.extract()?)?;
        let lacquering = parse_schedule_infos(module.getattr("time2")?.extract()?)?;

        Ok(Schedule {
            grinding,
            lacquering,
        })
    })?)
}
