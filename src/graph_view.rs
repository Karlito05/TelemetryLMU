use std::string::ToString;
use dioxus::html::object::data;
use dioxus::prelude::*;
use crate::telemetry::{SharedMemoryObjectOut};
use crate::TELEMETRY;


#[component]
pub fn GraphView(data_type: GraphViewDataType, width: i32, height: i32, style: Option<GraphViewStyle>) -> Element {
    let mut current_lap: Signal<Vec<(Vec<f64>, f64)>> = use_signal(|| vec!());
    let mut best_lap: Signal<Vec<(Vec<f64>, f64)>> = use_signal(|| vec!());
    let mut lap = use_signal(|| 0);
    let mut style = style.unwrap_or(GraphViewStyle {
        main_color: "white".to_string(),
        current_lap_colors: vec!["red".to_string(), "blue".to_string(), "green".to_string(), "yellow".to_string()],
        best_lap_color: vec!["rgba(255, 0, 0, 0.5)".to_string(), "rgba(0, 0, 255, 0.5)".to_string(), "rgba(0, 255, 0, 0.5)".to_string(), "rgba(255, 255, 0, 0.5)".to_string()],
        interline_count: 4,
    });

    style.interline_count += 1;

    current_lap.write().push((data_type.get_normalized_values(&TELEMETRY.read()), data_type.get_normalized_distance(&TELEMETRY.read())));

    if *lap.read() < data_type.get_lap(&TELEMETRY.read()) && data_type.get_normalized_distance(&TELEMETRY.read()) < 0.1 {

        println!("New Lap");

        *lap.write() = data_type.get_lap(&TELEMETRY.read());

        if data_type.is_last_best(&TELEMETRY.read()) {
            *best_lap.write() = {
                let mut last_dist = 0.0;
                current_lap.read().iter().filter_map(|(values, distance)| {

                    if last_dist - 0.05 <= *distance { // -0.05 just to give it some leeway
                        last_dist = *distance;
                        Some((values.clone(), *distance))
                    } else {
                        None
                    }
                }).collect()
            };
        }

        current_lap.write().clear();
    }

    rsx! {
        svg { width: width, height: height,
            if current_lap.len() > 0 {
                for i in 0..current_lap.read()[0].0.len() {
                    polyline {
                        fill: "none",
                        stroke: style.current_lap_colors[i].clone(),
                        stroke_width: 1.5,
                        points: current_lap.read().iter().map(|(values, distance)| {
                            format!("{}, {}, ", (distance * width as f64), (1.0 - values[i]) * height as f64)
                        }).collect::<String>()
                    }
                }
            }
            if best_lap.len() > 0 {
                for i in 0..best_lap.read()[0].0.len() {
                    polyline {
                        fill: "none",
                        stroke: style.best_lap_color[i].clone(),
                        stroke_width: 1,
                        points: best_lap.read().iter().map(|(values, distance)| {
                            format!("{}, {}, ", (distance * width as f64), (1.0 - values[i]) * height as f64)
                        }).collect::<String>()
                    }
                }
            }

            polyline {
                fill: "none",
                stroke: style.main_color.clone(),
                stroke_width: "2",
                points: "0, 0, 0, {height}, {width}, {height},  {width}, 0, 0, 0"
            }

            for i in (1..style.interline_count).rev() {
                text {
                    fill: style.main_color.clone(),
                    font_size: "16px",
                    opacity: 0.6,
                    x: 5,
                    y: ((i as f64 / style.interline_count as f64) * height as f64) - 4.0,
                    "{data_type.get_line_format(&TELEMETRY.read(), style.interline_count, -(i-style.interline_count))}"
                }
                line {
                    x1: "0", y1: (i as f64 / style.interline_count as f64) * height as f64,
                    x2: "{width}", y2: (i as f64 / style.interline_count as f64) * height as f64,
                    stroke: style.main_color.clone(),
                    stroke_width: "0.5",
                    // 10px dash, 5px gap
                    stroke_dasharray: "10, 5"
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct GraphViewStyle {
    pub main_color: String,
    pub current_lap_colors: Vec<String>, // note that if there are only x lines only the x colors will be used, the rest will be ignored
    pub best_lap_color: Vec<String>, // note that if there are only x lines only the x colors will be used, the rest will be ignored
    pub interline_count: i32, // how many lines should be drawn in the background (for reference)
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GraphViewDataType {
    Rpm (usize), //vehicle number
    Speed (usize), //vehicle number
    Throttle (usize), //vehicle number
    Brake (usize), //vehicle number
    Delta (usize, f64), //vehicle number, range (how much up and down should the normalized value be referencing in seconds)
    Pedals (usize),
}

impl GraphViewDataType {
    fn get_max_value(&self, t: &SharedMemoryObjectOut) -> f64 {
        match self {
            GraphViewDataType::Rpm(v) => {t.telemetry.telemetry_info[*v].m_engine_max_rpm}
            GraphViewDataType::Speed(..) => {350.0} // This can be implemented conditionally based on the class :)
            GraphViewDataType::Throttle(..) => {1.0}
            GraphViewDataType::Brake(..) => {1.0}
            GraphViewDataType::Delta(_, r) => {*r * 2.0}
            GraphViewDataType::Pedals(..) => {1.0}
        }
    }

    fn get_normalized_values(&self, t: &SharedMemoryObjectOut) -> Vec<f64> {
        match self {
            GraphViewDataType::Rpm(v) => {vec!(t.telemetry.telemetry_info[*v].m_engine_rpm / self.get_max_value(t))}
            GraphViewDataType::Speed(v) => {vec!(-t.telemetry.telemetry_info[*v].m_local_vel.z * 3.6 / self.get_max_value(t))}
            GraphViewDataType::Throttle(v) => {vec!(t.telemetry.telemetry_info[*v].m_unfiltered_throttle / self.get_max_value(t))}
            GraphViewDataType::Brake(v) => {vec!(t.telemetry.telemetry_info[*v].m_unfiltered_brake / self.get_max_value(t))}
            GraphViewDataType::Delta(v, r) => {vec!((t.telemetry.telemetry_info[*v].m_delta_best.clamp(-*r, *r) + *r) / self.get_max_value(t))}
            GraphViewDataType::Pedals(v) => {vec!(t.telemetry.telemetry_info[*v].m_unfiltered_brake / self.get_max_value(t), t.telemetry.telemetry_info[*v].m_unfiltered_throttle / self.get_max_value(t))}
        }
    }

    fn get_car_number(&self) -> usize {
        match self {
            GraphViewDataType::Rpm(v, ..) => {*v}
            GraphViewDataType::Speed(v, ..) => {*v}
            GraphViewDataType::Throttle(v, ..) => {*v}
            GraphViewDataType::Brake(v, ..) => {*v}
            GraphViewDataType::Delta(v, ..) => {*v}
            GraphViewDataType::Pedals(v, ..) => {*v}
        }
    }

    fn get_line_format(&self,t: &SharedMemoryObjectOut ,n_lines: i32, line: i32) -> String {
        match self {
            GraphViewDataType::Rpm(..) => {format!("{} RPM", (line as f64 / n_lines as f64) * self.get_max_value(t))}
            GraphViewDataType::Speed(..) => {format!("{} KM/H", (line as f64 / n_lines as f64) * self.get_max_value(t))}
            GraphViewDataType::Throttle(..) => {format!("{}%", (line as f64 / n_lines as f64) * self.get_max_value(t) * 100.0)}
            GraphViewDataType::Brake(..) => {format!("{}%", (line as f64 / n_lines as f64) * self.get_max_value(t) * 100.0)}
            GraphViewDataType::Delta(_, s, ..) => {format!("{} s",(line as f64 / n_lines as f64) * self.get_max_value(t) - s)}
            GraphViewDataType::Pedals(..) => {format!("{}%", (line as f64 / n_lines as f64) * self.get_max_value(t) * 100.0)}
        }
    }

    fn get_normalized_distance(&self, telemetry: &SharedMemoryObjectOut) -> f64 {
        // this returns the distance of how far in a lap the car is
        telemetry.scoring.veh_scoring_info[self.get_car_number()].m_lap_dist / telemetry.scoring.scoring_info.m_lap_dist
    }

    fn get_lap(&self, t: &SharedMemoryObjectOut) -> i32 {
        t.telemetry.telemetry_info[self.get_car_number()].m_lap_number
    }

    fn is_last_best(&self, t: &SharedMemoryObjectOut) -> bool {
        match self {
            GraphViewDataType::Delta(..) => {false} // we can disable having the best lap for certain types where it isn't needed
            _ => {t.scoring.veh_scoring_info[self.get_car_number()].m_last_lap_time <= t.scoring.veh_scoring_info[self.get_car_number()].m_best_lap_time}
        }
    }
}

// test