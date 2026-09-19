use crate::pages::telemetry::telemetry_main::TelemetryPage;
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::normal_mode) fn draw_top_bar_normal(
        &mut self,
        ui: &mut Ui,
        top_bar_rect: Rect,
    ) {
        ui.painter().rect_filled(
            top_bar_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        ui.put(top_bar_rect, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);

                self.draw_sidebar_button(ui);

                ui.separator();

                self.draw_driver_select(ui);

                ui.separator();

                self.draw_layout_select(ui);

                ui.separator();

                self.draw_reference_controls(ui);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(4.0);

                    self.draw_edit_layout_button(ui);
                })
            })
            .response
        });
    }
}
