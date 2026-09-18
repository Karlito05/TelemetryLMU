use std::{sync::Arc, time::Duration};

use eframe::egui::*;

use crate::{
    frontend::{
        components::{button, telemetry_not_found},
        frontend_main::{SettingsProvider, StateProvider},
    },
    interface::{IPVehicleClass, i8_array32_to_string, i8_array64_to_string},
    telemetry::Telemetry,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct CarInfo {
    pub first_entry: bool,
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    car: String,
    #[serde(skip)]
    car_class: IPVehicleClass,
    #[serde(skip)]
    driver_index: usize,
    #[serde(skip)]
    fuel_info: FuelInfo,
    #[serde(skip)]
    tires: [TireInfo; 4],
    #[serde(skip)]
    telemetry_provider: Arc<Option<Telemetry>>,
    #[serde(skip)]
    settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    state_provider: Arc<StateProvider>,
}

#[derive(Debug, Default)]
pub struct FuelInfo {
    pub fuel_percent: f32,
    pub virt_eng_percent: f32,
    pub fuel_liters: f32,
}

#[derive(Default, Debug)]
pub struct TireInfo {
    pub inside_temp: f32,
    pub outside_temp: f32,
    pub brake_temp: f32,
    pub health_percent: f32,
}
pub struct StaleDriverInfo {
    pub name: String,
    pub car: String,
    pub car_class: IPVehicleClass,
    pub index: usize,
}

pub struct DynDriverInfo {
    pub tires: [TireInfo; 4],
    pub fuel: FuelInfo,
}

impl CarInfo {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
        telemetry_provider: Arc<Option<Telemetry>>,
    ) -> Self {
        Self {
            first_entry: true,
            telemetry_provider,
            driver_index: 0,
            name: "".to_string(),
            car: "".to_string(),
            car_class: IPVehicleClass::Unknown,
            fuel_info: FuelInfo {
                fuel_percent: 0.0,
                virt_eng_percent: 0.0,
                fuel_liters: 0.0,
            },
            tires: [
                TireInfo::default(),
                TireInfo::default(),
                TireInfo::default(),
                TireInfo::default(),
            ],
            state_provider,
            settings_provider,
        }
    }
}

