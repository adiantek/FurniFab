use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::python3api::Python;
use crate::Error;

const PA_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/pa.py");
const JOHNSON_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/Johnson.py");
const JOHNSON_2_SCRIPT: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/Johnson_ver2.py");
const BRANCH_AND_BOUND: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/BB.py");
const NEH: &str = include_str!("../../algo/F2,rj,pmtn,Cmax/neh2.py");

#[derive(Debug, Deserialize)]
pub enum Script {
    Pa,
    Johnson,
    Johnson2,
    BranchAndBound,
    Neh,
}

impl Script {
    fn script(&self) -> &'static str {
        match self {
            Script::Pa => PA_SCRIPT,
            Script::Johnson => JOHNSON_SCRIPT,
            Script::Johnson2 => JOHNSON_2_SCRIPT,
            Script::BranchAndBound => BRANCH_AND_BOUND,
            Script::Neh => NEH,
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
    result_1: HashMap<String, Vec<[u64; 2]>>,
    result_2: HashMap<String, Vec<[u64; 2]>>,
}

fn parse_schedule_infos(
    task_schedules: HashMap<String, Vec<[u64; 2]>>,
) -> Result<Vec<Vec<ScheduleInfo>>, Error> {
    let mut infos: Vec<Vec<ScheduleInfo>> = vec![Vec::new(); task_schedules.len()];

    for (key, value) in task_schedules {
        let key: usize = key.parse()?;
        let task_schedules = &mut infos[key - 1];
        for timings in value {
            let [start_time, end_time] = timings;
            task_schedules.push(ScheduleInfo {
                start_time,
                end_time,
            })
        }
    }

    Ok(infos)
}

#[tauri::command]
pub async fn run_flow(tasks: Vec<Task>, script: Script) -> Result<Schedule, Error> {
    let data: Vec<[u64; 3]> = tasks
        .into_iter()
        .map(|task| [task.start_time, task.grinding_time, task.lacquering_time])
        .collect();

    let output: ScheduleRawResult = Python::eval_with_gil(script.script(), "run_algorithm", data)?;

    let grinding = parse_schedule_infos(output.result_1)?;
    let lacquering = parse_schedule_infos(output.result_2)?;

    Ok(Schedule {
        grinding,
        lacquering,
    })
}
