use crate::pages::car_info::car_info_main::CarInfo;
use eframe::egui::*;

impl CarInfo {
    pub(in crate::pages::car_info) fn draw_fuel_panel(&self, ui: &mut Ui, fuel_rect: Rect) {
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
}
