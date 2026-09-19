use crate::{
    components::button::button,
    pages::telemetry::telemetry_main::{GraphInfo, TelemetryPage},
    providers::telemetry_provider::telemetry_value_type::TelemetryValueType,
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode::top_bar) fn draw_add_graph_button(
        &mut self,
        ui: &mut Ui,
    ) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            "Add",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            if self.edit_mode_context.layout.graphs.len() < 10 {
                self.edit_mode_context.layout.graphs.push(GraphInfo {
                    color: Color32::WHITE,
                    show_ref: true,
                    n_gridlines: 3,
                    size_percent: 0.0,
                    ref_val_type: TelemetryValueType::Rpm,
                });

                let new_num_graphs = self.edit_mode_context.layout.graphs.len();

                self.edit_mode_context
                    .layout
                    .graphs
                    .iter_mut()
                    .for_each(|g| {
                        g.size_percent = 1.0 / new_num_graphs as f32;
                    });
            } else {
                self.show_add_limit_dialog = true
            }
        }
        if self.show_add_limit_dialog {
            Window::new("Couldn't Add Graph")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("You can't have more than 10 graphs!");

                    ui.horizontal(|ui| {
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Ok",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.show_add_limit_dialog = false;
                        }
                    });
                });
        }
    }
}
