use std::collections::HashMap;

use crate::Error;
use serde::{Deserialize, Serialize};

use crate::python3api;

const PA_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/pa.py");
const JOHNSON_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/Johnson.py");
const JOHNSON_2_SCRIPT: &str = include_str!("../../F2,rj,pmtn,Cmax/Johnson_ver2.py");
const BRANCH_AND_BOUND: &str = include_str!("../../F2,rj,pmtn,Cmax/BB.py");

#[derive(Debug, Deserialize)]
pub enum Script {
    Pa,
    Johnson,
    Johnson2,
    BranchAndBound,
}

impl Script {
    fn script(&self) -> &'static str {
        match self {
            Script::Pa => PA_SCRIPT,
            Script::Johnson => JOHNSON_SCRIPT,
            Script::Johnson2 => JOHNSON_2_SCRIPT,
            Script::BranchAndBound => BRANCH_AND_BOUND,
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

#[derive(Clone, Debug, Deserialize)]
pub struct ScheduleRawResult {
    result_1: HashMap<String, Vec<Vec<u64>>>,
    result_2: HashMap<String, Vec<Vec<u64>>>,
}

fn parse_schedule_infos(task_schedules: HashMap<String, Vec<Vec<u64>>>) -> Result<Vec<Vec<ScheduleInfo>>, Error> {
    let mut infos: Vec<Vec<ScheduleInfo>> = vec![Vec::new(); task_schedules.len()];

    for (key, value) in task_schedules.iter() {
        let task_schedules = &mut infos[key.parse::<usize>().map_err(|_| Error::Serde("invalid number".to_string()))? - 1];
        for timings in value {
            let mut timings = timings.iter();
            task_schedules.push(ScheduleInfo {
                start_time: *timings
                    .next()
                    .ok_or(Error::Serde("missing start time".to_string()))?,
                end_time: *timings
                    .next()
                    .ok_or(Error::Serde("missing end time".to_string()))?,
            })
        }
    }

    Ok(infos)
}

#[tauri::command]
pub async fn run_flow(
    app_handle: tauri::AppHandle,
    tasks: Vec<Task>,
    script: Script,
) -> Result<Schedule, Error> {
    let mut input_data = vec![vec![0; 3]; tasks.len()];
    for (i, task) in tasks.iter().enumerate() {
        input_data[i][0] = task.start_time as i32;
        input_data[i][1] = task.grinding_time as i32;
        input_data[i][2] = task.lacquering_time as i32;
    }
    let input_data_str = serde_json::to_string(&input_data).map_err(Error::from);
    if input_data_str.is_err() {
        return Err(input_data_str.unwrap_err());
    }
    let input_data_str = input_data_str.unwrap();

    let output = python3api::eval_python(
        app_handle,
        script.script(),
        "run_algorithm",
        &input_data_str,
    );
    if output.is_err() {
        return Err(output.unwrap_err());
    }
    let output = output.unwrap();

    let output: Result<ScheduleRawResult, Error> = serde_json::from_str(&output).map_err(Error::from);
    if output.is_err() {
        return Err(output.unwrap_err());
    }
    let output = output.unwrap();

    let grinding = parse_schedule_infos(output.result_1);
    if grinding.is_err() {
        return Err(grinding.unwrap_err());
    }
    let grinding = grinding.unwrap();
    let lacquering = parse_schedule_infos(output.result_2);
    if lacquering.is_err() {
        return Err(lacquering.unwrap_err());
    }
    let lacquering = lacquering.unwrap();
    Ok(Schedule {
        grinding,
        lacquering,
    })
}
