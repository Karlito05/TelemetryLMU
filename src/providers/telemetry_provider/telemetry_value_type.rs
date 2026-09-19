use core::fmt;

use crate::providers::telemetry_provider::interface::{SharedMemoryObjectOut, TelemVect3};

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

            _ => Err(()),
        }
    }
}

impl fmt::Display for TelemetryValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Rpm => "rpm",
            Self::Speed => "speed",
            Self::Throttle => "throttle",
            Self::Brake => "brake",
            Self::Delta => "delta",
            Self::Gear => "gear",
            Self::Steering => "steering",

            Self::Max => panic!("Can't call to_string on TelemetryValueType::Max"),
        };
        write!(f, "{s}")
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

            Self::Max => panic!("Can't call get value on TelemetryValueType::Max"),
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

            Self::Max => panic!("Can't call get_max_value on TelemetryValueType::Max"),
        }
    }

    pub fn normalize(&self, v: f64, t: &SharedMemoryObjectOut, car_num: usize) -> f64 {
        match self {
            Self::Delta => v / (self.get_max_value(t, car_num) / 2.0) + 0.5,
            Self::Steering => v / self.get_max_value(t, car_num) + 0.5,

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
            Self::Steering => {
                for i in 0..n_gridlines {
                    let str = format!(
                        "{} {}",
                        (self.get_max_value(telemetry, car_num) * (n_gridlines - 1 - i) as f64
                            / (n_gridlines - 1) as f64
                            - 0.5)
                            * 180.0,
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

    pub fn get_all_string() -> Vec<String> {
        (0..TelemetryValueType::Max as usize)
            .map(|i| TelemetryValueType::try_from(i).unwrap().to_string())
            .collect()
    }

    pub fn get_time_into_lap(t: &SharedMemoryObjectOut, car_num: usize) -> f64 {
        t.telemetry.telemetry_info[car_num].m_elapsed_time
            - t.telemetry.telemetry_info[car_num].m_lap_start_et
    }

    pub fn get_distance_into_lap(t: &SharedMemoryObjectOut, car_num: usize) -> f64 {
        t.scoring.veh_scoring_info[car_num].m_lap_dist
    }

    #[expect(unused)]
    pub fn get_normalized_distance_into_lap(t: &SharedMemoryObjectOut, car_num: usize) -> f64 {
        t.scoring.veh_scoring_info[car_num].m_lap_dist / t.scoring.scoring_info.m_lap_dist
    }

    pub fn normalize_distance_into_lap(t: &SharedMemoryObjectOut, v: f64) -> f64 {
        v / t.scoring.scoring_info.m_lap_dist
    }

    pub fn get_pos(t: &SharedMemoryObjectOut, car_num: usize) -> TelemVect3 {
        t.telemetry.telemetry_info[car_num].m_pos
    }
}
