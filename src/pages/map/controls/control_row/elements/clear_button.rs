use crate::{
    components::button::button,
    pages::map::map_main::{CarInfo, Dp, MapPage},
};
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::control_row) fn draw_clear_button(
        ui: &mut Ui,
        car_info: &mut Option<CarInfo>,
        car: &mut Vec<Dp>,
        cur_dp: &mut Option<usize>,
    ) {
        if button(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::RED,
            "Clear",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            *car_info = None;
            car.clear();
            *cur_dp = None;
        }
    }
}
