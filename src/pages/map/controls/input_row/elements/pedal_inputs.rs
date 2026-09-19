use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::input_row) fn draw_pedal_inputs(
        &self,
        ui: &mut Ui,
        throttle: f32,
        brake: f32,
    ) {
        let (bars_rect, _) = ui.allocate_exact_size(vec2(200.0, 40.0), Sense::empty());

        let throttle_rect = Rect::from_min_size(bars_rect.min, vec2(200.0, 16.0));
        let brake_rect = Rect::from_min_max(bars_rect.max - vec2(200.0, 16.0), bars_rect.max);

        ui.painter()
            .rect_filled(throttle_rect, 8, Color32::from_white_alpha(25));
        ui.painter().rect_filled(
            Rect::from_min_size(
                throttle_rect.min,
                throttle_rect.size() * vec2(throttle, 1.0),
            ),
            8,
            Color32::from_rgb(0, 255, 0),
        );

        ui.painter()
            .rect_filled(brake_rect, 8, Color32::from_white_alpha(25));
        ui.painter().rect_filled(
            Rect::from_min_size(brake_rect.min, brake_rect.size() * vec2(brake, 1.0)),
            8,
            Color32::from_rgb(255, 0, 0),
        );
    }
}
