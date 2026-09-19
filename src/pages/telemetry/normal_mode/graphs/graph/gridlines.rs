use eframe::egui::*;

use crate::utils::dashed_line::dashed_line;

pub(super) fn draw_gridlines(
    painter: &Painter,
    rect: Rect,
    n_gridlines: i32,
    margins: f32,
    labels: &[String],
) {
    let mut size = rect.size();
    size.y -= margins;
    let pos = rect.min;
    let spacing = size.y / (n_gridlines as f32 - 1.0);
    let width = size.x;

    for i in 0..n_gridlines {
        let y = (i) as f32 * spacing;
        dashed_line(
            painter,
            pos2(pos.x, y + pos.y + margins / 2.0),
            pos2(width + pos.x, y + pos.y + margins / 2.0),
            8.0,
            8.0,
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 127)),
        );

        painter.text(
            pos2(pos.x + 2.0, y + pos.y + margins / 2.0 - 5.0),
            Align2::LEFT_BOTTOM,
            labels[i as usize].clone(),
            FontId::new(12.0, FontFamily::default()),
            Color32::from_white_alpha(127),
        );
    }
}
