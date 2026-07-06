// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod graph_view;
mod lap_stores;
mod telemetry;

use graph_view::GraphViewState;
use graph_view::*;
use lap_stores::{despawn_logger, spawn_logger};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;
use telemetry::get_mmap;
use telemetry::TelemetryState;

use crate::lap_stores::LoggerSate;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .filter(|metadata| !metadata.target().starts_with("tracing"))
                .filter(|metadata| !metadata.target().starts_with("tauri"))
                .filter(|metadata| !metadata.target().starts_with("wry"))
                .filter(|metadata| metadata.target() != "telemetry_lmu_lib")
                .build(),
        )
        .setup(|app| {
            let backend = Mutex::new(TelemetryState { full_mode: false });

            let mmap = get_mmap("/dev/shm/LMU_Data", &backend);

            app.manage(backend);
            app.manage(Mutex::new(LoggerSate { loggers: vec![] }));

            app.manage(MmapState {
                mmap: Arc::new(mmap),
            });
            app.manage(Mutex::new(GraphViewState {
                threads: Vec::new(),
            }));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // .plugin(tauri_plugin_devtools::init())
        .invoke_handler(tauri::generate_handler![
            lap_data_subscribe,
            lap_data_unsubscribe,
            get_drivers,
            spawn_logger,
            despawn_logger,
            get_lap_data,
            get_telemetry_info,
            was_last_best,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
