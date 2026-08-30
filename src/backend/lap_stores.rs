use std::{fs, time::Duration};

use chrono::Local;
use eframe::egui::{Vec2, vec2};

use crate::{
    backend::telemetry::{GRAPH_VIEW_DATA_TYPE_COUNT, GraphViewDataType},
    interface::{TelemVect3, Telemetry},
};

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
#[serde(default)]
pub struct SaveData {
    pub lap_info: LapInfo,
    pub lap_data: Vec<LapData>,
    pub pos_data: Vec<PosData>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
#[serde(default)]
pub struct PosData {
    pub pos: TelemVect3,
    pub time_since_lap_start: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
#[serde(default)]
pub struct LapData {
    pub data_type: String,
    pub values: Vec<Vec2>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
#[serde(default)]
pub struct LapInfo {
    pub lap_time: f64,
    pub date: String,
}

pub struct Logger {
    pub car_num: usize,
    cur_lap: i32,
    pub save_data: SaveData,
    telemetry: Telemetry,
}

impl Logger {
    pub fn new(path: &str) -> Self {
        let telemetry = Telemetry::new(path);

        let cur_lap = 0;

        let mut save_data = SaveData::default();

        // Init the save data with correct data types but not values
        for i in 0..GRAPH_VIEW_DATA_TYPE_COUNT {
            let graph_type = GraphViewDataType::from_int(i, 0);

            save_data.lap_data.push(LapData {
                data_type: graph_type.to_string(),
                values: vec![],
            });
        }
        Self {
            car_num: 0,
            cur_lap,
            telemetry,
            save_data,
        }
    }

    pub fn add_datapoints(&mut self) -> bool {
        // Adds datapoint at current time to self.save_data
        // returns true if it detected new lap and doesn't add the datapoint

        let cur_data = self.telemetry.update_telemetry().unwrap();

        if self.cur_lap != cur_data.telemetry.telemetry_info[self.car_num].m_lap_number {
            self.cur_lap = cur_data.telemetry.telemetry_info[self.car_num].m_lap_number;
            return true;
        };

        for i in 0..GRAPH_VIEW_DATA_TYPE_COUNT {
            let graph_type = GraphViewDataType::from_int(i, self.car_num);
            self.save_data.lap_data[i as usize].values.push(vec2(
                graph_type.get_normalized_distance(&cur_data) as f32,
                graph_type.get_normalized_values(&cur_data) as f32,
            ));
        }

        self.save_data.pos_data.push(PosData {
            pos: cur_data.telemetry.telemetry_info[self.car_num].m_pos,
            time_since_lap_start: cur_data.scoring.veh_scoring_info[self.car_num].m_time_into_lap,
        });

        false
    }
    pub fn clear(&mut self) {
        // Clears the current contents of self.save_data

        for data in self.save_data.lap_data.iter_mut() {
            data.values.clear();
        }
        self.save_data.lap_info = LapInfo::default();
        self.save_data.pos_data = vec![];
    }
}
pub async fn save(mut save_data: SaveData, telemetry: Telemetry, car_num: usize, path: String) {
    // Saves the save data passed in after 100ms of delay due to telemetry refreshing. If the
    // lap was invalid it discards the data.
    println!("Tried to save");

    tokio::time::sleep(Duration::from_millis(100)).await;
    save_data.lap_info.date = Local::now().format("%d-%m-%Y-%H-%M-%S").to_string();

    save_data.lap_info.lap_time = telemetry
        .update_telemetry()
        .unwrap()
        .scoring
        .veh_scoring_info[car_num]
        .m_last_lap_time;

    if save_data.lap_info.lap_time > 0.0 {
        fs::write(
            path + &Local::now().format("%d-%m-%Y-%H-%M-%S").to_string() + ".json",
            serde_json::to_string(&save_data).unwrap(),
        )
        .unwrap();
    }
}
