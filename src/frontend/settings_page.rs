use eframe::egui::*;
use egui_phosphor_icons::icons;

use crate::frontend::{
    components::{input, switch},
    sidebar::Sidebar,
};
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub name: String,
    pub in_game_name: String,
    pub record_laps: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            in_game_name: "".to_owned(),
            record_laps: true,
        }
    }
}

impl Settings {
    pub fn draw_settings_page(&mut self, ui: &mut Ui, sidebar: &mut Sidebar) {
        Window::new("Settings")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .title_bar(false)
            .show(ui, |ui| {
                ui.set_min_size(vec2(500.0, 600.0));
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
                            sidebar.settings_open = false
                        }
                    });
                });

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
                            &mut self.name,
                            FontSelection::FontId(FontId::proportional(16.0)),
                            32,
                        )
                    })
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("In Game Name").size(20.0));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        input(
                            ui,
                            vec2(200.0, 32.0),
                            CornerRadius::same(8),
                            Stroke::new(1.0, Color32::from_gray(127)),
                            Color32::from_white_alpha(17),
                            &mut self.in_game_name,
                            FontSelection::FontId(FontId::proportional(16.0)),
                            32,
                        )
                    })
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Record Laps").size(20.0));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        self.record_laps = switch(
                            ui,
                            vec2(48.0, 24.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            Color32::from_rgb(19, 141, 241),
                            self.record_laps,
                        )
                    })
                });
            });
    }
}
