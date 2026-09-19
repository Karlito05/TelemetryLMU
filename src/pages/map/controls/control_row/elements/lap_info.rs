use crate::pages::map::{
    map_main::{CarInfo, MapPage},
    utils::badge::draw_badge,
};
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::control_row) fn draw_lap_info(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        info: CarInfo,
        rtl: bool,
    ) {
        ui.put(rect, |ui: &mut Ui| {
            ui.with_layout(
                if !rtl {
                    Layout::top_down(Align::Min)
                } else {
                    Layout::top_down(Align::Max)
                },
                |ui| {
                    ui.with_layout(
                        if !rtl {
                            Layout::left_to_right(Align::Min)
                        } else {
                            Layout::right_to_left(Align::Min)
                        },
                        |ui| {
                            ui.label(
                                RichText::new(info.driver.clone())
                                    .size(16.0)
                                    .color(Color32::WHITE),
                            );
                            let badge_rect =
                                ui.allocate_exact_size(vec2(40.0, 16.0), Sense::empty()).0;
                            draw_badge(info.class, ui, badge_rect);
                        },
                    );
                    ui.label(format!(
                        "{}:{:.3}",
                        (info.laptime / 60.0).trunc(),
                        info.laptime % 60.0
                    ));
                },
            )
            .response
        });
    }
}
