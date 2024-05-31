// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::RunEvent;

use app::bin_packing::*;
use app::conflicts::*;
use app::data::*;
use app::flow::*;
use app::python3api::Python;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_scheduling_conflicts,
            run_flow,
            run_bin_packing,
            import,
            export,
            load_data,
            save_data,
        ])
        .setup(|app| Python::initialize(app.handle()))
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, event| {
        if let RunEvent::Exit = event {
            Python::finalize().expect("failed to finalize python");
        }
    });
}