impl CarInfo {
    pub fn draw_car_info_page(&mut self, ui: &mut Ui) {
        ui.request_repaint_after(Duration::from_millis(16));

        if self.first_entry && !*self.state_provider.global_first_launch.read().unwrap() {
            self.draw_first_entry_dialog(ui);
        }

        if self.telemetry_provider.is_none() {
            if !self.first_entry {
                telemetry_not_found(ui);
            }
            return;
        }

        if self.name.is_empty()
            && let Ok(info) = self.get_stale_driver_info()
        {
            self.name = info.name;
            self.car = info.car;
            self.car_class = info.car_class;
            self.driver_index = info.index;
        }

        if !self.name.is_empty() {
            let dyn_driver_inf = self.get_dyn_driver_info();
            self.tires = dyn_driver_inf.tires;
            self.fuel_info = dyn_driver_inf.fuel;
        }

        let rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                16.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        8.0
                    } else {
                        16.0
                    },
                ui.available_height() - 16.0,
            ),
        );

        ui.painter()
            .rect_filled(rect, CornerRadius::same(24), Color32::from_rgb(22, 23, 28));

        let title_box = self.draw_title(ui, rect);

        let fuel_rect = Rect::from_min_size(
            title_box.max + vec2(-title_box.size().x, 16.0),
            vec2(rect.size().x - 32.0, 172.0),
        );
        self.draw_fuel_panel(ui, fuel_rect);

        let tires_rect = Rect::from_min_size(
            pos2(fuel_rect.min.x, fuel_rect.max.y + 16.0),
            vec2(fuel_rect.size().x / 2.0 - 8.0, 250.0),
        );
        self.draw_tires_panel(ui, tires_rect);

        let input_rect = Rect::from_min_size(
            pos2(
                fuel_rect.min.x + fuel_rect.size().x / 2.0 + 8.0,
                fuel_rect.max.y + 16.0,
            ),
            vec2(fuel_rect.size().x / 2.0 - 8.0, 250.0),
        );
        self.draw_input_panel(ui, input_rect);
    }

    fn draw_first_entry_dialog(&mut self, ui: &mut Ui) {
        Window::new("Hello")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.set_min_size(vec2(500.0, 0.0));
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(
                            "Welcome to the car info page, where you can see the current state of your car. Because of a game limitation, this only works for your own car - the app identifies it automatically based on the in-game name you've set in your settings. Beyond that, the page is fairly self-explanatory.",
                        )
                        .size(18.0),
                    );
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
                        self.first_entry = false;
                    }
                });
            });
    }

    fn draw_tires_panel(&self, ui: &mut Ui, tires_rect: Rect) {
        ui.painter().rect_filled(
            tires_rect,
            CornerRadius::same(16),
            Color32::from_white_alpha(17),
        );

        let usable_rect = Rect::from_min_max(
            tires_rect.min + vec2(16.0, 16.0),
            tires_rect.max - vec2(16.0, 16.0),
        );
        ui.put(usable_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui_phosphor_icons::icons::TIRE
                            .regular()
                            .size(14.0)
                            .color(Color32::from_white_alpha(64)),
                    );
                    ui.label(
                        RichText::new("TIRES")
                            .size(12.0)
                            .family(FontFamily::Name("BarlowCondensed".into()))
                            .color(Color32::from_white_alpha(64)),
                    );
                });
                ui.add_space(8.0);
                let avail =
                    usable_rect.size() - vec2(0.0, usable_rect.size().y - ui.available_height());
                let quad_size = vec2((avail.x - 8.0) / 2.0, (avail.y - 8.0) / 2.0);
                let first_rect = Rect::from_min_size(
                    pos2(usable_rect.min.x, usable_rect.max.y - ui.available_height()),
                    quad_size,
                );
                let spacing = 8.0;

                // If anybody who read this didn't have a stroke, please improve this code!
                self.draw_tire(ui, first_rect, "FL", &self.tires[0]);
                self.draw_tire(
                    ui,
                    Rect::from_min_size(
                        pos2(first_rect.max.x + spacing, first_rect.min.y),
                        quad_size,
                    ),
                    "FR",
                    &self.tires[1],
                );
                self.draw_tire(
                    ui,
                    Rect::from_min_size(
                        pos2(first_rect.min.x, first_rect.max.y + spacing),
                        quad_size,
                    ),
                    "RL",
                    &self.tires[2],
                );
                self.draw_tire(
                    ui,
                    Rect::from_min_size(
                        pos2(first_rect.max.x + spacing, first_rect.max.y + spacing),
                        quad_size,
                    ),
                    "RR",
                    &self.tires[3],
                );
            })
            .response
        });
    }

    fn draw_tire(&self, ui: &mut Ui, tire_rect: Rect, tire_name: &str, tire_info: &TireInfo) {
        ui.painter().rect_filled(
            tire_rect,
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
        );
        let usable_rect = Rect::from_min_max(
            tire_rect.min + vec2(4.0, 4.0),
            tire_rect.max - vec2(4.0, 4.0),
        );

        ui.put(usable_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(tire_name)
                            .color(Color32::WHITE)
                            .family(FontFamily::Name("BarlowCondensed".into()))
                            .size(14.0),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let (rect, _) = ui.allocate_exact_size(vec2(48.0, 16.0), Sense::empty());

                        ui.painter().rect_filled(
                            rect,
                            CornerRadius::same(2),
                            interpolate_color(
                                tire_info.health_percent,
                                0.0,
                                0.5,
                                1.0,
                                Color32::from_rgb(255, 0, 0),
                                Color32::from_rgb(127, 127, 0),
                                Color32::from_rgb(0, 255, 0),
                            )
                            .gamma_multiply(0.1),
                        );

                        ui.painter().text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            format!("{}%", (tire_info.health_percent * 1000.0).round() / 10.0),
                            FontId::new(14.0, FontFamily::Proportional),
                            interpolate_color(
                                tire_info.health_percent,
                                0.0,
                                0.5,
                                1.0,
                                Color32::from_rgb(255, 0, 0),
                                Color32::from_rgb(127, 127, 0),
                                Color32::from_rgb(0, 255, 0),
                            ),
                        );
                    });
                });
                ui.horizontal_centered(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(vec2(14.0, ui.available_height()), Sense::empty());
                    ui.painter().rect_filled(
                        rect,
                        CornerRadius::same(7),
                        Color32::from_white_alpha(25),
                    );
                    ui.painter().rect_filled(
                        Rect::from_min_max(
                            pos2(
                                rect.min.x,
                                rect.min.y
                                    + rect.size().y
                                        * ((200.0
                                            - (tire_info.inside_temp + tire_info.outside_temp))
                                            / 200.0)
                                            .clamp(0.0, 1.0),
                            ),
                            rect.max,
                        ),
                        CornerRadius::same(7),
                        interpolate_color(
                            (tire_info.inside_temp + tire_info.outside_temp) / 2.0,
                            40.0,
                            80.0,
                            100.0,
                            Color32::from_rgb(0, 0, 255),
                            Color32::from_rgb(0, 255, 0),
                            Color32::from_rgb(255, 0, 0),
                        ),
                    );

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Inside")
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .size(14.0)
                                    .color(Color32::from_white_alpha(191)),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{}°C",
                                        (tire_info.inside_temp * 100.0).round() / 100.0
                                    ))
                                    .size(14.0)
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .color(interpolate_color(
                                        tire_info.inside_temp,
                                        30.0,
                                        80.0,
                                        100.0,
                                        Color32::from_rgb(66, 135, 245),
                                        Color32::from_rgb(0, 255, 0),
                                        Color32::from_rgb(255, 0, 0),
                                    )),
                                );
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Outside")
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .size(14.0)
                                    .color(Color32::from_white_alpha(191)),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{}°C",
                                        (tire_info.outside_temp * 100.0).round() / 100.0
                                    ))
                                    .size(14.0)
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .color(interpolate_color(
                                        tire_info.outside_temp,
                                        30.0,
                                        80.0,
                                        100.0,
                                        Color32::from_rgb(66, 135, 245),
                                        Color32::from_rgb(0, 255, 0),
                                        Color32::from_rgb(255, 0, 0),
                                    )),
                                );
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Brake")
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .size(14.0)
                                    .color(Color32::from_white_alpha(191)),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new(format!(
                                        "{}°C",
                                        (tire_info.brake_temp * 100.0).round() / 100.0
                                    ))
                                    .size(14.0)
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .color(interpolate_color(
                                        tire_info.brake_temp,
                                        30.0,
                                        120.0,
                                        800.0,
                                        Color32::from_rgb(66, 135, 245),
                                        Color32::from_rgb(0, 255, 0),
                                        Color32::from_rgb(255, 0, 0),
                                    )),
                                );
                            });
                        });
                    });
                });
            })
            .response
        });
    }

    fn draw_input_panel(&self, ui: &mut Ui, input_rect: Rect) {
        ui.painter().rect_filled(
            input_rect,
            CornerRadius::same(16),
            Color32::from_white_alpha(17),
        );
        let usable_rect = Rect::from_min_max(
            input_rect.min + vec2(16.0, 16.0),
            input_rect.max - vec2(16.0, 16.0),
        );

        ui.put(usable_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui_phosphor_icons::icons::TIRE
                            .regular()
                            .size(14.0)
                            .color(Color32::from_white_alpha(64)),
                    );
                    ui.label(
                        RichText::new("INPUT")
                            .size(12.0)
                            .family(FontFamily::Name("BarlowCondensed".into()))
                            .color(Color32::from_white_alpha(64)),
                    );
                });

                ui.add_space(16.0);

                ui.label(
                    RichText::new("Throttle")
                        .size(16.0)
                        .family(FontFamily::Name("JetBrainsMono".into()))
                        .color(Color32::WHITE),
                );
                ui.allocate_ui(vec2(ui.available_width(), 12.0), |ui| {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 8.0, Color32::from_white_alpha(25));
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            ui.max_rect().min,
                            vec2(ui.max_rect().size().x * 0.6, ui.max_rect().size().y),
                        ),
                        8.0,
                        Color32::GREEN,
                    );
                });
                ui.add_space(40.0);

                ui.label(
                    RichText::new("Brake")
                        .size(16.0)
                        .family(FontFamily::Name("JetBrainsMono".into()))
                        .color(Color32::WHITE),
                );
                ui.allocate_ui(vec2(ui.available_width(), 12.0), |ui| {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 8.0, Color32::from_white_alpha(25));
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            ui.max_rect().min,
                            vec2(ui.max_rect().size().x * 0.3, ui.max_rect().size().y),
                        ),
                        8.0,
                        Color32::RED,
                    );
                });

                ui.add_space(40.0);

                ui.label(
                    RichText::new("Steering")
                        .size(16.0)
                        .family(FontFamily::Name("JetBrainsMono".into()))
                        .color(Color32::WHITE),
                );
                let input = 1.0;
                ui.allocate_ui(vec2(ui.available_width(), 12.0), |ui| {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 8.0, Color32::from_white_alpha(25));
                    ui.painter().rect_filled(
                        Rect::from_min_max(
                            pos2(
                                ui.max_rect().min.x
                                    + ui.max_rect().size().x
                                        * if input > 0.0 { 1.0 } else { 1.0 + input }
                                        / 2.0,
                                ui.max_rect().min.y,
                            ),
                            pos2(
                                ui.max_rect().min.x
                                    + ui.max_rect().size().x
                                        * if input < 0.0 { 1.0 } else { 1.0 + input }
                                        / 2.0,
                                ui.max_rect().max.y,
                            ),
                        ),
                        8.0,
                        Color32::CYAN,
                    );
                });
            })
            .response
        });
    }

    fn draw_fuel_panel(&self, ui: &mut Ui, fuel_rect: Rect) {
        ui.painter().rect_filled(
            fuel_rect,
            CornerRadius::same(16),
            Color32::from_white_alpha(17),
        );

        let usable_rect = Rect::from_min_max(
            fuel_rect.min + vec2(16.0, 16.0),
            fuel_rect.max - vec2(16.0, 16.0),
        );

        ui.put(usable_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui_phosphor_icons::icons::GAS_PUMP
                            .regular()
                            .size(14.0)
                            .color(Color32::from_white_alpha(64)),
                    );
                    ui.label(
                        RichText::new("FUEL & ENERGY")
                            .size(12.0)
                            .family(FontFamily::Name("BarlowCondensed".into()))
                            .color(Color32::from_white_alpha(64)),
                    );
                });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(40.0, 40.0), Sense::empty());
                    ui.painter().rect_filled(
                        rect,
                        CornerRadius::same(8),
                        Color32::from_rgba_unmultiplied(255, 132, 0, 127),
                    );
                    ui.put(
                        rect,
                        Label::new(
                            egui_phosphor_icons::icons::GAS_PUMP
                                .regular()
                                .color(Color32::from_rgb(254, 178, 0))
                                .size(24.0),
                        ),
                    );

                    ui.add_space(4.0);

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{}%",
                                    (self.fuel_info.fuel_percent * 10000.0).round() / 100.0
                                ))
                                .size(16.0)
                                .family(FontFamily::Name("JetBrainsMono".into()))
                                .color(Color32::WHITE),
                            );
                            ui.label(
                                RichText::new(format!(
                                    "{}L",
                                    (self.fuel_info.fuel_liters * 100.0).round() / 100.0
                                ))
                                .size(12.0)
                                .family(FontFamily::Name("JetBrainsMono".into()))
                                .color(Color32::from_white_alpha(127)),
                            );
                        });
                        ui.add_space(4.0);
                        let (bar_rect, _) = ui
                            .allocate_exact_size(vec2(ui.available_width(), 10.0), Sense::empty());
                        ui.painter().rect_filled(
                            bar_rect,
                            CornerRadius::same(5),
                            Color32::from_white_alpha(25),
                        );
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                bar_rect.min,
                                vec2(
                                    bar_rect.size().x * self.fuel_info.fuel_percent,
                                    bar_rect.size().y,
                                ),
                            ),
                            CornerRadius::same(5),
                            Color32::from_rgb(255, 127, 0),
                        )
                    })
                });

                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(40.0, 40.0), Sense::empty());
                    ui.painter().rect_filled(
                        rect,
                        CornerRadius::same(8),
                        Color32::from_rgba_unmultiplied(0, 144, 255, 127),
                    );
                    ui.put(
                        rect,
                        Label::new(
                            egui_phosphor_icons::icons::BATTERY_CHARGING
                                .regular()
                                .color(Color32::from_rgb(0, 198, 255))
                                .size(24.0),
                        ),
                    );

                    ui.add_space(4.0);

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "{}%",
                                (self.fuel_info.virt_eng_percent * 10000.0).round() / 100.0
                            ))
                            .size(16.0)
                            .family(FontFamily::Name("JetBrainsMono".into()))
                            .color(Color32::WHITE),
                        );
                        ui.add_space(4.0);
                        let (bar_rect, _) = ui
                            .allocate_exact_size(vec2(ui.available_width(), 10.0), Sense::empty());
                        ui.painter().rect_filled(
                            bar_rect,
                            CornerRadius::same(5),
                            Color32::from_white_alpha(25),
                        );
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                bar_rect.min,
                                vec2(
                                    bar_rect.size().x * self.fuel_info.virt_eng_percent,
                                    bar_rect.size().y,
                                ),
                            ),
                            CornerRadius::same(5),
                            Color32::from_rgb(0, 132, 255),
                        )
                    })
                });
            })
            .response
        });
    }

    fn draw_title(&self, ui: &mut Ui, rect: Rect) -> Rect {
        let str: String;
        let name_text_rect = ui.painter().text(
            rect.min + vec2(16.0, 16.0),
            Align2::LEFT_TOP,
            if !self.name.is_empty() {
                &self.name
            } else {
                str = format!(
                    "Driver {} couldn't be found in the lobby",
                    self.settings_provider.in_game_name.read().unwrap()
                );
                &str
            },
            FontId::new(32.0, FontFamily::Name("RacingSansOne".into())),
            Color32::WHITE,
        );
        let car_text_rect = ui.painter().text(
            pos2(rect.min.x + 16.0, name_text_rect.max.y),
            Align2::LEFT_TOP,
            if !self.name.is_empty() {
                &self.car
            } else {
                "You might want to change the \"in game name\" field in the settings"
            },
            FontId::new(16.0, FontFamily::Proportional),
            Color32::WHITE,
        );

        let badge_rect = Rect::from_min_size(
            pos2(car_text_rect.max.x + 16.0, car_text_rect.min.y),
            vec2(48.0, car_text_rect.size().y),
        );
        self.draw_badge(ui, badge_rect);

        Rect::from_min_max(name_text_rect.min, badge_rect.max)
    }

    fn draw_badge(&self, ui: &mut Ui, badge_rect: Rect) {
        let draw_badge = |badge_rect: Rect, color: Color32, name: &str| {
            ui.painter().rect(
                badge_rect,
                CornerRadius::same(4),
                color,
                Stroke::new(2.0, color.to_opaque()),
                StrokeKind::Inside,
            );
            ui.painter().text(
                badge_rect.center(),
                Align2::CENTER_CENTER,
                name,
                FontId::new(16.0, FontFamily::Name("RethinkSans".into())),
                color.to_opaque(),
            );
        };

        match self.car_class {
            IPVehicleClass::Gt3 => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(13, 157, 0, 64),
                "GT3",
            ),

            IPVehicleClass::Gte => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(255, 204, 0, 64),
                "GTE",
            ),
            IPVehicleClass::Lmp3 => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(123, 0, 255, 64),
                "LMP3",
            ),
            IPVehicleClass::Lmp2 | IPVehicleClass::Lmp2Elms => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(0, 127, 221, 64),
                "LMP2",
            ),
            IPVehicleClass::Hypercar => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(223, 39, 28, 64),
                "HY",
            ),
            _ => {}
        };
    }
    pub fn get_stale_driver_info(&self) -> Result<StaleDriverInfo, String> {
        let telemetry = *self
            .telemetry_provider
            .as_ref()
            .as_ref()
            .unwrap()
            .get_telemetry_object();

        let mut drivers: Vec<(String, usize)> = Vec::new();

        for (i, car) in telemetry.scoring.veh_scoring_info.iter().enumerate() {
            let name = i8_array32_to_string(&car.m_driver_name);
            if !name.is_empty() {
                drivers.push((name, i));
            }
        }
        let cur_driver_id;

        if let Some(driver) = drivers
            .iter()
            .find(|(name, _)| name == &*self.settings_provider.in_game_name.read().unwrap())
        {
            cur_driver_id = driver.1
        } else {
            return Err("Driver not found".to_owned());
        }

        let name =
            i8_array32_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_driver_name);
        let car =
            i8_array64_to_string(&telemetry.scoring.veh_scoring_info[cur_driver_id].m_vehicle_name);
        let car_class = telemetry.telemetry.telemetry_info[cur_driver_id].m_vehicle_class;

        Ok(StaleDriverInfo {
            name,
            car,
            car_class,
            index: cur_driver_id,
        })
    }
    pub fn get_dyn_driver_info(&self) -> DynDriverInfo {
        let telemetry = self
            .telemetry_provider
            .as_ref()
            .as_ref()
            .unwrap()
            .get_telemetry_object();

        let wheels = &telemetry.telemetry.telemetry_info[self.driver_index].m_wheel;
        let wheel0 = wheels[0];
        let wheel1 = wheels[1];
        let wheel2 = wheels[2];
        let wheel3 = wheels[3];

        let inner_temp0 = wheel0.m_tire_inner_layer_temperature;
        let outside_temp0 = wheel0.m_temperature;
        let inner_temp1 = wheel1.m_tire_inner_layer_temperature;
        let outside_temp1 = wheel1.m_temperature;
        let inner_temp2 = wheel2.m_tire_inner_layer_temperature;
        let outside_temp2 = wheel2.m_temperature;
        let inner_temp3 = wheel3.m_tire_inner_layer_temperature;
        let outside_temp3 = wheel3.m_temperature;

        let tires = [
            TireInfo {
                health_percent: wheel0.m_wear as f32,
                inside_temp: (inner_temp0.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                outside_temp: (outside_temp0.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                brake_temp: (wheel0.m_brake_temp - 273.15) as f32,
            },
            TireInfo {
                health_percent: wheel1.m_wear as f32,
                inside_temp: (inner_temp1.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                outside_temp: (outside_temp1.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                brake_temp: (wheel1.m_brake_temp - 273.15) as f32,
            },
            TireInfo {
                health_percent: wheel2.m_wear as f32,
                inside_temp: (inner_temp2.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                outside_temp: (outside_temp2.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                brake_temp: (wheel2.m_brake_temp - 273.15) as f32,
            },
            TireInfo {
                health_percent: wheel3.m_wear as f32,
                inside_temp: (inner_temp3.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                outside_temp: (outside_temp3.iter().sum::<f64>() / 3.0 - 273.15) as f32,
                brake_temp: (wheel3.m_brake_temp - 273.15) as f32,
            },
        ];
        let fuel = FuelInfo {
            fuel_percent: (telemetry.telemetry.telemetry_info[self.driver_index].m_fuel
                / telemetry.telemetry.telemetry_info[self.driver_index].m_fuel_capacity)
                as f32,
            fuel_liters: telemetry.telemetry.telemetry_info[self.driver_index].m_fuel as f32,
            virt_eng_percent: telemetry.telemetry.telemetry_info[self.driver_index]
                .m_virtual_energy,
        };

        DynDriverInfo { tires, fuel }
    }
}

fn interpolate_color(
    value: f32,
    min: f32,
    mid: f32,
    max: f32,
    start_color: Color32,
    mid_color: Color32,
    end_color: Color32,
) -> Color32 {
    let value = min.max(max.min(value));

    if value <= mid {
        let t = (value - min) / (mid - min);
        return interpolate_between(start_color, mid_color, t);
    }

    let t = (value - mid) / (max - mid);
    interpolate_between(mid_color, end_color, t)
}

fn interpolate_between(a: Color32, b: Color32, t: f32) -> Color32 {
    let r_val = (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t).round() as u8;
    let g_val = (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t).round() as u8;
    let b_val = (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t).round() as u8;

    Color32::from_rgb(r_val, g_val, b_val)
}
