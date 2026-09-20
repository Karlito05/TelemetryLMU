use eframe::egui::*;

use crate::pages::telemetry::telemetry_main::TelemetryPage;

impl TelemetryPage {
    pub(in crate::pages::telemetry) fn draw_edit_mode(&mut self, ui: &mut Ui) {
        let top_bar_rect = Rect::from_min_max(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                16.0,
            ),
            ui.viewport_rect().max - vec2(16.0, 16.0),
        );

        self.draw_top_bar_edit(ui, top_bar_rect);

        let graphs_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                80.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        8.0
                    } else {
                        16.0
                    },
                ui.viewport_rect().size().y - (32.0 + 64.0),
            ),
        );
        self.draw_graphs_edit(ui, graphs_rect);
    }
}
