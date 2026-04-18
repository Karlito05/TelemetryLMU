// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod graph_view;
mod telemetry;

use graph_view::*;
use std::sync::Arc;
use tauri::Manager;
use telemetry::get_mmap;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_devtools::init())
        .setup(|app| {
            app.manage(MmapState {
                mmap: Arc::new(get_mmap("/dev/shm/LMU_Data")),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_context,
            get_values,
            get_lap,
            is_last_best,
            lap_data_subscribe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
