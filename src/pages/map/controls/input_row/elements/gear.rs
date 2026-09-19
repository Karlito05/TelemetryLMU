use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::input_row) fn draw_gear(&self, ui: &mut Ui, gear: i32) {
        let rect = ui.allocate_exact_size(vec2(50.0, 55.0), Sense::empty()).0;
        ui.put(rect, |ui: &mut Ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Gear")
                        .size(12.0)
                        .color(Color32::from_white_alpha(64)),
                );
                ui.label(
                    RichText::new(format!("{}", gear))
                        .size(24.0)
                        .color(Color32::WHITE)
                        .family(FontFamily::Name("JetBrainsMono".into())),
                );
            })
            .response
        });
    }
}
