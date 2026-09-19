pub mod car_info;
pub mod map;
pub mod settings;
pub mod telemetry;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, Default)]
pub enum Page {
    #[default]
    Telemetry,
    Info,
    Map,
}
