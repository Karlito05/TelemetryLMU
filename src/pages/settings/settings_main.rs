use eframe::egui::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::providers::{settings_provider::SettingsProvider, state_provider::StateProvider};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPage {
    #[serde(skip)]
    pub(super) settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    pub(super) state_provider: Arc<StateProvider>,
}

impl SettingsPage {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
    ) -> Self {
        Self {
            settings_provider,
            state_provider,
        }
    }

    pub fn draw_settings_page(&mut self, ui: &mut Ui) {
        Window::new("Settings")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .title_bar(false)
            .show(ui, |ui| {
                ui.set_min_size(vec2(500.0, 600.0));

                self.draw_header(ui);

                self.draw_name_field(ui);
                self.draw_in_game_name_field(ui);
                self.draw_record_laps_field(ui);
                self.draw_record_all_cars_field(ui);
                self.draw_lap_save_path_field(ui);
                self.draw_change_pfp_field(ui);
            });
    }
}
