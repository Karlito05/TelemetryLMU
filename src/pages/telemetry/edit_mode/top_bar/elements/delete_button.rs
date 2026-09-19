use crate::{
    components::button::button,
    pages::telemetry::telemetry_main::{EditLayoutInfo, EditModeContext, TelemetryPage},
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode::top_bar) fn draw_delete_button(
        &mut self,
        ui: &mut Ui,
    ) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgb(255, 0, 0),
            "Delete",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.show_delete_layout_dialog = true;
        }

        if self.show_delete_layout_dialog {
            if self.layouts.len() >= 2 {
                Window::new("Delete Layout?")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.label("Are you sure you want to delete this layout?");

                        ui.horizontal(|ui| {
                            if button(
                                ui,
                                vec2(64.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_rgb(255, 0, 0),
                                "Delete",
                                FontId::new(16.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                self.layouts.remove(self.edit_mode_context.layout.index);
                                self.cur_layout_index = 0;
                                self.show_delete_layout_dialog = false;
                                self.edit_mode_context = EditModeContext {
                                    layout: EditLayoutInfo {
                                        graphs: vec![],
                                        index: 0,
                                    },
                                    started_edtiting: false,
                                };
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
                                self.show_delete_layout_dialog = false;
                            }
                        });
                    });
            } else {
                Window::new("Couldn't Delete Layout")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.label("You must have at least 1 layout!");

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
                                self.show_delete_layout_dialog = false;
                            }
                        });
                    });
            }
        }
    }
}
