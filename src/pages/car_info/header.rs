use eframe::egui::*;

use crate::{
    pages::car_info::car_info_main::CarInfo,
    providers::telemetry_provider::interface::IPVehicleClass,
};

impl CarInfo {
    pub(super) fn draw_title(&self, ui: &mut Ui, rect: Rect) -> Rect {
        let str: String;
        let name_text_rect = ui.painter().text(
            rect.min + vec2(16.0, 16.0),
            Align2::LEFT_TOP,
            if !self.name.is_empty() {
                &self.name
            } else {
                str = format!(
                    "Driver {} couldn't be found in the lobby",
                    self.settings_provider.in_game_name.read().unwrap()
                );
                &str
            },
            FontId::new(32.0, FontFamily::Name("RacingSansOne".into())),
            Color32::WHITE,
        );
        let car_text_rect = ui.painter().text(
            pos2(rect.min.x + 16.0, name_text_rect.max.y),
            Align2::LEFT_TOP,
            if !self.name.is_empty() {
                &self.car
            } else {
                "You might want to change the \"in game name\" field in the settings"
            },
            FontId::new(16.0, FontFamily::Proportional),
            Color32::WHITE,
        );

        let badge_rect = Rect::from_min_size(
            pos2(car_text_rect.max.x + 16.0, car_text_rect.min.y),
            vec2(48.0, car_text_rect.size().y),
        );
        self.draw_badge(ui, badge_rect);

        Rect::from_min_max(name_text_rect.min, badge_rect.max)
    }

    fn draw_badge(&self, ui: &mut Ui, badge_rect: Rect) {
        let draw_badge = |badge_rect: Rect, color: Color32, name: &str| {
            ui.painter().rect(
                badge_rect,
                CornerRadius::same(4),
                color,
                Stroke::new(2.0, color.to_opaque()),
                StrokeKind::Inside,
            );
            ui.painter().text(
                badge_rect.center(),
                Align2::CENTER_CENTER,
                name,
                FontId::new(16.0, FontFamily::Name("RethinkSans".into())),
                color.to_opaque(),
            );
        };

        match self.car_class {
            IPVehicleClass::Gt3 => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(13, 157, 0, 64),
                "GT3",
            ),

            IPVehicleClass::Gte => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(255, 204, 0, 64),
                "GTE",
            ),
            IPVehicleClass::Lmp3 => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(123, 0, 255, 64),
                "LMP3",
            ),
            IPVehicleClass::Lmp2 | IPVehicleClass::Lmp2Elms => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(0, 127, 221, 64),
                "LMP2",
            ),
            IPVehicleClass::Hypercar => draw_badge(
                badge_rect,
                Color32::from_rgba_unmultiplied(223, 39, 28, 64),
                "HY",
            ),
            _ => {}
        };
    }
}
