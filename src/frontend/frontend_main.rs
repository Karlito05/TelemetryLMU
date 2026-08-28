use eframe::egui;

use crate::frontend::{
    car_info_page::CarInfo, settings_page::Settings, sidebar::Sidebar, telemetry_page,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    sidebar: Sidebar,
    settings: Settings,
    telemetry: telemetry_page::TelemetryPage,
    #[serde(skip)]
    car_info: CarInfo,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Page {
    Telemetry,
    Info,
    Map,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

#[expect(clippy::derivable_impls)]
impl Default for App {
    fn default() -> Self {
        Self {
            sidebar: Sidebar::default(),
            telemetry: telemetry_page::TelemetryPage::default(),
            settings: Settings::default(),
            car_info: CarInfo::default(),
        }
    }
}

// Main GUI entrypoint
impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sidebar.draw_sidebar(ui, &self.settings);

        if self.sidebar.settings_open {
            self.settings.draw_settings_page(ui, &mut self.sidebar);
        }

        egui::CentralPanel::default().show(ui, |ui| match self.sidebar.active_page {
            Page::Telemetry => self.telemetry.draw_telemetry_page(ui, &mut self.sidebar),
            Page::Info => {
                self.car_info
                    .draw_car_info_page(ui, &mut self.sidebar, &mut self.settings)
            }
            Page::Map => {}
        });
    }
}
