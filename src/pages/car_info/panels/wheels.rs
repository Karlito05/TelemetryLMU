use crate::{
    pages::car_info::car_info_main::{CarInfo, TireInfo},
    utils::interpolate_color::interpolate_color,
};
use eframe::egui::*;

impl CarInfo {
    pub(in crate::pages::car_info) fn draw_tires_panel(&self, ui: &mut Ui, tires_rect: Rect) {
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
}
