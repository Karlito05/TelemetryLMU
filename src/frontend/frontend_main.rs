use core::fmt;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use eframe::egui::{self, TextureHandle};

use crate::{
    backend::lap_stores::Logger,
    frontend::{
        car_info_page::CarInfo,
        map_page::MapPage,
        settings_page::SettingsPage,
        sidebar::Sidebar,
        telemetry_page::{self, TelemetryPage},
    },
    telemetry::Telemetry,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct StateProvider {
    pub page: RwLock<Page>,
    pub sidebar_open: RwLock<bool>,
    pub settings_open: RwLock<bool>,
}
impl Default for StateProvider {
    fn default() -> Self {
        Self {
            page: RwLock::new(Page::Telemetry),
            sidebar_open: RwLock::new(true),
            settings_open: RwLock::new(false),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct SettingsProvider {
    pub name: RwLock<String>,
    pub in_game_name: RwLock<String>,
    pub record_laps: RwLock<bool>,
    pub record_save_path: RwLock<String>,
    pub pfp_bytes: RwLock<Option<Vec<u8>>>,
    #[serde(skip)] // textures can't be serialized, recreate on load
    pub pfp_texture: RwLock<Option<TextureHandle>>,
    pub log_all_cars: RwLock<bool>,
}

impl fmt::Debug for SettingsProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsProvider")
            .field("name", &self.name)
            .field("in_game_name", &self.in_game_name)
            .field("record_laps", &self.record_laps)
            .field("record_save_path", &self.record_save_path)
            .field("pfp_bytes", &self.pfp_bytes)
            .field("log_all_cars", &self.log_all_cars)
            .finish()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct App {
    settings_provider: Arc<SettingsProvider>,
    state_provider: Arc<StateProvider>,
    telemetry_page: telemetry_page::TelemetryPage,
    #[serde(skip)]
    sidebar: Sidebar,
    #[serde(skip)]
    settings_page: SettingsPage,
    #[serde(skip)]
    map_page: MapPage,
    #[serde(skip)]
    car_info_page: CarInfo,
    #[serde(skip)]
    logger: Logger,
    #[serde(skip)]
    interface: crate::interface::Interface,
    #[serde(skip)]
    telemetry: Telemetry,
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize, Default)]
pub enum Page {
    #[default]
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

        app.sidebar = Sidebar::new(app.settings_provider.clone(), app.state_provider.clone());
        app.settings_page =
            SettingsPage::new(app.settings_provider.clone(), app.state_provider.clone());
        app.map_page = MapPage::new(app.settings_provider.clone(), app.state_provider.clone());
        app.car_info_page = CarInfo::new(app.settings_provider.clone(), app.state_provider.clone());

        let cur_layout_index = app.telemetry_page.cur_layout_index;
        let layouts = app.telemetry_page.layouts;
        app.telemetry_page =
            TelemetryPage::new(app.settings_provider.clone(), app.state_provider.clone());

        app.telemetry_page.layouts = layouts;
        app.telemetry_page.cur_layout_index = cur_layout_index;

        app.telemetry =
            Telemetry::new("/dev/shm/LMU_Data".into(), app.settings_provider.clone()).unwrap();

        app.settings_page.restore_pfp(&cc.egui_ctx);
        app
    }
}

impl Default for App {
    fn default() -> Self {
        let settings_provider = Arc::new(SettingsProvider::default());
        let state_provider = Arc::new(StateProvider::default());
        Self {
            sidebar: Sidebar::new(settings_provider.clone(), state_provider.clone()),
            map_page: MapPage::new(settings_provider.clone(), state_provider.clone()),
            telemetry_page: telemetry_page::TelemetryPage::new(
                settings_provider.clone(),
                state_provider.clone(),
            ),
            settings_page: SettingsPage::new(settings_provider.clone(), state_provider.clone()),
            car_info_page: CarInfo::new(settings_provider.clone(), state_provider.clone()),
            logger: Logger::new("/dev/shm/LMU_Data"),
            interface: crate::interface::Interface::new("/dev/shm/LMU_Data"),
            telemetry: Telemetry::new("/dev/shm/LMU_Data".into(), settings_provider.clone())
                .unwrap(),
            state_provider,
            settings_provider,
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
        self.sidebar.draw_sidebar(ui);

        if *self.state_provider.settings_open.read().unwrap() {
            self.settings_page.draw_settings_page(ui);
        }

        // TODO: Handle this in the main telemetry
        // if self.settings_page.record_laps && self.telemetry.full_mode {
        //     self.process_record_laps();
        // }

        // egui::Window::new("Debug").show(ui.ctx(), |ui| {
        //     ui.ctx().clone().inspection_ui(ui);
        //     ui.ctx().clone().memory_ui(ui);
        //     ui.ctx().clone().settings_ui(ui);
        //     ui.ctx().clone().texture_ui(ui);
        // });

        egui::CentralPanel::default().show(ui, |ui| {
            match *self.state_provider.page.read().unwrap() {
                Page::Telemetry => self.telemetry_page.draw_telemetry_page(ui, &self.telemetry),
                Page::Info => self.car_info_page.draw_car_info_page(ui),
                Page::Map => self.map_page.draw_map_page(ui),
            }
        });
    }
}

// impl App {
//     fn process_record_laps(&mut self) {
//         if let Some(d) = self
//             .telemetry
//             .find_driver(self.settings_page.in_game_name.clone())
//         {
//             if d.1 as usize == self.logger.car_num {
//                 if self.logger.add_datapoints() {
//                     TOKIO
//                         .get()
//                         .expect("tokio runtime not initialised")
//                         .spawn(save(
//                             self.logger.save_data.clone(),
//                             crate::interface::Telemetry::new("/dev/shm/LMU_Data"),
//                             self.logger.car_num,
//                             self.settings_page.record_save_path.clone() + "/",
//                         ));
//                     self.logger.clear();
//                 }
//             } else {
//                 self.logger.clear();
//                 self.logger.car_num = d.1 as usize;
//             }
//         }
//     }
// }
