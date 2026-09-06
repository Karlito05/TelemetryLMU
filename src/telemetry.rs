// TODO:
// Make it so the it saves to the path declared in settings.
// Make it so the user can choose if it should log only him or the whole lobby.

use std::{
    fs,
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use chrono::Local;

use crate::{
    TOKIO,
    frontend::frontend_main::SettingsProvider,
    interface::{
        self, IPVehicleClass, Interface, SharedMemoryObjectOut, i8_array32_to_string,
        i8_array64_to_string,
    },
};

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
            Self::Delta => v / (self.get_max_value(t, car_num) / 2.0) + 0.5,
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

#[derive(Debug)]
pub struct Telemetry {
    pub cur_lap: Arc<Mutex<[Lap; 104]>>,
    pub last_lap: Arc<Mutex<[Lap; 104]>>,
    pub best_lap: Arc<Mutex<[Lap; 104]>>,
    pub cur_lap_nums: Arc<Mutex<[i32; 104]>>,
    telemetry: Arc<Mutex<interface::Interface>>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    settings_provider: Arc<SettingsProvider>,
}

#[derive(Default, Clone, Debug)]
pub struct Lap {
    pub datapoints: [Vec<f32>; TelemetryValueType::Max as usize],
    pub laptime: Option<f32>,
}

impl Telemetry {
    pub fn new(path: PathBuf, settings_provider: Arc<SettingsProvider>) -> Result<Self, String> {
        let telemetry = Arc::new(Mutex::new(interface::Interface::new(
            &path.to_string_lossy(),
        )));

        if !telemetry.lock().unwrap().full_mode {
            return Err("Could not start telemetry. Check if the game is running!".to_owned());
        }

        let cur_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let last_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let best_lap = Arc::new(Mutex::new(std::array::from_fn(|_| Lap::default())));
        let cur_lap_nums = Arc::new(Mutex::new(std::array::from_fn(|_| 0)));
        let running = Arc::new(AtomicBool::new(true));

        let thread_best_lap = Arc::clone(&best_lap);
        let thread_settings_provider = Arc::clone(&settings_provider);
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
                        let interface = Interface::new("/dev/shm/LMU_Data");
                        if (*thread_settings_provider.log_all_cars.read().unwrap()
                            || i8_array32_to_string(
                                &thread_telemetry
                                    .lock()
                                    .unwrap()
                                    .update_telemetry()
                                    .unwrap()
                                    .scoring
                                    .veh_scoring_info[j]
                                    .m_driver_name,
                            ) == *thread_settings_provider.in_game_name.read().unwrap())
                            && *thread_settings_provider.record_laps.read().unwrap()
                        {
                            TOKIO.get().expect("tokio runtime not initialised").spawn(
                                set_laptime_best_and_save(
                                    Arc::clone(&thread_last_lap),
                                    Arc::clone(&thread_best_lap),
                                    "/dev/shm/LMU_Data", // We
                                    // just make a new interface here because it's inexpensive and would
                                    // cause deadlocks if we didn't
                                    j,
                                    thread_settings_provider
                                        .record_save_path
                                        .read()
                                        .unwrap()
                                        .clone()
                                        + "/",
                                ),
                            );
                        } else {
                            TOKIO.get().expect("tokio runtime not initialised").spawn(
                                set_laptime_and_best(
                                    Arc::clone(&thread_last_lap),
                                    Arc::clone(&thread_best_lap),
                                    "/dev/shm/LMU_Data",
                                    j,
                                ),
                            );
                        }
                        for logged_value in driver.datapoints.iter_mut() {
                            logged_value.clear();
                        }
                    }
                    for (h, dp) in data.0.iter().enumerate() {
                        if driver.datapoints[h].len() > 21600 {
                            driver.datapoints[h].remove(0);
                        }
                        driver.datapoints[h].push(*dp as f32);
                    }
                }
                thread::sleep(Duration::from_millis(16));
            }
        });

        Ok(Self {
            cur_lap,
            last_lap,
            best_lap,
            running,
            handle: Some(handle),
            cur_lap_nums,
            telemetry,
            settings_provider,
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
    t: &interface::Interface,
    cur_laps: [i32; 104],
) -> [([f64; TelemetryValueType::Max as usize], Option<i32>); 104] {
    let cur_data = t.update_telemetry().unwrap();

    let mut ret: [([f64; TelemetryValueType::Max as usize], Option<i32>); 104] =
        std::array::from_fn(|_| (std::array::from_fn(|_| 0.0), None));
    for j in 0..104 {
        let new_lap_num = cur_data.telemetry.telemetry_info[j].m_lap_number;
        if cur_laps[j] != new_lap_num
            && TelemetryValueType::DistanceIntoLap.get_value(&cur_data, j) < 100.0
        {
            ret[j].1 = Some(new_lap_num);
        }
        for i in 0..TelemetryValueType::Max as usize {
            let tel_type = TelemetryValueType::try_from(i).unwrap();
            ret[j].0[i] = tel_type.get_value(&cur_data, j);
        }
    }
    ret
}

async fn set_laptime_best_and_save(
    last_lap: Arc<Mutex<[Lap; 104]>>,
    best_lap: Arc<Mutex<[Lap; 104]>>,
    interface_path: &str,
    car_num: usize,
    save_path: String,
) {
    let interface = Interface::new(interface_path);
    set_laptime(last_lap.clone(), &interface, car_num).await;
    set_best(best_lap, last_lap.clone(), car_num).await;
    save(last_lap, &interface, car_num, save_path).await;
}

async fn set_laptime_and_best(
    last_lap: Arc<Mutex<[Lap; 104]>>,
    best_lap: Arc<Mutex<[Lap; 104]>>,
    interface_path: &str,
    car_num: usize,
) {
    let interface = Interface::new(interface_path);
    set_laptime(last_lap.clone(), &interface, car_num).await;
    set_best(best_lap, last_lap, car_num).await;
}

async fn set_best(
    best_lap: Arc<Mutex<[Lap; 104]>>,
    last_lap: Arc<Mutex<[Lap; 104]>>,
    car_num: usize,
) {
    let mut best_lap_guard = best_lap.lock().unwrap();
    let last_lap_guard = last_lap.lock().unwrap();
    if let Some(best_laptime) = best_lap_guard[car_num].laptime {
        if let Some(last_laptime) = last_lap_guard[car_num].laptime {
            if best_laptime > last_laptime {
                best_lap_guard[car_num] = last_lap_guard[car_num].clone();
            }
        }
    }
}

async fn set_laptime(
    last_lap: Arc<Mutex<[Lap; 104]>>,
    interface: &interface::Interface,
    car_num: usize,
) {
    tokio::time::sleep(Duration::from_millis(100)).await;

    let laptime = {
        interface
            .update_telemetry()
            .unwrap()
            .scoring
            .veh_scoring_info[car_num]
            .m_last_lap_time as f32
    };

    last_lap.lock().unwrap()[car_num].laptime = Some(laptime);
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SaveData {
    date: String,
    track: String,
    driver_name: String,
    car: String,
    car_class: IPVehicleClass,
    lap_time: f32,
    // Conditions
    lap_data: [Vec<f32>; TelemetryValueType::Max as usize],
}

async fn save(
    lap: Arc<Mutex<[Lap; 104]>>,
    interface: &interface::Interface,
    car_num: usize,
    path: String,
) {
    let time = Local::now().format("%d-%m-%Y-%H-%M-%S").to_string();
    let telemetry = interface.update_telemetry().unwrap();

    let track = i8_array64_to_string(&telemetry.scoring.scoring_info.m_track_name);
    let car = i8_array64_to_string(&telemetry.scoring.veh_scoring_info[car_num].m_vehicle_name);
    let car_class = telemetry.telemetry.telemetry_info[car_num].m_vehicle_class;
    let driver_name =
        i8_array32_to_string(&telemetry.scoring.veh_scoring_info[car_num].m_driver_name);

    let to_save_lap: Lap;
    {
        to_save_lap = lap.lock().unwrap()[car_num].clone();
    }
    let save_data = SaveData {
        date: time,
        track: track.clone(),
        driver_name: driver_name.clone(),
        car,
        car_class,
        lap_time: to_save_lap.laptime.unwrap_or(-1.0), // -1.0 fails the save check down
        // the line
        lap_data: to_save_lap.datapoints,
    };

    if save_data.lap_time > 0.0
        && TelemetryValueType::DistanceIntoLap.normalize(
            (*save_data.lap_data[TelemetryValueType::DistanceIntoLap as usize]
                .first()
                .unwrap_or(&1.0)) as f64, // 1.0 so it fails the check
            &telemetry,
            car_num,
        ) < 0.5
        && save_data.lap_data[0].len() > 60
    // have recorded at least a second
    {
        fs::write(
            path + &track
                + "-"
                + &Local::now().format("%d.%m.%Y %H:%M:%S").to_string()
                + "-"
                + &driver_name
                + ".json",
            serde_json::to_string(&save_data).unwrap(),
        )
        .unwrap();
    }
}
