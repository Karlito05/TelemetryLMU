use std::{path::PathBuf, str::FromStr, time::Duration};

use eframe::egui;

use crate::{
    TOKIO,
    backend::lap_stores::{Logger, save},
    frontend::{
        car_info_page::CarInfo, map_page::MapPage, settings_page::Settings, sidebar::Sidebar,
        telemetry_page,
    },
    telemetry::Telemetry,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct App {
    sidebar: Sidebar,
    settings_page: Settings,
    telemetry_page: telemetry_page::TelemetryPage,
    #[serde(skip)]
    map_page: MapPage,
    #[serde(skip)]
    car_info_page: CarInfo,
    #[serde(skip)]
    logger: Logger,
    #[serde(skip)]
    telemetry: crate::interface::Telemetry,
    #[serde(skip)]
    telemetry_interface: Telemetry,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Page {
    Telemetry,
    Info,
    Map,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: App = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        // rebuild the GPU texture from the persisted bytes
        app.settings_page.restore_pfp(&cc.egui_ctx);

        app
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            sidebar: Sidebar::default(),
            map_page: MapPage::default(),
            telemetry_page: telemetry_page::TelemetryPage::default(),
            settings_page: Settings::default(),
            car_info_page: CarInfo::default(),
            logger: Logger::new("/dev/shm/LMU_Data"),
            telemetry: crate::interface::Telemetry::new("/dev/shm/LMU_Data"),
            telemetry_interface: Telemetry::new("/dev/shm/LMU_Data".into()).unwrap(),
        }
    }
}

// Main GUI entrypoint
impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.request_repaint_after(Duration::from_millis(16));
        self.sidebar.draw_sidebar(ui, &self.settings_page);

        if self.sidebar.settings_open {
            self.settings_page.draw_settings_page(ui, &mut self.sidebar);
        }

        if self.settings_page.record_laps && self.telemetry.full_mode {
            self.process_record_laps();
        }

        // egui::Window::new("Debug").show(ui.ctx(), |ui| {
        //     ui.ctx().clone().inspection_ui(ui);
        //     ui.ctx().clone().memory_ui(ui);
        //     ui.ctx().clone().settings_ui(ui);
        //     ui.ctx().clone().texture_ui(ui);
        // });

        egui::CentralPanel::default().show(ui, |ui| match self.sidebar.active_page {
            Page::Telemetry => self.telemetry_page.draw_telemetry_page(
                ui,
                &mut self.sidebar,
                &self.settings_page,
                &self.telemetry_interface,
            ),
            Page::Info => self.car_info_page.draw_car_info_page(
                ui,
                &mut self.sidebar,
                &mut self.settings_page,
            ),
            Page::Map => self
                .map_page
                .draw_map_page(ui, &self.sidebar, &self.settings_page),
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
                            crate::interface::Telemetry::new("/dev/shm/LMU_Data"),
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
