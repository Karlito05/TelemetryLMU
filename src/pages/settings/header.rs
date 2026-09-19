use crate::pages::settings::settings_main::SettingsPage;
use eframe::egui::*;
use egui_phosphor_icons::icons;

impl SettingsPage {
    pub(super) fn draw_header(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Settings")
                    .size(32.0)
                    .color(Color32::WHITE)
                    .family(FontFamily::Name("RacingSansOne".into())),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(
                        Button::new(icons::X.regular().size(32.0))
                            .fill(Color32::TRANSPARENT)
                            .stroke(Stroke::NONE),
                    )
                    .clicked()
                {
                    *self.state_provider.settings_open.write().unwrap() = false
                }
            });
        });
    }
}
