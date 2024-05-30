use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::AppHandle;

use crate::Error;

const FILENAME: &str = "data.json";

#[tauri::command]
pub async fn import() -> Result<Option<String>, Error> {
    load_path(FileDialogBuilder::new().pick_file())
}

#[tauri::command]
pub async fn export(data: String) -> Result<(), Error> {
    save_path(FileDialogBuilder::new().save_file(), data)
}

#[tauri::command]
pub async fn load_data(handle: AppHandle) -> Result<Option<String>, Error> {
    load_path(handle.path_resolver().resolve_resource(FILENAME))
}

#[tauri::command]
pub async fn save_data(handle: AppHandle, data: String) -> Result<(), Error> {
    save_path(handle.path_resolver().resolve_resource(FILENAME), data)
}

fn load_path(file: Option<PathBuf>) -> Result<Option<String>, Error> {
    if let Some(file) = file.filter(|path| path.exists()) {
        let mut data = String::new();
        File::open(file.as_path())?.read_to_string(&mut data)?;
        Ok(Some(data))
    } else {
        Ok(None)
    }
}

fn save_path(file: Option<PathBuf>, data: String) -> Result<(), Error> {
    if let Some(file) = file {
        File::create(file.as_path())?.write_all(data.as_bytes())?;
    }
    Ok(())
}
