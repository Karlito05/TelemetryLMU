use crate::pages::telemetry::telemetry_main::TelemetryPage;
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry) fn draw_normal_mode(&mut self, ui: &mut Ui) {
        let top_bar_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                16.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        0.0
                    } else {
                        16.0
                    },
                48.0,
            ),
        );

        self.draw_top_bar_normal(ui, top_bar_rect);

        let graphs_rect = Rect::from_min_max(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                80.0,
            ),
            ui.viewport_rect().max - vec2(16.0, 16.0),
        );

        self.draw_graphs_normal(ui, graphs_rect);
    }
}
