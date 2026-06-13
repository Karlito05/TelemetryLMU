// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod graph_view;
mod telemetry;

use graph_view::*;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;
use telemetry::get_mmap;
use telemetry::TelemetryState;
use graph_view::GraphViewState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .filter(|metadata| !metadata.target().starts_with("tracing"))
                .filter(|metadata| !metadata.target().starts_with("tauri"))
                .filter(|metadata| !metadata.target().starts_with("wry"))
                .filter(|metadata| metadata.target() != "telemetry_lmu_lib")
                .build(),
        )
        .setup(|app| {
            let backend = Mutex::new(TelemetryState {
                full_mode: false,
            });

            let mmap = get_mmap("/dev/shm/LMU_Data", &backend);

            app.manage(backend);

            app.manage(MmapState {
                mmap: Arc::new(mmap),
            });
            app.manage(Mutex::new(GraphViewState{threads: Vec::new(), current_driver: 0}));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // .plugin(tauri_plugin_devtools::init())
        .invoke_handler(tauri::generate_handler![
            lap_data_subscribe,
            lap_data_unsubscribe,
            get_drivers,
            set_car_num
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
