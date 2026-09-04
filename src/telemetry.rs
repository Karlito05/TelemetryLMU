use std::{
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::interface::{self, SharedMemoryObjectOut};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default, Debug)]
pub enum TelemetryValueType {
    #[default]
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
    pub fn get_value(&self, t: &SharedMemoryObjectOut, driver: usize) -> f64 {
        match self {
            Self::Rpm => t.telemetry.telemetry_info[driver].m_engine_rpm,
            Self::Speed => -t.telemetry.telemetry_info[driver].m_local_vel.z * 3.6,
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

    pub fn to_string(&self) -> String {
        match self {
            Self::Rpm => "rpm".to_owned(),
            Self::Speed => "speed".to_owned(),
            Self::Throttle => "throttle".to_owned(),
            Self::Brake => "brake".to_owned(),
            Self::Delta => "delta".to_owned(),
            Self::Gear => "gear".to_owned(),
            Self::Steering => "steering".to_owned(),
            Self::DistanceIntoLap => "distance_into_lap".to_owned(),
            Self::TimeIntoLap => "time_into_lap".to_owned(),
            Self::Max => panic!("Can't call to_string on TelemetryValueType::Max"),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "rpm" => Self::Rpm,
            "speed" => Self::Speed,
            "throttle" => Self::Throttle,
            "brake" => Self::Brake,
            "delta" => Self::Delta,
            "gear" => Self::Gear,
            "steering" => Self::Steering,
            "distance_into_lap" => Self::DistanceIntoLap,
            "time_into_lap" => Self::TimeIntoLap,
            other => panic!("Unknown TelemetryValueType: {other}"),
        }
    }

    pub fn get_unit(&self) -> String {
        match self {
            Self::Rpm => "RPM".to_owned(),
            Self::Speed => "km/h".to_owned(),
            Self::Throttle => "%".to_owned(),
            Self::Brake => "%".to_owned(),
            Self::Delta => "s".to_owned(),
            Self::Gear => "".to_owned(),
            Self::Steering => "deg".to_owned(),
            Self::DistanceIntoLap => "m".to_owned(),
            Self::TimeIntoLap => "s".to_owned(),
            Self::Max => panic!("Can't call get_unit on TelemetryValueType::Max"),
        }
    }

    pub fn get_max_value(&self, t: &SharedMemoryObjectOut, car_num: usize) -> f64 {
        match self {
            Self::Rpm => t.telemetry.telemetry_info[car_num].m_engine_max_rpm,
            Self::Speed => 350.0,
            Self::Throttle => 1.0,
            Self::Brake => 1.0,
            Self::Delta => 10.0,
            Self::Gear => t.telemetry.telemetry_info[car_num].m_max_gears as f64,
            Self::Steering => 1.0,
            Self::DistanceIntoLap => t.scoring.scoring_info.m_lap_dist,
            Self::TimeIntoLap => 0.0,
            Self::Max => panic!("Can't call get_max_value on TelemetryValueType::Max"),
        }
    }

    pub fn normalize(&self, v: f64, t: &SharedMemoryObjectOut, car_num: usize) -> f64 {
        match self {
            Self::Delta => v / (self.get_max_value(t, car_num) / 2.0),
            Self::TimeIntoLap => -1.0,
            Self::Max => panic!("Can't call get_max_value on TelemetryValueType::Max"),
            _ => v / self.get_max_value(t, car_num),
        }
    }

    pub fn get_unit_labels(
        &self,
        telemetry: &SharedMemoryObjectOut,
        n_gridlines: i32,
        car_num: usize,
    ) -> Vec<String> {
        let mut ret = vec![];
        match self {
            Self::Throttle => {
                for i in 0..n_gridlines {
                    let str = format!(
                        "{} {}",
                        self.get_max_value(telemetry, car_num) * (n_gridlines - 1 - i) as f64
                            / (n_gridlines - 1) as f64
                            * 100.0,
                        self.get_unit()
                    );
                    ret.push(str);
                }
            }
            Self::Brake => {
                for i in 0..n_gridlines {
                    let str = format!(
                        "{} {}",
                        self.get_max_value(telemetry, car_num) * (n_gridlines - 1 - i) as f64
                            / (n_gridlines - 1) as f64
                            * 100.0,
                        self.get_unit()
                    );
                    ret.push(str);
                }
            }
            Self::Delta => {
                for i in 0..n_gridlines {
                    let str = format!(
                        "{} {}",
                        self.get_max_value(telemetry, car_num) * (n_gridlines - 1 - i) as f64
                            / (n_gridlines - 1) as f64
                            - 5.0,
                        self.get_unit()
                    );
                    ret.push(str);
                }
            }
            Self::Max => {
                panic!("Can't call get_car_unit_labels() on GraphViewDataType::Unknown ")
            }
            _ => {
                for i in 0..n_gridlines {
                    let str = format!(
                        "{} {}",
                        self.get_max_value(telemetry, car_num) * (n_gridlines - 1 - i) as f64
                            / (n_gridlines - 1) as f64,
                        self.get_unit()
                    );
                    ret.push(str);
                }
            }
        }
        ret
    }
}

// TODO:
// Add a hard limit on how big the data can get
// At 21600 (should be good for 6min of data)

#[derive(Debug)]
pub struct Telemetry {
    pub cur_lap: Arc<Mutex<[Lap; 104]>>,
    pub last_lap: Arc<Mutex<[Lap; 104]>>,
    pub cur_lap_nums: Arc<Mutex<[i32; 104]>>,
    telemetry: Arc<Mutex<interface::Telemetry>>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

#[derive(Default, Clone, Debug)]
pub struct Lap {
    pub datapoints: [Vec<f32>; TelemetryValueType::Max as usize],
    pub laptime: Option<f32>,
}

impl Telemetry {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let telemetry = Arc::new(Mutex::new(interface::Telemetry::new(
            &path.to_string_lossy(),
        )));

        if !telemetry.lock().unwrap().full_mode {
            return Err("Could not start telemetry. Check if the game is running!".to_owned());
        }

        let cur_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let last_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let cur_lap_nums = Arc::new(Mutex::new(std::array::from_fn(|_| 0)));
        let running = Arc::new(AtomicBool::new(true));

        let thread_telemetry = Arc::clone(&telemetry);
        let thread_cur_lap = Arc::clone(&cur_lap);
        let thread_last_lap = Arc::clone(&last_lap);
        let thread_cur_lap_nums = Arc::clone(&cur_lap_nums);
        let thread_running = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while thread_running.load(Ordering::Relaxed) {
                let new_data = get_telemetry(
                    &thread_telemetry.lock().unwrap(),
                    *thread_cur_lap_nums.lock().unwrap(),
                );

                for ((j, driver), data) in thread_cur_lap
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .enumerate()
                    .zip(new_data.iter())
                {
                    if let Some(new_lap) = data.1 {
                        thread_last_lap.lock().unwrap()[j] = driver.clone();
                        thread_cur_lap_nums.lock().unwrap()[j] = new_lap;
                        for logged_value in driver.datapoints.iter_mut() {
                            logged_value.clear();
                        }
                    }
                    for (h, dp) in data.0.iter().enumerate() {
                        driver.datapoints[h].push(*dp as f32);
                    }
                }
                thread::sleep(Duration::from_millis(16));
            }
        });

        Ok(Self {
            cur_lap,
            last_lap,
            running,
            handle: Some(handle),
            cur_lap_nums,
            telemetry,
        })
    }

    pub fn get_drivers(&self) -> Vec<(String, i32)> {
        self.telemetry.lock().unwrap().get_drivers()
    }

    pub fn get_telemetry_object(&self) -> Box<SharedMemoryObjectOut> {
        self.telemetry.lock().unwrap().update_telemetry().unwrap()
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
        let new_lap_num = cur_data.telemetry.telemetry_info[j].m_lap_number;
        if cur_laps[j] != new_lap_num {
            ret[j].1 = Some(new_lap_num);
        }
        for i in 0..TelemetryValueType::Max as usize {
            let tel_type = TelemetryValueType::try_from(i).unwrap();
            ret[j].0[i] = tel_type.get_value(&cur_data, j);
        }
    }
    ret
}
