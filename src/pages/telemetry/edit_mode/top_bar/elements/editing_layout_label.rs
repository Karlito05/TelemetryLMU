use crate::pages::telemetry::telemetry_main::TelemetryPage;
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode::top_bar) fn draw_editing_layout_label(
        &mut self,
        ui: &mut Ui,
    ) {
        ui.add(
            Label::new(
                RichText::new(format!(
                    "Editing: {}",
                    self.layouts[self.edit_mode_context.layout.index].name
                ))
                .size(16.0)
                .color(Color32::WHITE),
            )
            .selectable(false),
        );
    }
}
