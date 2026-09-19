use crate::{components::input::input, pages::settings::settings_main::SettingsPage};
use eframe::egui::*;

impl SettingsPage {
    pub(in crate::pages::settings) fn draw_name_field(&mut self, ui: &mut Ui) {
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Name").size(20.0));
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
    }
}
