use eframe::egui::*;
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub name: String,
    pub in_game_name: String,
}

impl Settings {
    pub fn draw_settings_page(&mut self, ui: &mut Ui) {
        Window::new("Settings")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .title_bar(false)
            .show(ui, |ui| {
                ui.set_min_size(vec2(500.0, 600.0));
                ui.label(
                    RichText::new("Settings")
                        .size(32.0)
                        .color(Color32::WHITE)
                        .family(FontFamily::Name("RacingSansOne".into())),
                );
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Name").size(24.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {})
                });
            });
    }
}
