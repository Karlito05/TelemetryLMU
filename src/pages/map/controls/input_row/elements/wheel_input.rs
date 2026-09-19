use std::f32::consts::PI;

use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::input_row) fn draw_wheel_input(
        &self,
        ui: &mut Ui,
        steering: f32,
    ) {
        ui.add(
            Image::new(include_image!(
                "../../../../../../public/icons/steering-wheel-blue.svg"
            ))
            .rotate((steering - 0.5) * 360.0 * (PI / 180.0), Vec2::splat(0.5)),
        );
    }
}
