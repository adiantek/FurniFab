use crate::Error;
use scheduling_conflicts::{schedulers, Instance};

#[tauri::command]
pub fn run_scheduling_conflicts(instance: Instance) -> Result<String, Error> {
    let schedule = schedulers::list_algorithm(&instance);
    serde_json::to_string(&schedule).map_err(Error::from)
}
