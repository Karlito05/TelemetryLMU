use crate::telemetry::{i8_array_to_string, JoinHandleIdent, TelemetryState};
use crate::telemetry::{update_telemetry, SharedMemoryObjectOut};
use log::{error, info, warn};
use memmap2::Mmap;
use serde::Serialize;
use std::sync::Mutex;
use std::{sync::Arc, time::Duration};
use tauri::{ipc::Channel, State};
use tokio::task::coop::RestoreOnPending;
use tokio::time::sleep;

pub struct GraphViewState {
    pub threads: Vec<JoinHandleIdent>,
}

pub struct MmapState {
    pub mmap: Arc<Mmap>,
}

#[derive(Clone, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum LapEvent {
    RenderingData {
        max_value: f64,
        unit: String,
        id: String,
    },
    LapDataPoint {
        values: Vec<f64>,
        distance: f64,
    },
    LapFinished {
        was_best: bool,
    },
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct Driver {
    pub index: usize,
    pub name: String,
}

#[derive(Clone, Serialize)]
pub struct DataPoint {
    values: Vec<f64>,
    distance: f64,
    lap_num: i32,
    graph_type: String,
}

#[tauri::command]
pub async fn get_lap_data(
    mmap: State<'_, MmapState>,
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    car_num: usize,
    tele_types: Vec<String>,
) -> Result<Vec<DataPoint>, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Full mode not active. Turn on your game and then refresh.".to_owned());
    }

    let telemetry = update_telemetry(&mmap.mmap).unwrap();

    let mut return_val: Vec<DataPoint> = vec![];
    for tele_type_string in tele_types {
        let tele_type = GraphViewDataType::from_string(&tele_type_string, car_num);
        return_val.push(DataPoint {
            values: tele_type.get_normalized_values(&telemetry),
            distance: tele_type.get_normalized_distance(&telemetry),
            lap_num: tele_type.get_lap(&telemetry),
            graph_type: tele_type.to_string(),
        });
    }

    Ok(return_val)
}

#[derive(Clone, Serialize)]
pub struct TelemetryInfo {
    max_value: f64,
    unit: String,
    graph_type: String,
}

#[tauri::command]
pub async fn get_telemetry_info(
    mmap: State<'_, MmapState>,
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    car_num: usize,
    tele_types: Vec<String>,
) -> Result<Vec<TelemetryInfo>, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Full mode not active. Turn on your game and then refresh.".to_owned());
    }

    let telemetry = update_telemetry(&mmap.mmap).unwrap();

    let mut return_val: Vec<TelemetryInfo> = vec![];
    for tele_type_string in tele_types {
        let tele_type = GraphViewDataType::from_string(&tele_type_string, car_num);
        return_val.push(TelemetryInfo {
            max_value: tele_type.get_max_value(&telemetry),
            unit: tele_type.get_unit(),
            graph_type: tele_type.to_string(),
        });
    }

    Ok(return_val)
}

#[tauri::command]
pub async fn was_last_best(
    mmap: State<'_, MmapState>,
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    car_num: usize,
) -> Result<bool, String> {
    if !telemetry_state.lock().unwrap().full_mode {
        return Err("Full mode not active. Turn on your game and then refresh.".to_owned());
    }
    let telemetry = update_telemetry(&mmap.mmap).unwrap();
    Ok(GraphViewDataType::Rpm(car_num).is_last_best(&telemetry))
}

