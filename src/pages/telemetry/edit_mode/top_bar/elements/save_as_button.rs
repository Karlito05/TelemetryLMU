use crate::{
    components::button::button,
    pages::telemetry::telemetry_main::{
        EditLayoutInfo, EditModeContext, LayoutInfo, TelemetryPage,
    },
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode::top_bar) fn draw_save_as_button(
        &mut self,
        ui: &mut Ui,
    ) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgb(19, 141, 241),
            "Save as",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.save_as_dialog_info.show = true
        }
        if self.save_as_dialog_info.show {
            Window::new("Name?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter the name for your new layout: ");

                    ui.text_edit_singleline(&mut self.save_as_dialog_info.name);

                    ui.horizontal(|ui: &mut Ui| {
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_rgb(19, 141, 241),
                            "Confirm",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.layouts.push(LayoutInfo {
                                graphs: self.edit_mode_context.layout.graphs.clone(),
                                name: self.save_as_dialog_info.name.clone(),
                            });
                            self.cur_layout_index = self.layouts.len() - 1;
                            self.edit_mode_context = EditModeContext {
                                layout: EditLayoutInfo {
                                    graphs: vec![],
                                    index: 0,
                                },
                                started_edtiting: false,
                            };
                            self.save_as_dialog_info.show = false;
                            self.in_layout_edit_mode = false;
                        }
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Cancel",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.save_as_dialog_info.show = false;
                        }
                    })
                });
        }
    }
}
