use crate::{
    components::dropdown::{DropdownItem, dropdown},
    pages::telemetry::telemetry_main::TelemetryPage,
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::normal_mode::top_bar) fn draw_driver_select(
        &mut self,
        ui: &mut Ui,
    ) {
        ui.add(
            Label::new(RichText::new("Driver:").size(16.0).color(Color32::WHITE)).selectable(false),
        );

        ui.add_space(2.0);

        dropdown(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            &mut self.cur_driver,
            "Select a driver",
            FontId {
                size: 14.0,
                family: FontFamily::Proportional,
            },
            self.telemetry_provider
                .as_ref()
                .as_ref()
                .unwrap()
                .get_drivers()
                .iter()
                .map(|driver| DropdownItem {
                    value: driver.clone(),
                    display_value: driver.0.clone(),
                })
                .collect(),
        );
    }
}
