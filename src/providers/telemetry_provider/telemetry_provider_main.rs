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
    providers::{
        settings_provider::SettingsProvider,
        telemetry_provider::{
            interface::{
                IPVehicleClass, Interface, SharedMemoryObjectOut, TelemVect3, i8_array32_to_string,
                i8_array64_to_string,
            },
            telemetry_value_type::TelemetryValueType,
        },
    },
};

#[derive(Debug)]
#[expect(unused)]
pub struct Telemetry {
    pub cur_lap: Arc<Mutex<[Lap; 104]>>,
    pub last_lap: Arc<Mutex<[Lap; 104]>>,
    pub best_lap: Arc<Mutex<[Lap; 104]>>,
    pub cur_lap_nums: Arc<Mutex<[i32; 104]>>,
    telemetry: Arc<Mutex<Interface>>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    settings_provider: Arc<SettingsProvider>,
}

#[derive(Default, Clone, Debug)]
pub struct Lap {
    pub datapoints: [Vec<f32>; TelemetryValueType::Max as usize],
    pub distances: Vec<f32>,
    pub positions: Vec<TelemVect3>,
    pub times: Vec<f32>,
    pub laptime: Option<f32>,
}

impl Lap {
    fn push_sample(
        &mut self,
        datapoints: &[f64; TelemetryValueType::Max as usize],
        distance: f64,
        position: TelemVect3,
        time: f64,
    ) {
        const MAX_SAMPLES: usize = 21600;

        if self.distances.len() >= MAX_SAMPLES {
            for values in &mut self.datapoints {
                values.remove(0);
            }
            self.distances.remove(0);
            self.positions.remove(0);
            self.times.remove(0);
        }

        for (values, datapoint) in self.datapoints.iter_mut().zip(datapoints) {
            values.push(*datapoint as f32);
        }
        self.distances.push(distance as f32);
        self.positions.push(position);
        self.times.push(time as f32);
    }
}

impl Telemetry {
    pub fn new(path: PathBuf, settings_provider: Arc<SettingsProvider>) -> Result<Self, String> {
        let telemetry = Arc::new(Mutex::new(Interface::new(&path.to_string_lossy())));

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
                            #[cfg(not(target_os = "windows"))]
                            let path = thread_settings_provider
                                .record_save_path
                                .read()
                                .unwrap()
                                .clone()
                                + "/";
                            #[cfg(target_os = "windows")]
                            let path = thread_settings_provider
                                .record_save_path
                                .read()
                                .unwrap()
                                .clone()
                                + "\\";

                            TOKIO.get().expect("tokio runtime not initialised").spawn(
                                set_laptime_best_and_save(
                                    Arc::clone(&thread_last_lap),
                                    Arc::clone(&thread_best_lap),
                                    "/dev/shm/LMU_Data", // We
                                    // just make a new interface here because it's inexpensive and would
                                    // cause deadlocks if we didn't
                                    j,
                                    path,
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
                        driver.positions.clear();
                        driver.distances.clear();
                        driver.times.clear();
                        driver.laptime = None;
                    }
                    driver.push_sample(&data.0, data.2, data.3, data.4);
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

type NewData = [(
    [f64; TelemetryValueType::Max as usize],
    Option<i32>,
    f64,
    TelemVect3,
    f64,
); 104];

/// Returns A new DP for each of the arrays and an optional lap number if it has changed
fn get_telemetry(t: &Interface, cur_laps: [i32; 104]) -> NewData {
    let cur_data = t.update_telemetry().unwrap();

    let mut ret: NewData = std::array::from_fn(|_| {
        (
            std::array::from_fn(|_| 0.0),
            None,
            0.0,
            TelemVect3::default(),
            0.0,
        )
    });
    for j in 0..104 {
        let new_lap_num = cur_data.telemetry.telemetry_info[j].m_lap_number;
        if cur_laps[j] != new_lap_num
            && TelemetryValueType::get_distance_into_lap(&cur_data, j) < 100.0
        {
            ret[j].1 = Some(new_lap_num);
        }

        #[expect(clippy::needless_range_loop)]
        for i in 0..TelemetryValueType::Max as usize {
            let tel_type = TelemetryValueType::try_from(i).unwrap();
            ret[j].0[i] = tel_type.get_value(&cur_data, j);
        }
        ret[j].2 = TelemetryValueType::get_distance_into_lap(&cur_data, j);
        ret[j].3 = TelemetryValueType::get_pos(&cur_data, j);
        ret[j].4 = TelemetryValueType::get_time_into_lap(&cur_data, j);
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
        if let Some(last_laptime) = last_lap_guard[car_num].laptime
            && best_laptime > last_laptime
        {
            best_lap_guard[car_num] = last_lap_guard[car_num].clone();
        }
    } else {
        if let Some(last_laptime) = last_lap_guard[car_num].laptime
            && last_laptime > 0.0
        {
            best_lap_guard[car_num] = last_lap_guard[car_num].clone();
        }
    }
}

async fn set_laptime(last_lap: Arc<Mutex<[Lap; 104]>>, interface: &Interface, car_num: usize) {
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

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SaveData {
    pub date: String,
    pub track: String,
    pub driver_name: String,
    pub car: String,
    pub car_class: IPVehicleClass,
    pub distances: Vec<f32>,
    pub positions: Vec<TelemVect3>,
    pub times: Vec<f32>,
    pub lap_time: f32,
    // Conditions
    pub lap_data: [Vec<f32>; TelemetryValueType::Max as usize],
}

async fn save(lap: Arc<Mutex<[Lap; 104]>>, interface: &Interface, car_num: usize, path: String) {
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
        distances: to_save_lap.distances,
        positions: to_save_lap.positions,
        times: to_save_lap.times,
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
        && TelemetryValueType::normalize_distance_into_lap(
            &telemetry,
            *save_data.distances.first().unwrap() as f64,
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
