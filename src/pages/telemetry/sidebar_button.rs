use crate::pages::telemetry::telemetry_main::TelemetryPage;
use eframe::egui::*;
use egui_phosphor_icons::icons;

impl TelemetryPage {
    pub(super) fn draw_sidebar_button(&self, ui: &mut Ui) {
        let (sidebar_icon_rect, response) = ui.allocate_exact_size(
            vec2(ui.available_height() - 8.0, ui.available_height() - 8.0),
            Sense::click(),
        );
        if response.hovered() {
            ui.painter().rect_filled(
                sidebar_icon_rect,
                CornerRadius::same(40),
                Color32::from_white_alpha(25),
            );
        }
        if response.clicked() {
            ui.painter().rect_filled(
                sidebar_icon_rect,
                CornerRadius::same(40),
                Color32::from_white_alpha(25),
            );
            self.state_provider.sidebar_open.write().unwrap().toggle();
        }

        ui.put(
            sidebar_icon_rect,
            Label::new(
                icons::SIDEBAR_SIMPLE
                    .regular()
                    .size(32.0)
                    .color(Color32::from_rgb(19, 141, 241)),
            )
            .selectable(false),
        );
    }
}
