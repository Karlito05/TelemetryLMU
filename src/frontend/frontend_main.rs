use eframe::egui;

use crate::frontend::{car_info::CarInfo, settings::Settings, sidebar::Sidebar, telemetry_page};

pub struct App {
    sidebar: Sidebar,
    settings: Settings,
    telemetry: telemetry_page::TelemetryPage,
    car_info: CarInfo,
}

#[derive(Debug, PartialEq)]
pub enum Page {
    Telemetry,
    Info,
    Map,
    Settings,
}

// Default init
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sidebar.draw_sidebar(ui);

        egui::CentralPanel::default().show(ui, |ui| match self.sidebar.active_page {
            Page::Telemetry => self.telemetry.draw_telemetry_page(ui, &mut self.sidebar),
            Page::Info => {
                self.car_info
                    .draw_car_info_page(ui, &mut self.sidebar, &mut self.settings)
            }
            Page::Settings => {
                self.settings.draw_settings_page();
            }
            Page::Map => {}
        });
    }
}
