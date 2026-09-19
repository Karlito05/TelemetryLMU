use crate::pages::car_info::car_info_main::CarInfo;
use eframe::egui::*;

impl CarInfo {
    pub(in crate::pages::car_info) fn draw_input_panel(&self, ui: &mut Ui, input_rect: Rect) {
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
                        egui_phosphor_icons::icons::GAME_CONTROLLER
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
                            vec2(
                                ui.max_rect().size().x * self.inputs.throttle,
                                ui.max_rect().size().y,
                            ),
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
                            vec2(
                                ui.max_rect().size().x * self.inputs.brake,
                                ui.max_rect().size().y,
                            ),
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
                let input = self.inputs.steering.clamp(-1.0, 1.0);
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
}
