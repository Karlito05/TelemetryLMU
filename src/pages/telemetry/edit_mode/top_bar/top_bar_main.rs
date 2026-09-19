use crate::pages::telemetry::telemetry_main::TelemetryPage;
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode) fn draw_top_bar_edit(
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

                self.draw_editing_layout_label(ui);

                ui.separator();

                self.draw_save_button(ui);

                ui.separator();

                self.draw_save_as_button(ui);

                ui.separator();

                self.draw_discard_button(ui);

                ui.separator();

                self.draw_add_graph_button(ui);

                ui.separator();

                self.draw_delete_button(ui);
            })
            .response
        });
    }
}
