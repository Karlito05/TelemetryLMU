use crate::telemetry::{update_telemetry, SharedMemoryObjectOut};
use memmap2::Mmap;
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use tauri::{ipc::Channel, State};
use tokio::time::sleep;

#[derive(Clone, Serialize)]
pub struct DataPointResponse {
    pub values: Vec<f64>,
    pub distance: f64,
}

// TODO: Implement error handling, Send deserialized output to keep it neat

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum LapEvent {
    RenderingData { max_value: f64, unit: String },
    LapDataPoint { values: Vec<f64>, distance: f64 },
    LapFinished { was_best: bool },
}

#[tauri::command]
pub fn lap_data_subscribe(
    state: State<'_, MmapState>,
    tele_type: String,
    car_num: usize,
    on_event: Channel<LapEvent>,
) {
    let mmap_clone = Arc::clone(&state.mmap);
    tauri::async_runtime::spawn(async move {
        let telemetry = update_telemetry(&mmap_clone)
            .ok_or_else(|| "TelemetryReadFailed".to_string())
            .unwrap();
        let tele_type = GraphViewDataType::from_string(&tele_type, car_num);
        let mut current_lap = 0;

        // Handle the context output
        on_event
            .send(LapEvent::RenderingData {
                max_value: tele_type.get_max_value(&telemetry),
                unit: tele_type.get_unit(),
            })
            .unwrap();

        loop {
            // Main data distribution logic
            let telemetry = update_telemetry(&mmap_clone)
                .ok_or_else(|| "TelemetryReadFailed".to_string())
                .unwrap();

            if tele_type.get_lap(&telemetry) != current_lap {
                current_lap = tele_type.get_lap(&telemetry);
                on_event
                    .send(LapEvent::LapFinished {
                        was_best: tele_type.is_last_best(&telemetry),
                    })
                    .unwrap();
            }
            on_event
                .send(LapEvent::LapDataPoint {
                    values: tele_type.get_normalized_values(&telemetry),
                    distance: tele_type.get_normalized_distance(&telemetry),
                })
                .unwrap();

            sleep(Duration::from_millis(16)).await;
        }
    });
}

#[tauri::command]
pub fn is_last_best(
    state: State<'_, MmapState>,
    tele_type: String,
    car_num: usize,
) -> Result<bool, String> {
    let mmap = &state.mmap;
    let telemetry = update_telemetry(mmap).ok_or_else(|| "TelemetryReadFailed".to_string())?;
    let tele_type = GraphViewDataType::from_string(&tele_type, car_num);

    Ok(tele_type.is_last_best(&telemetry))
}

#[tauri::command]
pub fn get_lap(
    state: State<'_, MmapState>,
    tele_type: String,
    car_num: usize,
) -> Result<i32, String> {
    let mmap = &state.mmap;
    let telemetry = update_telemetry(mmap).ok_or_else(|| "TelemetryReadFailed".to_string())?;
    let tele_type = GraphViewDataType::from_string(&tele_type, car_num);

    Ok(tele_type.get_lap(&telemetry))
}

pub struct MmapState {
    pub mmap: Arc<Mmap>,
}

#[tauri::command]
pub fn get_values(
    state: State<'_, MmapState>,
    tele_type: String,
    car_num: usize,
) -> Result<DataPointResponse, String> {
    let mmap = &state.mmap;
    let telemetry = update_telemetry(mmap).ok_or_else(|| "TelemetryReadFailed".to_string())?;
    let tele_type = GraphViewDataType::from_string(&tele_type, car_num);

    if tele_type.is_data_valid(&telemetry) {
        Ok(DataPointResponse {
            values: tele_type.get_normalized_values(&telemetry),
            distance: tele_type.get_normalized_distance(&telemetry),
        })
    } else {
        Err("DataNotValid".to_string())
    }
}

#[derive(Clone, Default, Serialize)]
pub struct TelemetryContext {
    pub max_value: f64,
    pub unit: String,
}

