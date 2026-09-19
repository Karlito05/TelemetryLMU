use crate::interface::IPVehicleClass;
use eframe::egui::*;

pub fn draw_badge(class: IPVehicleClass, ui: &mut Ui, badge_rect: Rect) {
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
            FontId::new(14.0, FontFamily::Name("RethinkSans".into())),
            color.to_opaque(),
        );
    };

    match class {
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
