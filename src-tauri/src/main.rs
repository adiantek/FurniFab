// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use tauri::api::process::{Command, CommandEvent};
use tauri::AppHandle;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Default, Serialize)]
struct CommandOutput {
    stdout: String,
    stderr: String,
    error: Vec<String>,
}

#[tauri::command]
async fn run_resource(handle: AppHandle, exec: String, stdin: String) -> Result<CommandOutput> {
    let resource = handle
        .path_resolver()
        .resolve_resource(exec)
        .ok_or("failed to resolve resource")?;

    let resource_parent = resource.parent().ok_or("failed to get resource dir")?;

    let program = resource
        .to_str()
        .ok_or("cannot convert resource path to string")?;

    let (mut events, mut child) = Command::new(program)
        .current_dir(resource_parent.into())
        .spawn()
        .map_err(|err| err.to_string())?;

    child
        .write(stdin.as_bytes())
        .map_err(|err| err.to_string())?;

    let mut output = CommandOutput::default();

    while let Some(event) = events.recv().await {
        match event {
            CommandEvent::Stderr(line) => output.stderr.push_str(&line),
            CommandEvent::Stdout(line) => output.stdout.push_str(&line),
            CommandEvent::Error(err) => output.error.push(err),
            CommandEvent::Terminated(_) => {}
            _ => unreachable!(),
        }
    }

    Ok(output)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_resource])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
