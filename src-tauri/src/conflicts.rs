use crate::Error;
use scheduling_conflicts::{schedulers, Instance};

#[tauri::command]
pub fn run_scheduling_conflicts(instance: Instance) -> Result<String, Error> {
    let schedule = schedulers::list_algorithm(&instance);
    let score = schedule.calculate_score();
    serde_json::to_string(&(schedule, score)).map_err(Error::from)
}