#[tauri::command]
pub fn get_context(
    state: State<'_, MmapState>,
    tele_type: String,
    car_num: usize,
) -> TelemetryContext {
    let mmap = &state.mmap;

    let telemetry = match update_telemetry(mmap) {
        Some(t) => t,
        None => return TelemetryContext::default(),
    };

    let tele_type = GraphViewDataType::from_string(&tele_type, car_num);

    tele_type.get_telemetry_context(&telemetry)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum GraphViewDataType {
    Rpm(usize),        //vehicle number
    Speed(usize),      //vehicle number
    Throttle(usize),   //vehicle number
    Brake(usize),      //vehicle number
    Delta(usize, f64), //vehicle number, range (how much up and down should the normalized value be referencing in seconds)
}

impl GraphViewDataType {
    fn from_string(str: &str, car_num: usize) -> GraphViewDataType {
        match str {
            "rpm" => GraphViewDataType::Rpm(car_num),
            "speed" => GraphViewDataType::Speed(car_num),
            "throttle" => GraphViewDataType::Throttle(car_num),
            "brake" => GraphViewDataType::Brake(car_num),
            "delta" => GraphViewDataType::Delta(car_num, 5.0),
            &_ => todo!(),
        }
    }

    fn get_max_value(&self, t: &SharedMemoryObjectOut) -> f64 {
        match self {
            GraphViewDataType::Rpm(v) => t.telemetry.telemetry_info[*v].m_engine_max_rpm,
            GraphViewDataType::Speed(..) => 350.0, // This can be implemented conditionally based on the class :)
            GraphViewDataType::Throttle(..) => 1.0,
            GraphViewDataType::Brake(..) => 1.0,
            GraphViewDataType::Delta(_, r) => r * 2.0,
        }
    }

    fn get_normalized_values(&self, t: &SharedMemoryObjectOut) -> Vec<f64> {
        match self {
            GraphViewDataType::Rpm(v) => {
                vec![t.telemetry.telemetry_info[*v].m_engine_rpm / self.get_max_value(t)]
            }
            GraphViewDataType::Speed(v) => {
                vec![-t.telemetry.telemetry_info[*v].m_local_vel.z * 3.6 / self.get_max_value(t)]
            }
            GraphViewDataType::Throttle(v) => {
                vec![t.telemetry.telemetry_info[*v].m_unfiltered_throttle / self.get_max_value(t)]
            }
            GraphViewDataType::Brake(v) => {
                vec![t.telemetry.telemetry_info[*v].m_unfiltered_brake / self.get_max_value(t)]
            }
            GraphViewDataType::Delta(v, r) => {
                vec![
                    (t.telemetry.telemetry_info[*v].m_delta_best.clamp(-*r, *r) + *r)
                        / self.get_max_value(t),
                ]
            }
        }
    }

    fn get_unit(&self) -> String {
        match self {
            GraphViewDataType::Rpm(_) => "RPM".to_owned(),
            GraphViewDataType::Speed(_) => "km/h".to_owned(),
            GraphViewDataType::Throttle(_) => "%".to_owned(),
            GraphViewDataType::Brake(_) => "%".to_owned(),
            GraphViewDataType::Delta(_, _) => "s".to_owned(),
        }
    }

    fn get_car_number(&self) -> usize {
        match self {
            GraphViewDataType::Rpm(v, ..) => *v,
            GraphViewDataType::Speed(v, ..) => *v,
            GraphViewDataType::Throttle(v, ..) => *v,
            GraphViewDataType::Brake(v, ..) => *v,
            GraphViewDataType::Delta(v, ..) => *v,
        }
    }

    fn get_normalized_distance(&self, telemetry: &SharedMemoryObjectOut) -> f64 {
        // this returns the distance of how far in a lap the car is
        telemetry.scoring.veh_scoring_info[self.get_car_number()].m_lap_dist
            / telemetry.scoring.scoring_info.m_lap_dist
    }

    fn get_lap(&self, t: &SharedMemoryObjectOut) -> i32 {
        t.telemetry.telemetry_info[self.get_car_number()].m_lap_number
    }

    fn is_last_best(&self, t: &SharedMemoryObjectOut) -> bool {
        match self {
            GraphViewDataType::Delta(..) => false, // we can disable having the best lap for certain types where it isn't needed
            _ => {
                t.scoring.veh_scoring_info[self.get_car_number()].m_last_lap_time
                    <= t.scoring.veh_scoring_info[self.get_car_number()].m_best_lap_time
            }
        }
    }

    fn is_data_valid(&self, t: &SharedMemoryObjectOut) -> bool {
        match self {
            GraphViewDataType::Rpm(_) => self.get_normalized_values(t)[0].is_finite(),
            GraphViewDataType::Speed(_) => self.get_normalized_values(t)[0].is_finite(),
            GraphViewDataType::Throttle(_) => self.get_normalized_values(t)[0].is_finite(),
            GraphViewDataType::Brake(_) => self.get_normalized_values(t)[0].is_finite(),
            GraphViewDataType::Delta(_, _) => self.get_normalized_values(t)[0].is_finite(),
        }
    }

    fn get_telemetry_context(&self, t: &SharedMemoryObjectOut) -> TelemetryContext {
        TelemetryContext {
            max_value: self.get_max_value(t),
            unit: self.get_unit(),
        }
    }
}
