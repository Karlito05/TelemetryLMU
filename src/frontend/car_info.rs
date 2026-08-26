use eframe::egui::*;

use crate::{
    frontend::{settings::Settings, sidebar::Sidebar},
    telemetry_back::IPVehicleClass,
};

pub struct CarInfo {
    name: String,
    car: String,
    car_class: IPVehicleClass,
    fuel_info: FuelInfo,
    tires: [TireInfo; 4],
}

struct FuelInfo {
    fuel_percent: f32,
    virt_eng_percent: f32,
    fuel_liters: f32,
}

#[derive(Default, Debug)]
struct TireInfo {
    inside_temp: f32,
    outside_temp: f32,
    brake_temp: f32,
    health_percent: f32,
}

impl Default for CarInfo {
    fn default() -> Self {
        Self {
            name: "TestAcc".to_string(),
            car: "Test car".to_string(),
            car_class: IPVehicleClass::Gt3,
            fuel_info: FuelInfo {
                fuel_percent: 0.64,
                virt_eng_percent: 0.43,
                fuel_liters: 55.0,
            },
            tires: [
                TireInfo::default(),
                TireInfo::default(),
                TireInfo::default(),
                TireInfo::default(),
            ],
        }
    }
}

impl CarInfo {
    pub fn draw_car_info_page(
        &mut self,
        ui: &mut Ui,
        sidebar: &mut Sidebar,
        settings: &mut Settings,
    ) {
        let rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
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
                        let (rect, _) = ui.allocate_exact_size(vec2(26.0, 16.0), Sense::empty());

                        ui.painter().rect_filled(
                            rect,
                            CornerRadius::same(2),
                            Color32::from_rgba_unmultiplied(0, 255, 0, 25),
                        ); // TODO: Lerp based on health

                        ui.painter().text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            format!("{}%", tire_info.health_percent * 100.0),
                            FontId::new(14.0, FontFamily::Proportional),
                            Color32::from_rgba_unmultiplied(0, 255, 0, 255),
                        ); // TODO: Lerp based on health
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
                        Color32::from_rgba_unmultiplied(0, 255, 0, 255),
                    );
                    // TODO: LERP the colors based on temps :)
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
                                    RichText::new(format!("{}°C", tire_info.inside_temp.round()))
                                        .size(14.0)
                                        .color(Color32::from_rgb(0, 255, 0))
                                        .family(FontFamily::Name("JetBrainsMono".into())),
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
                                    RichText::new(format!("{}°C", tire_info.outside_temp.round()))
                                        .size(14.0)
                                        .color(Color32::from_rgb(0, 255, 0))
                                        .family(FontFamily::Name("JetBrainsMono".into())),
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
                                    RichText::new(format!("{}°C", tire_info.brake_temp.round()))
                                        .size(14.0)
                                        .color(Color32::from_rgb(0, 255, 0))
                                        .family(FontFamily::Name("JetBrainsMono".into())),
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
                                RichText::new(format!("{}%", self.fuel_info.fuel_percent * 100.0))
                                    .size(16.0)
                                    .family(FontFamily::Name("JetBrainsMono".into()))
                                    .color(Color32::WHITE),
                            );
                            ui.label(
                                RichText::new(format!("{}L", self.fuel_info.fuel_liters))
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
                            RichText::new(format!("{}%", self.fuel_info.virt_eng_percent * 100.0))
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
        let name_text_rect = ui.painter().text(
            rect.min + vec2(16.0, 16.0),
            Align2::LEFT_TOP,
            &self.name,
            FontId::new(32.0, FontFamily::Name("RacingSansOne".into())),
            Color32::WHITE,
        );
        let car_text_rect = ui.painter().text(
            pos2(rect.min.x + 16.0, name_text_rect.max.y),
            Align2::LEFT_TOP,
            &self.car,
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
}
