use eframe::egui::*;

use crate::frontend::components::input;
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
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        input(
                            ui,
                            vec2(200.0, 32.0),
                            CornerRadius::same(8),
                            Stroke::new(2.0, Color32::from_gray(127)),
                            Color32::from_white_alpha(25),
                            &mut self.name,
                            FontSelection::FontId(FontId::proportional(16.0)),
                            32,
                        )
                    })
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("In Game Name").size(24.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        input(
                            ui,
                            vec2(200.0, 32.0),
                            CornerRadius::same(8),
                            Stroke::new(2.0, Color32::from_gray(127)),
                            Color32::from_white_alpha(25),
                            &mut self.in_game_name,
                            FontSelection::FontId(FontId::proportional(16.0)),
                            32,
                        )
                    })
                });
            });
    }
}
