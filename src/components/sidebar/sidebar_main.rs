use eframe::egui::*;
use std::sync::Arc;

use crate::providers::{settings_provider::SettingsProvider, state_provider::StateProvider};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Sidebar {
    #[serde(skip)]
    pub(super) state_provider: Arc<StateProvider>,
    #[serde(skip)]
    pub(super) settings_provider: Arc<SettingsProvider>,
}

impl Sidebar {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
    ) -> Self {
        Self {
            state_provider,
            settings_provider,
        }
    }
}

impl Sidebar {
    pub fn draw_sidebar(&mut self, ui: &mut Ui) {
        // let screen_width = ui.ctx().viewport_rect().width();
        let sidebar_width = 300.0;
        let sidebar_height = ui.ctx().viewport_rect().height();

        if !*self.state_provider.sidebar_open.read().unwrap() {
            return;
        }

        Panel::left(Id::new("e_sidebar"))
            .resizable(false)
            .exact_size(sidebar_width)
            .show_separator_line(false)
            .show(ui, |ui| {
                let sidebar_back_rect = Rect::from_min_size(
                    pos2(16.0, 16.0),
                    vec2(sidebar_width - 32.0, sidebar_height - 32.0),
                );
                ui.painter().rect_filled(
                    sidebar_back_rect,
                    CornerRadius::same(24),
                    Color32::from_rgb(23, 23, 28),
                );

                ui.put(sidebar_back_rect, |ui: &mut Ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(16.0);

                        self.draw_top(ui);

                        ui.add_space(32.0);

                        self.draw_buttons(ui);

                        ui.add_space(ui.available_height() - 70.0);

                        if self.draw_profile(ui).clicked() {
                            *self.state_provider.settings_open.write().unwrap() = true;
                        }
                    })
                    .response
                })
            });
    }
}
