use eframe::egui;

use crate::{
    TOKIO,
    backend::lap_stores::{Logger, save},
    frontend::{car_info_page::CarInfo, settings_page::Settings, sidebar::Sidebar, telemetry_page},
    interface::Telemetry,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    sidebar: Sidebar,
    settings_page: Settings,
    telemetry_page: telemetry_page::TelemetryPage,
    #[serde(skip)]
    car_info_page: CarInfo,
    #[serde(skip)]
    logger: Logger,
    #[serde(skip)]
    telemetry: Telemetry,
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

impl Default for App {
    fn default() -> Self {
        Self {
            sidebar: Sidebar::default(),
            telemetry_page: telemetry_page::TelemetryPage::default(),
            settings_page: Settings::default(),
            car_info_page: CarInfo::default(),
            logger: Logger::new("/dev/shm/LMU_Data"),
            telemetry: Telemetry::new("/dev/shm/LMU_Data"),
        }
    }
}

// Main GUI entrypoint
impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sidebar.draw_sidebar(ui, &self.settings_page);

        if self.sidebar.settings_open {
            self.settings_page.draw_settings_page(ui, &mut self.sidebar);
        }

        if self.settings_page.record_laps && self.telemetry.full_mode {
            self.process_record_laps();
        }

        egui::CentralPanel::default().show(ui, |ui| match self.sidebar.active_page {
            Page::Telemetry => {
                self.telemetry_page
                    .draw_telemetry_page(ui, &mut self.sidebar, &self.settings_page)
            }
            Page::Info => self.car_info_page.draw_car_info_page(
                ui,
                &mut self.sidebar,
                &mut self.settings_page,
            ),
            Page::Map => {}
        });
    }
}

impl App {
    fn process_record_laps(&mut self) {
        if let Some(d) = self
            .telemetry
            .find_driver(self.settings_page.in_game_name.clone())
        {
            if d.1 as usize == self.logger.car_num {
                if self.logger.add_datapoints() {
                    TOKIO
                        .get()
                        .expect("tokio runtime not initialised")
                        .spawn(save(
                            self.logger.save_data.clone(),
                            Telemetry::new("/dev/shm/LMU_Data"),
                            self.logger.car_num,
                            self.settings_page.record_save_path.clone() + "/",
                        ));
                    self.logger.clear();
                }
            } else {
                self.logger.clear();
                self.logger.car_num = d.1 as usize;
            }
        }
    }
}
