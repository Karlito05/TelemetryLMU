// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod graph_view;
mod telemetry;

use graph_view::{MmapState, get_context, get_values, get_lap, is_last_best};
use telemetry::get_mmap;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(MmapState {
                mmap: get_mmap("/dev/shm/LMU_Data")
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_context, get_values, get_lap, is_last_best])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
