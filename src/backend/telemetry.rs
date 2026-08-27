use crate::interface::SharedMemoryObjectOut;

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

    #[expect(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
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

    pub fn get_normalized_values(&self, t: &SharedMemoryObjectOut) -> f64 {
        match self {
            GraphViewDataType::Rpm(v) => {
                t.telemetry.telemetry_info[*v].m_engine_rpm / self.get_max_value(t)
            }
            GraphViewDataType::Speed(v) => {
                -t.telemetry.telemetry_info[*v].m_local_vel.z * 3.6 / self.get_max_value(t)
            }
            GraphViewDataType::Throttle(v) => {
                t.telemetry.telemetry_info[*v].m_unfiltered_throttle / self.get_max_value(t)
            }
            GraphViewDataType::Brake(v) => {
                t.telemetry.telemetry_info[*v].m_unfiltered_brake / self.get_max_value(t)
            }
            GraphViewDataType::Delta(v, r) => {
                (t.telemetry.telemetry_info[*v].m_delta_best.clamp(-*r, *r) + *r)
                    / self.get_max_value(t)
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

    pub fn get_unit_labels(
        &self,
        telemetry: &SharedMemoryObjectOut,
        n_gridlines: i32,
    ) -> Vec<String> {
        let mut ret = vec![];
        for i in 0..n_gridlines {
            let str = format!(
                "{} {}",
                self.get_max_value(telemetry) * (n_gridlines - 1 - i) as f64
                    / (n_gridlines - 1) as f64,
                self.get_unit()
            );
            ret.push(str);
        }
        ret
    }

    pub fn get_normalized_distance(&self, telemetry: &SharedMemoryObjectOut) -> f64 {
        // this returns the distance of how far in a lap the car is
        telemetry.scoring.veh_scoring_info[self.get_car_number()].m_lap_dist
            / telemetry.scoring.scoring_info.m_lap_dist
    }

    pub fn get_lap(&self, t: &SharedMemoryObjectOut) -> i32 {
        t.telemetry.telemetry_info[self.get_car_number()].m_lap_number
    }

    pub fn get_all_string() -> Vec<String> {
        (0..GRAPH_VIEW_DATA_TYPE_COUNT)
            .map(|i| GraphViewDataType::from_int(i, 0).to_string())
            .collect()
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
