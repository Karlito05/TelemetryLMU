use crate::pages::telemetry::telemetry_main::{EditLayoutInfo, EditModeContext, TelemetryPage};
use eframe::egui::*;
use egui_phosphor_icons::icons;

impl TelemetryPage {
    pub(in crate::pages::telemetry::normal_mode::top_bar) fn draw_edit_layout_button(
        &mut self,
        ui: &mut Ui,
    ) {
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
            self.in_layout_edit_mode = true;
            self.edit_mode_context = EditModeContext {
                layout: EditLayoutInfo {
                    graphs: self.layouts[self.cur_layout_index].graphs.clone(),
                    index: self.cur_layout_index,
                },
                started_edtiting: false,
            };
        }

        ui.put(
            sidebar_icon_rect,
            Label::new(
                icons::PENCIL
                    .regular()
                    .size(32.0)
                    .color(Color32::from_rgb(19, 141, 241)),
            )
            .selectable(false),
        );
    }
}
