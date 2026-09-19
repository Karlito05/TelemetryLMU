use crate::{components::button::button, pages::settings::settings_main::SettingsPage};
use eframe::egui::*;

impl SettingsPage {
    pub(in crate::pages::settings) fn draw_lap_save_path_field(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Lap Save Path").size(20.0));
                ui.label(
                    RichText::new(&*self.settings_provider.record_save_path.read().unwrap())
                        .size(12.0),
                );
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if button(
                    ui,
                    vec2(140.0, 32.0),
                    CornerRadius::same(8),
                    Color32::from_white_alpha(25),
                    "Browse",
                    FontId::new(16.0, FontFamily::Proportional),
                    Color32::WHITE,
                )
                .clicked()
                {
                    let current_path = self
                        .settings_provider
                        .record_save_path
                        .read()
                        .unwrap()
                        .clone();

                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Select folder")
                        .set_directory(current_path)
                        .add_filter("All files", &["*"])
                        .set_can_create_directories(true)
                        .pick_folder()
                    {
                        *self.settings_provider.record_save_path.write().unwrap() =
                            path.display().to_string();
                    }
                }
            })
        });
    }
}
