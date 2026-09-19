use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::input_row) fn draw_speed(&self, ui: &mut Ui, speed: f32) {
        let rect = ui.allocate_exact_size(vec2(100.0, 55.0), Sense::empty()).0;
        ui.put(rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Speed")
                        .size(12.0)
                        .color(Color32::from_white_alpha(64)),
                );
                ui.label(
                    RichText::new(format!("{}km/h", speed))
                        .size(24.0)
                        .color(Color32::WHITE)
                        .family(FontFamily::Name("JetBrainsMono".into())),
                );
            })
            .response
        });
    }
}
