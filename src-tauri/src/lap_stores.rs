use crate::{
    graph_view::{GraphViewDataType, MmapState, GRAPH_VIEW_DATA_TYPE_COUNT},
    telemetry::update_telemetry,
};
use chrono::Local;
use log::info;
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::{
    async_runtime::{spawn, JoinHandle},
    State,
};
use tokio::time::{sleep, Duration};

pub struct LoggerSate {
    pub loggers: Vec<JoinHandle<()>>,
}

#[derive(Debug, Clone, serde::Serialize)]
struct SaveData {
    lap_info: LapInfo,
    data: Vec<LapData>,
}
#[derive(Debug, Clone, serde::Serialize)]
struct LapInfo {
    lap_time: f64,
    date: String,
}
#[derive(Debug, Clone, serde::Serialize)]
struct LapData {
    data_type: String,
    data: Vec<DataPoint>, //values + distance
}
#[derive(Debug, Clone, serde::Serialize)]
struct DataPoint {
    values: Vec<f64>,
    distance: f64,
}
#[tauri::command]
pub async fn spawn_logger(
    mmap: State<'_, MmapState>,
    logger_state: State<'_, Mutex<LoggerSate>>,
    car_num: usize,
) -> Result<(), String> {
    // Init the thread
    let mmap_clone = Arc::clone(&mmap.mmap);
    let handle = spawn(async move {
        // Main Loop
        loop {
            let mut save_data: SaveData = SaveData {
                lap_info: LapInfo {
                    lap_time: -1.0,
                    date: "".to_string(),
                },
                data: vec![],
            };
            let cur_lap = update_telemetry(&mmap_clone)
                .unwrap()
                .telemetry
                .telemetry_info[car_num]
                .m_lap_number;

            // Init the save data with correct data types but not values
            for i in 0..GRAPH_VIEW_DATA_TYPE_COUNT {
                let graph_type = GraphViewDataType::from_int(i, car_num);

                save_data.data.push(LapData {
                    data_type: graph_type.to_string(),
                    data: vec![],
                });
            }

            // Fill in the values untill end of the lap
            loop {
                let telemetry = update_telemetry(&mmap_clone).unwrap();

                // Handle new lap
                if cur_lap != telemetry.telemetry.telemetry_info[car_num].m_lap_number {
                    // Add some delay so the tele has time to refresh
                    sleep(Duration::from_millis(100)).await;
                    let telemetry = update_telemetry(&mmap_clone).unwrap();

                    save_data.lap_info.date = Local::now().format("%d-%m-%Y-%H-%M-%S").to_string();

                    save_data.lap_info.lap_time =
                        telemetry.scoring.veh_scoring_info[car_num].m_last_lap_time;
                    break;
                };

                for i in 0..GRAPH_VIEW_DATA_TYPE_COUNT {
                    let graph_type = GraphViewDataType::from_int(i, car_num);
                    // FIX THIS
                    // if graph_type.to_string() == "delta".to_owned() {
                    //     continue;
                    // };
                    save_data.data[i as usize].data.push(DataPoint {
                        values: graph_type.get_normalized_values(&telemetry),
                        distance: graph_type.get_normalized_distance(&telemetry),
                    })
                }

                sleep(Duration::from_millis(16)).await;
            }

            if save_data.lap_info.lap_time != -1.0 {
                fs::write(
                    Local::now().format("%d-%m-%Y-%H-%M-%S").to_string() + ".json",
                    serde_json::to_string_pretty(&save_data).unwrap(),
                )
                .unwrap();
            }
        }
    });
    info!("Spawned logger");

    logger_state.lock().unwrap().loggers.push(handle);
    Ok(())
}

#[tauri::command]
pub async fn despawn_logger(logger_state: State<'_, Mutex<LoggerSate>>) -> Result<(), String> {
    for handle in &logger_state.lock().unwrap().loggers {
        handle.abort()
    }
    logger_state.lock().unwrap().loggers = vec![];
    Ok(())
}
