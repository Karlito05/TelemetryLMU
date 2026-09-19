use crate::{components::button::button, pages::settings::settings_main::SettingsPage};
use eframe::egui::*;

impl SettingsPage {
    pub(in crate::pages::settings) fn draw_change_pfp_field(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Change Profile Picture").size(20.0));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                #[expect(clippy::collapsible_if)]
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
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title("Select a file")
                        .add_filter("Images", &["png", "jpg", "jpeg"])
                        .pick_file()
                    {
                        if let Ok(bytes) = std::fs::read(&path) {
                            self.load_pfp(ui.ctx(), &bytes);
                        }
                    }
                }
            });
        });
    }
}
