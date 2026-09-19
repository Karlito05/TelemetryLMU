use std::fs;

use crate::{
    components::button::button,
    pages::telemetry::telemetry_main::TelemetryPage,
    providers::telemetry_provider::telemetry_provider_main::{Lap, SaveData},
};
use eframe::egui::*;

impl TelemetryPage {
    pub(in crate::pages::telemetry::normal_mode::top_bar) fn draw_reference_controls(
        &mut self,
        ui: &mut Ui,
    ) {
        ui.add(
            Label::new(RichText::new("Reference:").size(16.0).color(Color32::WHITE))
                .selectable(false),
        );

        #[expect(clippy::collapsible_if)]
        if button(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            "Select ref from file",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("Select a reference file")
                .set_directory(
                    self.settings_provider
                        .record_save_path
                        .read()
                        .unwrap()
                        .clone(),
                )
                .add_filter("JSON files", &["json"])
                .pick_file()
            {
                let contents = fs::read_to_string(path).unwrap_or_default();
                let save_data: SaveData = serde_json::from_str(&contents).unwrap_or_default();

                self.ref_lap_override = Some(Lap {
                    datapoints: save_data.lap_data,
                    distances: save_data.distances,
                    positions: save_data.positions,
                    times: save_data.times,
                    laptime: Some(save_data.lap_time),
                });
            }
        }
        if button(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgba_unmultiplied(255, 0, 0, 127),
            "Clear",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.ref_lap_override = None;
        }
    }
}
