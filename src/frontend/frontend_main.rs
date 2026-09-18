use core::fmt;
use std::sync::{Arc, RwLock};

use eframe::egui::*;

use crate::{
    frontend::{
        car_info_page::CarInfo,
        components::{button, input, switch},
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
    pub global_first_launch: RwLock<bool>,
    #[serde(skip)]
    pub page: RwLock<Page>,
    #[serde(skip)]
    pub sidebar_open: RwLock<bool>,
    #[serde(skip)]
    pub settings_open: RwLock<bool>,
}
impl Default for StateProvider {
    fn default() -> Self {
        Self {
            global_first_launch: RwLock::new(true),
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
    map_page: MapPage,
    #[serde(skip)]
    car_info_page: CarInfo,
    #[serde(skip)]
    interface: crate::interface::Interface,
    #[serde(skip)]
    telemetry_provider: Arc<Option<Telemetry>>,
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
        app.telemetry_provider = Arc::new(
            Telemetry::new("/dev/shm/LMU_Data".into(), app.settings_provider.clone()).ok(),
        );

        app.sidebar = Sidebar::new(app.settings_provider.clone(), app.state_provider.clone());
        app.settings_page =
            SettingsPage::new(app.settings_provider.clone(), app.state_provider.clone());
        app.car_info_page = CarInfo::new(
            app.settings_provider.clone(),
            app.state_provider.clone(),
            app.telemetry_provider.clone(),
        );

        let first_launch_map_page = app.map_page.first_entry;
        app.map_page = MapPage::new(app.settings_provider.clone(), app.state_provider.clone());
        app.map_page.first_entry = first_launch_map_page;

        let cur_layout_index = app.telemetry_page.cur_layout_index;
        let layouts = app.telemetry_page.layouts;
        app.telemetry_page = TelemetryPage::new(
            app.settings_provider.clone(),
            app.state_provider.clone(),
            app.telemetry_provider.clone(),
        );

        app.telemetry_page.layouts = layouts;
        app.telemetry_page.cur_layout_index = cur_layout_index;

        app.settings_page.restore_pfp(&cc.egui_ctx);
        app
    }
}

impl Default for App {
    fn default() -> Self {
        let settings_provider = Arc::new(SettingsProvider::default());
        let state_provider = Arc::new(StateProvider::default());
        let telemetry_provider =
            Arc::new(Telemetry::new("/dev/shm/LMU_Data".into(), settings_provider.clone()).ok());
        Self {
            sidebar: Sidebar::new(settings_provider.clone(), state_provider.clone()),
            map_page: MapPage::new(settings_provider.clone(), state_provider.clone()),
            telemetry_page: telemetry_page::TelemetryPage::new(
                settings_provider.clone(),
                state_provider.clone(),
                telemetry_provider.clone(),
            ),
            settings_page: SettingsPage::new(settings_provider.clone(), state_provider.clone()),
            car_info_page: CarInfo::new(
                settings_provider.clone(),
                state_provider.clone(),
                telemetry_provider.clone(),
            ),
            interface: crate::interface::Interface::new("/dev/shm/LMU_Data"),
            telemetry_provider,
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
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // ui.request_repaint_after(Duration::from_millis(16));
        self.sidebar.draw_sidebar(ui);

        if *self.state_provider.settings_open.read().unwrap() {
            self.settings_page.draw_settings_page(ui);
        }

        if *self.state_provider.global_first_launch.read().unwrap() {
            self.show_onboarding_global(ui);
        }

        CentralPanel::default().show(ui, |ui| match *self.state_provider.page.read().unwrap() {
            Page::Telemetry => self.telemetry_page.draw_telemetry_page(ui),
            Page::Info => self.car_info_page.draw_car_info_page(ui),
            Page::Map => self.map_page.draw_map_page(ui),
        });
    }
}

impl App {
    fn show_onboarding_global(&mut self, ui: &mut Ui) {
        Window::new("Hello")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.set_min_size(vec2(500.0, 400.0));
                ui.vertical(|ui| {
                    ui.label(RichText::new("Hello 👋").size(32.0).strong().color(Color32::WHITE));
                    ui.separator();
                    ui.label(RichText::new("To get started, please fill out the following values: ").size(24.0).color(Color32::WHITE));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Name").size(20.0).color(Color32::from_gray(200)));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            input(
                                ui,
                                vec2(200.0, 32.0),
                                CornerRadius::same(8),
                                Stroke::new(1.0, Color32::from_gray(127)),
                                Color32::from_white_alpha(17),
                                &mut self.settings_provider.name.write().unwrap(),
                                FontSelection::FontId(FontId::proportional(16.0)),
                                32,
                            )
                        })
                    });
                    ui.add_space(4.0);
                    ui.label(RichText::new("Name is just a cosmetic thing. You can enter your real name or a nickname. The app doesn't really care!").size(16.0));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("In Game Name").size(20.0).color(Color32::from_gray(200)));

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            input(
                                ui,
                                vec2(200.0, 32.0),
                                CornerRadius::same(8),
                                Stroke::new(1.0, Color32::from_gray(127)),
                                Color32::from_white_alpha(17),
                                &mut self.settings_provider.in_game_name.write().unwrap(),
                                FontSelection::FontId(FontId::proportional(16.0)),
                                32,
                            )
                        })
                    });
                    ui.add_space(4.0);
                    ui.label(RichText::new("In game name must be your exact in game name or this app won't funtion propertly.").size(16.0));

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Record Laps").size(20.0).color(Color32::from_gray(200)));

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let current = *self.settings_provider.record_laps.read().unwrap();
                            let new_value = switch(
                                ui,
                                vec2(48.0, 24.0),
                                CornerRadius::same(8),
                                Color32::from_white_alpha(25),
                                Color32::from_rgb(19, 141, 241),
                                current,
                            );
                            *self.settings_provider.record_laps.write().unwrap() = new_value;
                        })
                    });
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Record All Cars").size(20.0).color(Color32::from_gray(200)));

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let current = *self.settings_provider.log_all_cars.read().unwrap();
                            let new_value = switch(
                                ui,
                                vec2(48.0, 24.0),
                                CornerRadius::same(8),
                                Color32::from_white_alpha(25),
                                Color32::from_rgb(19, 141, 241),
                                current,
                            );
                            *self.settings_provider.log_all_cars.write().unwrap() = new_value;
                        })
                    });
                    ui.add_space(4.0);
                    ui.label(RichText::new("With these 2 toggles you can choose if you want your laps to be recorded and if you want to record everyone in the lobby.").size(16.0));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new("Lap Save Path").size(20.0).color(Color32::from_gray(200)));
                            ui.label(
                                RichText::new(
                                    &*self.settings_provider.record_save_path.read().unwrap(),
                                )
                                    .size(12.0),
                            );
                        });

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if button(
                                ui,
                                vec2(140.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_white_alpha(25),
                                "Browse",
                                FontId::new(16.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                                .clicked()
                            {
                                let current_path = self
                                    .settings_provider
                                    .record_save_path
                                    .read()
                                    .unwrap()
                                    .clone();

                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Select folder")
                                    .set_directory(current_path)
                                    .add_filter("All files", &["*"])
                                    .set_can_create_directories(true)
                                    .pick_folder()
                                {
                                    *self.settings_provider.record_save_path.write().unwrap() =
                                        path.display().to_string();
                                }
                            }
                        })
                    });
                    ui.add_space(4.0);
                    ui.label(RichText::new("Here you must select a folder where your laps should be stored!").size(16.0));
                    ui.separator();
                    ui.label(RichText::new("You can change all of these and more in the settings which you can get to by clicking the settings icon on the sidebar!").size(20.0));
                    if button(
                        ui,
                        vec2(64.0, 32.0),
                        CornerRadius::same(8),
                        Color32::from_white_alpha(25),
                        "Close",
                        FontId::new(16.0, FontFamily::Proportional),
                        Color32::WHITE,
                    )
                    .clicked()
                    {
                        *self.state_provider.global_first_launch.write().unwrap() = false;
                    }
                });
            });
    }
}
