use crate::{
    components::dropdown::{DropdownItem, dropdown},
    pages::telemetry::telemetry_main::TelemetryPage,
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::normal_mode::top_bar) fn draw_layout_select(
        &mut self,
        ui: &mut Ui,
    ) {
        ui.add(
            Label::new(RichText::new("Layout:").size(16.0).color(Color32::WHITE)).selectable(false),
        );

        ui.add_space(2.0);

        dropdown(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            &mut self.cur_layout_index,
            "",
            FontId {
                size: 14.0,
                family: FontFamily::Proportional,
            },
            self.layouts
                .iter()
                .enumerate()
                .map(|(i, l)| DropdownItem {
                    value: i,
                    display_value: l.name.clone(),
                })
                .collect(),
        );
    }
}
