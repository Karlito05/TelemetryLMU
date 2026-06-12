// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod graph_view;
mod telemetry;

use graph_view::*;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;
use telemetry::get_mmap;
use telemetry::BackendState;

extern crate pretty_env_logger;
#[macro_use] extern crate log;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    pretty_env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_devtools::init())
        .setup(|app| {
            let backend = Mutex::new(BackendState {
                full_mode: false,
                threads: Vec::new(),
            });

            let mmap = get_mmap("/dev/shm/LMU_Data", &backend);

            app.manage(backend);

            app.manage(MmapState {
                mmap: Arc::new(mmap),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            lap_data_subscribe,
            lap_data_unsubscribe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
