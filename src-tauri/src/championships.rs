use crate::{
    graph_view::MmapState,
    telemetry::{i8_array32_to_string, i8_array64_to_string, update_telemetry, TelemetryState},
};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, serde::Serialize, Clone)]
pub struct ChampDriverData {
    name: String,
    car: String,
    car_class: String,
    id: i32,
}

#[tauri::command]
pub async fn get_champ_drivers(
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    mmap: State<'_, MmapState>,
) -> Result<Vec<ChampDriverData>, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Telemetry is not in full mode!".into());
    }

    let telemetry = update_telemetry(&mmap.mmap).unwrap();
    let mut drivers: Vec<ChampDriverData> = Vec::new();
    for i in 0..103 {
        let name = i8_array32_to_string(&telemetry.scoring.veh_scoring_info[i].m_driver_name);
        let car = i8_array64_to_string(&telemetry.scoring.veh_scoring_info[i].m_vehicle_name);
        let car_class =
            i8_array32_to_string(&telemetry.scoring.veh_scoring_info[i].m_vehicle_class);
        if name != "" {
            drivers.push(ChampDriverData {
                name,
                car,
                car_class,
                id: i as i32,
            });
        }
    }
    Ok(drivers)
}
