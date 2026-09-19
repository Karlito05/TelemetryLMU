use eframe::egui::*;

pub fn dashed_line(painter: &Painter, start: Pos2, end: Pos2, dash: f32, gap: f32, stroke: Stroke) {
    let dir = (end - start).normalized();
    let len = start.distance(end);

    let mut dist = 0.0;

    while dist < len {
        let a = start + dir * dist;
        let b = start + dir * (dist + dash).min(len);

        painter.line_segment([a, b], stroke);

        dist += dash + gap;
    }
}
