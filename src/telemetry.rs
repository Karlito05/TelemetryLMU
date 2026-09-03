use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

use crate::interface::{self, SharedMemoryObjectOut};

enum TelemetryValueType {
    Rpm,
    Speed,
    Throttle,
    Brake,
    Delta,
    Gear,
    Steering,
    DistanceIntoLap,
    TimeIntoLap,

    Max,
}

impl TryFrom<usize> for TelemetryValueType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rpm),
            1 => Ok(Self::Speed),
            2 => Ok(Self::Throttle),
            3 => Ok(Self::Brake),
            4 => Ok(Self::Delta),
            5 => Ok(Self::Gear),
            6 => Ok(Self::Steering),
            7 => Ok(Self::DistanceIntoLap),
            8 => Ok(Self::TimeIntoLap),
            _ => Err(()),
        }
    }
}

impl TelemetryValueType {
    fn get_value(&self, t: &SharedMemoryObjectOut, driver: usize) -> f64 {
        match self {
            Self::Rpm => t.telemetry.telemetry_info[driver].m_engine_rpm,
            Self::Speed => t.telemetry.telemetry_info[driver].m_local_vel.z,
            Self::Throttle => t.telemetry.telemetry_info[driver].m_unfiltered_throttle,
            Self::Brake => t.telemetry.telemetry_info[driver].m_unfiltered_brake,
            Self::Delta => t.telemetry.telemetry_info[driver].m_delta_best,
            Self::Gear => t.telemetry.telemetry_info[driver].m_gear as f64,
            Self::Steering => t.telemetry.telemetry_info[driver].m_unfiltered_steering,
            Self::DistanceIntoLap => t.scoring.veh_scoring_info[driver].m_lap_dist,
            Self::TimeIntoLap => t.scoring.veh_scoring_info[driver].m_time_into_lap,
            Self::Max => panic!("Can't call get value on TelemetryValueType::Max"),
        }
    }
}

pub struct Telemetry {
    pub cur_lap: Arc<Mutex<[Lap; 104]>>,
    pub last_lap: Arc<Mutex<[Lap; 104]>>,
    pub cur_lap_nums: Arc<Mutex<[i32; 104]>>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

#[derive(Default, Clone)]
pub struct Lap {
    datapoints: [Vec<f32>; TelemetryValueType::Max as usize],
    laptime: Option<f32>,
}

impl Telemetry {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let thread_telemetry = interface::Telemetry::new(&path.to_string_lossy());

        if !thread_telemetry.full_mode {
            return Err("Could not start telemetry. Check if the game is running!".to_owned());
        }

        let cur_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let last_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let cur_lap_nums = Arc::new(Mutex::new(std::array::from_fn(|_| 0)));
        let running = Arc::new(AtomicBool::new(true));

        let thread_cur_lap = Arc::clone(&cur_lap);
        let thread_last_lap = Arc::clone(&last_lap);
        let thread_cur_lap_nums = Arc::clone(&cur_lap_nums);
        let thread_running = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while thread_running.load(Ordering::Relaxed) {
                let new_data =
                    get_telemetry(&thread_telemetry, *thread_cur_lap_nums.lock().unwrap());

                for (j, driver) in thread_cur_lap.lock().unwrap().iter_mut().enumerate() {
                    new_data.iter().enumerate().for_each(|(i, data)| {
                        if data.1.is_some() {
                            thread_last_lap.lock().unwrap()[j] = driver.clone();

                            for logged_value in driver.datapoints.iter_mut() {
                                logged_value.clear();
                            }
                        }

                        driver.datapoints[i].push(data.0[i] as f32);
                    })
                }
            }
        });

        Ok(Self {
            cur_lap,
            last_lap,
            running,
            handle: Some(handle),
            cur_lap_nums,
        })
    }
}

/// Returns A new DP for each of the arrays and an optional lap number if it has changed
fn get_telemetry(
    t: &interface::Telemetry,
    cur_laps: [i32; 104],
) -> [([f64; TelemetryValueType::Max as usize], Option<i32>); 104] {
    let cur_data = t.update_telemetry().unwrap();

    let mut ret: [([f64; TelemetryValueType::Max as usize], Option<i32>); 104] =
        std::array::from_fn(|_| (std::array::from_fn(|_| 0.0), None));
    for j in 0..104 {
        for i in 0..TelemetryValueType::Max as usize {
            ret[j].0[i] = TelemetryValueType::try_from(i)
                .unwrap()
                .get_value(&cur_data, j);
            if cur_laps[j] != cur_data.telemetry.telemetry_info[i].m_lap_number {
                ret[j].1 = Some(cur_data.telemetry.telemetry_info[i].m_lap_number)
            }
        }
    }
    ret
}
