use crate::{components::switch::switch, pages::settings::settings_main::SettingsPage};
use eframe::egui::*;

impl SettingsPage {
    pub(in crate::pages::settings) fn draw_record_laps_field(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Record Laps").size(20.0));

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
    }
}
