use crate::Error;
use scheduling_conflicts::{schedulers, Instance, Schedule, Scheduler};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Algorithm {
    List,
    VNS,
}

impl Scheduler for Algorithm {
    fn schedule(self, instance: &Instance) -> Schedule {
        match self {
            Algorithm::List => schedulers::list_algorithm(instance),
            Algorithm::VNS => schedulers::vns(instance),
        }
    }
}

#[tauri::command]
pub async fn run_scheduling_conflicts(
    instance: Instance,
    algorithm: Algorithm,
) -> Result<String, Error> {
    let schedule = algorithm.schedule(&instance);
    if !schedule.verify() {
        Err(Error::InvalidSchedule)
    } else {
        serde_json::to_string(&schedule).map_err(Error::from)
    }
}