#[tauri::command]
pub fn lap_data_subscribe(
    mmap: State<'_, MmapState>,
    telemetry_state: State<'_, Mutex<TelemetryState>>,
    graph_view_state: State<'_, Mutex<GraphViewState>>,
    car_num: usize,
    tele_type: String,
    on_event: Channel<LapEvent>,
) {
    if !telemetry_state.lock().unwrap().full_mode {
        return;
    }
    let mmap_clone = Arc::clone(&mmap.mmap);
    let id = format!("{tele_type}-{car_num}");
    let id_for_log = id.clone();
    let join_handle = tauri::async_runtime::spawn(async move {
        info!("Current car num is {car_num}");
        let telemetry = match update_telemetry(&mmap_clone) {
            Some(v) => v,
            None => {
                warn!("Telemetry read failed on subscribe for thread {id_for_log}");
                return;
            }
        };
        let tele_type = GraphViewDataType::from_string(&tele_type, car_num);
        let mut current_lap = 0;

        // Handle the context output
        on_event
            .send(LapEvent::RenderingData {
                max_value: tele_type.get_max_value(&telemetry),
                unit: tele_type.get_unit(),
                id: format!("{}-{car_num}", tele_type.to_string()),
            })
            .unwrap();

        loop {
            // Main data distribution logic
            let telemetry = match update_telemetry(&mmap_clone) {
                Some(v) => v,
                None => {
                    // No live producer currently available; keep thread alive.
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
            };

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

    graph_view_state
        .lock()
        .unwrap()
        .threads
        .push(JoinHandleIdent { id, join_handle });
}

#[tauri::command]
pub async fn lap_data_unsubscribe(
    graph_view_state: State<'_, Mutex<GraphViewState>>,
    id: String,
) -> Result<(), String> {
    let mut i = 0;
    for handle in &graph_view_state.lock().unwrap().threads {
        if handle.id == id {
            handle.join_handle.abort();
            info!("Stopped thread {i}");
            break;
        } else {
            i += 1;
        }
    }
    if i != graph_view_state.lock().unwrap().threads.len() {
        graph_view_state.lock().unwrap().threads.remove(i);
    } else {
        error!("Failed to remove thread {id}")
    }
    let remaining_threads = graph_view_state.lock().unwrap().threads.len();
    if remaining_threads != 0 {
        info!("There are {} more remaining threads", remaining_threads)
    }
    Ok(())
}

#[tauri::command]
pub async fn get_drivers(mmap: State<'_, MmapState>) -> Result<Vec<Driver>, String> {
    let telemetry =
        update_telemetry(&mmap.mmap).ok_or_else(|| "TelemetryReadFailed".to_string())?;

    let mut drivers: Vec<Driver> = Vec::new();
    for i in 0..103 {
        let name = i8_array_to_string(&telemetry.scoring.veh_scoring_info[i].m_driver_name);
        if name != "" {
            drivers.push(Driver {
                index: i,
                name: name,
            });
        }
    }
    Ok(drivers)
}

pub const GRAPH_VIEW_DATA_TYPE_COUNT: i32 = 5;

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum GraphViewDataType {
    Rpm(usize) = 0,    //vehicle number
    Speed(usize),      //vehicle number
    Throttle(usize),   //vehicle number
    Brake(usize),      //vehicle number
    Delta(usize, f64), //vehicle number, range (how much up and down should the normalized value be referencing in seconds)
}

impl GraphViewDataType {
    pub fn from_string(str: &str, car_num: usize) -> GraphViewDataType {
        match str {
            "rpm" => GraphViewDataType::Rpm(car_num),
            "speed" => GraphViewDataType::Speed(car_num),
            "throttle" => GraphViewDataType::Throttle(car_num),
            "brake" => GraphViewDataType::Brake(car_num),
            "delta" => GraphViewDataType::Delta(car_num, 5.0),
            &_ => todo!(),
        }
    }
    pub fn from_int(int: i32, car_num: usize) -> GraphViewDataType {
        match int {
            0 => GraphViewDataType::Rpm(car_num),
            1 => GraphViewDataType::Speed(car_num),
            2 => GraphViewDataType::Throttle(car_num),
            3 => GraphViewDataType::Brake(car_num),
            4 => GraphViewDataType::Delta(car_num, 5.0),
            _ => todo!(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GraphViewDataType::Rpm(..) => "rpm".to_owned(),
            GraphViewDataType::Speed(..) => "speed".to_owned(),
            GraphViewDataType::Throttle(..) => "throttle".to_owned(),
            GraphViewDataType::Brake(..) => "brake".to_owned(),
            GraphViewDataType::Delta(..) => "delta".to_owned(),
        }
    }

    pub fn get_max_value(&self, t: &SharedMemoryObjectOut) -> f64 {
        match self {
            GraphViewDataType::Rpm(v) => t.telemetry.telemetry_info[*v].m_engine_max_rpm,
            GraphViewDataType::Speed(..) => 350.0, // This can be implemented conditionally based on the class :)
            GraphViewDataType::Throttle(..) => 1.0,
            GraphViewDataType::Brake(..) => 1.0,
            GraphViewDataType::Delta(_, r) => r * 2.0,
        }
    }

    pub fn get_normalized_values(&self, t: &SharedMemoryObjectOut) -> Vec<f64> {
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

    pub fn get_unit(&self) -> String {
        match self {
            GraphViewDataType::Rpm(_) => "RPM".to_owned(),
            GraphViewDataType::Speed(_) => "km/h".to_owned(),
            GraphViewDataType::Throttle(_) => "%".to_owned(),
            GraphViewDataType::Brake(_) => "%".to_owned(),
            GraphViewDataType::Delta(_, _) => "s".to_owned(),
        }
    }

    pub fn get_car_number(&self) -> usize {
        match self {
            GraphViewDataType::Rpm(v, ..) => *v,
            GraphViewDataType::Speed(v, ..) => *v,
            GraphViewDataType::Throttle(v, ..) => *v,
            GraphViewDataType::Brake(v, ..) => *v,
            GraphViewDataType::Delta(v, ..) => *v,
        }
    }

    pub fn get_normalized_distance(&self, telemetry: &SharedMemoryObjectOut) -> f64 {
        // this returns the distance of how far in a lap the car is
        telemetry.scoring.veh_scoring_info[self.get_car_number()].m_lap_dist
            / telemetry.scoring.scoring_info.m_lap_dist
    }

    pub fn get_lap(&self, t: &SharedMemoryObjectOut) -> i32 {
        t.telemetry.telemetry_info[self.get_car_number()].m_lap_number
    }

    pub fn is_last_best(&self, t: &SharedMemoryObjectOut) -> bool {
        match self {
            GraphViewDataType::Delta(..) => false, // we can disable having the best lap for certain types where it isn't needed
            _ => {
                t.scoring.veh_scoring_info[self.get_car_number()].m_last_lap_time
                    <= t.scoring.veh_scoring_info[self.get_car_number()].m_best_lap_time
            }
        }
    }
}
