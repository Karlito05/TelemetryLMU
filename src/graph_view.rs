use std::ops::Deref;
use dioxus::prelude::*;
use crate::telemetry::{SharedMemoryObjectOut};
use crate::TELEMETRY;


#[component]
pub fn GraphView(data_type: GraphViewDataType, width: i32, height: i32) -> Element {
    let mut data: Signal<Vec<(f64, f64)>> = use_signal(|| vec!());
    let mut lap: i32 = 0;

    data.write().push((data_type.get_normalized_value(&TELEMETRY.read()), data_type.get_normalized_distance(&TELEMETRY.read())));

    if lap < data_type.get_lap(&TELEMETRY.read()) && data_type.get_normalized_distance(&TELEMETRY.read()) < 0.1 {
        lap = data_type.get_lap(&TELEMETRY.read());
        data.write().clear();
    }

    rsx! {
        svg { width: width, height: height,
                // Draw a simple polyline from data
            polyline {
                fill: "none",
                stroke: "white",
                stroke_width: "1",
                points: data.read()
                    .iter()
                    .map(|(nx, ny)| {
                        let x = ny * width as f64; // Flip Y so 0.0 is at the bottom
                        let y = height as f64 - (nx * height as f64);
                        format!("{x},{y}")
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            polyline {
                fill: "none",
                stroke: "white",
                stroke_width: "2",
                points: "0, 0, 0, {height}, {width}, {height},  {width}, 0, 0, 0"
            }

            for i in (1..5).rev() {
                text {
                    fill: "white",
                    font_size: "16px",
                    opacity: 0.6,
                    x: 5,
                    y: ((i as f64 / 5.0) * height as f64) - 4.0,
                    "{data_type.get_line_format(&TELEMETRY.read(), 5, -(i-5))}"
                }
                line {
                    x1: "0", y1: (i as f64 / 5.0) * height as f64,
                    x2: "{width}", y2: (i as f64 / 5.0) * height as f64,
                    stroke: "white",
                    stroke_width: "0.5",
                    // 10px dash, 5px gap
                    stroke_dasharray: "10, 5"
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GraphViewDataType {
    Rpm (usize), //vehicle number
    Speed (usize), //vehicle number
    Throttle (usize), //vehicle number
    Brake (usize), //vehicle number
    Delta (usize, f64), //vehicle number, range (how much up and down should the normalized value be referencing in seconds)
}

impl GraphViewDataType {
    fn get_max_value(&self, t: &SharedMemoryObjectOut) -> f64 {
        match self {
            GraphViewDataType::Rpm(v) => {t.telemetry.telemetry_info[*v].m_engine_max_rpm}
            GraphViewDataType::Speed(..) => {350.0} // This can be implemented conditionally based on the class :)
            GraphViewDataType::Throttle(..) => {1.0}
            GraphViewDataType::Brake(..) => {1.0}
            GraphViewDataType::Delta(_, r) => {*r * 2.0}
        }
    }

    fn get_normalized_value(&self, t: &SharedMemoryObjectOut) -> f64 {
        match self {
            GraphViewDataType::Rpm(v) => {t.telemetry.telemetry_info[*v].m_engine_rpm / self.get_max_value(t)}
            GraphViewDataType::Speed(v) => {-t.telemetry.telemetry_info[*v].m_local_vel.z * 3.6 / self.get_max_value(t)}
            GraphViewDataType::Throttle(v) => {t.telemetry.telemetry_info[*v].m_filtered_throttle / self.get_max_value(t)}
            GraphViewDataType::Brake(v) => {t.telemetry.telemetry_info[*v].m_filtered_brake / self.get_max_value(t)}
            GraphViewDataType::Delta(v, r) => {t.telemetry.telemetry_info[*v].m_delta_best.clamp(-*r, *r) + *r / self.get_max_value(t)}
        }
    }

    fn get_car_number(&self) -> usize {
        match self {
            GraphViewDataType::Rpm(v, ..) => {*v}
            GraphViewDataType::Speed(v, ..) => {*v}
            GraphViewDataType::Throttle(v, ..) => {*v}
            GraphViewDataType::Brake(v, ..) => {*v}
            GraphViewDataType::Delta(v, ..) => {*v}
        }
    }

    fn get_line_format(&self,t: &SharedMemoryObjectOut ,n_lines: i32, line: i32) -> String {
        match self {
            GraphViewDataType::Rpm(v, ..) => {format!("{} RPM", (line as f64 / n_lines as f64) * self.get_max_value(t))}
            GraphViewDataType::Speed(v, ..) => {format!("{} KM/H", (line as f64 / n_lines as f64) * self.get_max_value(t))}
            GraphViewDataType::Throttle(v, ..) => {format!("{}%", (line as f64 / n_lines as f64) * self.get_max_value(t))}
            GraphViewDataType::Brake(v, ..) => {format!("{}%", (line as f64 / n_lines as f64) * self.get_max_value(t))}
            GraphViewDataType::Delta(v, s, ..) => {format!("{} s",(line as f64 / n_lines as f64) * self.get_max_value(t) - s)}
        }
    }

    fn get_normalized_distance(&self, telemetry: &SharedMemoryObjectOut) -> f64 {
        // this returns the distance of how far in a lap the car is
        telemetry.scoring.veh_scoring_info[self.get_car_number()].m_lap_dist / telemetry.scoring.scoring_info.m_lap_dist
    }

    fn get_lap(&self, t: &SharedMemoryObjectOut) -> i32 {
        t.telemetry.telemetry_info[self.get_car_number()].m_lap_number
    }
}

