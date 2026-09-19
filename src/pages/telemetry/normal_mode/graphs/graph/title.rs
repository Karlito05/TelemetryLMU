use eframe::egui::*;

pub(super) fn draw_title(painter: &Painter, rect: Rect, text: &str, margins: f32, color: Color32) {
    painter.text(
        pos2(
            margins / 4.0 + rect.min.x,
            rect.height() - margins / 4.0 + rect.min.y,
        ),
        Align2::LEFT_CENTER,
        text,
        FontId {
            size: 16.0,
            family: FontFamily::Name("RacingSansOne".into()),
        },
        color,
    );
}
