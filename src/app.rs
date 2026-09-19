use crate::{
    components::{button::button, input::input, sidebar::sidebar_main::Sidebar, switch::switch},
    pages::{
        Page, car_info::car_info_main::CarInfo, map::map_main::MapPage,
        settings::settings_main::SettingsPage, telemetry::telemetry_main::TelemetryPage,
    },
    providers::{
        settings_provider::SettingsProvider,
        state_provider::StateProvider,
        telemetry_provider::{interface::Interface, telemetry_provider_main::Telemetry},
    },
};
use eframe::egui::*;
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct App {
    settings_provider: Arc<SettingsProvider>,
    state_provider: Arc<StateProvider>,
    telemetry_page: TelemetryPage,
    map_page: MapPage,
    car_info_page: CarInfo,
    #[serde(skip)]
    sidebar: Sidebar,
    #[serde(skip)]
    settings_page: SettingsPage,
    #[serde(skip)]
    interface: Interface,
    #[serde(skip)]
    telemetry_provider: Arc<Option<Telemetry>>,
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

        let first_launch_car_info_page = app.car_info_page.first_entry;
        app.car_info_page = CarInfo::new(
            app.settings_provider.clone(),
            app.state_provider.clone(),
            app.telemetry_provider.clone(),
        );
        app.car_info_page.first_entry = first_launch_car_info_page;

        let first_launch_map_page = app.map_page.first_entry;
        app.map_page = MapPage::new(app.settings_provider.clone(), app.state_provider.clone());
        app.map_page.first_entry = first_launch_map_page;

        let cur_layout_index = app.telemetry_page.cur_layout_index;
        let layouts = app.telemetry_page.layouts;
        let first_launch_telemetry_page = app.telemetry_page.first_entry;
        app.telemetry_page = TelemetryPage::new(
            app.settings_provider.clone(),
            app.state_provider.clone(),
            app.telemetry_provider.clone(),
        );
        app.telemetry_page.layouts = layouts;
        app.telemetry_page.cur_layout_index = cur_layout_index;
        app.telemetry_page.first_entry = first_launch_telemetry_page;

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
            telemetry_page: TelemetryPage::new(
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
            interface: Interface::new("/dev/shm/LMU_Data"),
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
                            );
                            let mut name = self.settings_provider.name.write().unwrap();
                            let name_len = name.len();
                            if name.len() >= 16 {
                                name.remove( name_len - 1);
                            }
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
                            if self.settings_provider.record_save_path.read().unwrap().is_empty() {
                                ui.label(RichText::new("Lap Save Path").size(20.0).color(Color32::RED));
                            } else {
                                ui.label(RichText::new("Lap Save Path").size(20.0).color(Color32::from_gray(200)));
                            }
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
                    #[expect(clippy::collapsible_if)]
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
                        if !self.settings_provider.record_save_path.read().unwrap().is_empty() {
                           *self.state_provider.global_first_launch.write().unwrap() = false;
                        }
                    }
                });
            });
    }
}
