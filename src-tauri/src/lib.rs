// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod graph_view;
mod telemetry;

use graph_view::*;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;
use telemetry::get_mmap;

pub struct BackendState {
    pub full_mode: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_devtools::init())
        .setup(|app| {
            let backend = Mutex::new(BackendState { full_mode: false });

            let mmap = get_mmap("/dev/shm/LMU_Data", &backend);

            app.manage(backend);

            app.manage(MmapState {
                mmap: Arc::new(mmap),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![lap_data_subscribe,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
