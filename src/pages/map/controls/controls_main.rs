use crate::{components::slider::slider, pages::map::map_main::MapPage};
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map) fn draw_controls(&mut self, ui: &mut Ui, rect: Rect) {
        ui.painter()
            .rect_filled(rect, 16, Color32::from_white_alpha(17));

        let usable_rect =
            Rect::from_min_max(rect.min + vec2(16.0, 16.0), rect.max - vec2(16.0, 16.0));

        ui.put(usable_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                slider(
                    ui,
                    vec2(ui.available_width(), 24.0),
                    Color32::from_rgb(19, 141, 241),
                    &mut self.time,
                    0.0,
                    1.0,
                    8.0,
                );

                self.draw_control_row(ui);
                self.draw_input_row(ui);
            })
            .response
        });
    }
}
