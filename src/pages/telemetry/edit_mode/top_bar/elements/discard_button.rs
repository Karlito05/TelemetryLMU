use crate::{
    components::button::button,
    pages::telemetry::telemetry_main::{EditLayoutInfo, EditModeContext, TelemetryPage},
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode::top_bar) fn draw_discard_button(
        &mut self,
        ui: &mut Ui,
    ) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            "Discard",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.edit_mode_context = EditModeContext {
                layout: EditLayoutInfo {
                    graphs: vec![],
                    index: 0,
                },
                started_edtiting: false,
            };
            self.in_layout_edit_mode = false;
        }
    }
}
