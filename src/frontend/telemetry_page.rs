use crate::telemetry_back::Telemetry;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontFamily, FontId, Pos2, Rect, Stroke, Vec2, pos2, vec2,
};

pub struct TelemetryPage {
    telemetry: Telemetry,
}

impl Default for TelemetryPage {
    fn default() -> Self {
        Self {
            telemetry: Telemetry::new("LMU_Data"),
        }
    }
}

impl TelemetryPage {
    pub fn main(&mut self, ui: &mut egui::Ui) {
        self.graph(
            ui,
            &vec![vec2(0.0, 0.0), vec2(1.0, 1.0)],
            &vec![vec2(1.0, 0.0), vec2(0.0, 1.0)],
            Color32::from_rgb(0, 0, 255),
            3,
            1.0,
            "%".into(),
            vec2(1000.0, 300.0),
            CornerRadius {
                nw: 24,
                ne: 24,
                sw: 0,
                se: 0,
            },
            100.0,
        );
    }

    #[expect(clippy::too_many_arguments)]
    fn graph(
        &mut self,
        ui: &mut egui::Ui,
        cur_lap: &Vec<Vec2>,
        ref_lap: &Vec<Vec2>,
        color: Color32,
        n_gridlines: i32,
        max_val: f64,
        unit: String,
        size: Vec2,
        corner_radius: CornerRadius,
        margins: f32, // Total so val/2 on each side
    ) {
        // Allocate the rect
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::empty());

        // Background
        ui.painter()
            .rect_filled(rect, corner_radius, Color32::from_rgb(22, 23, 28));

        self.draw_gridlines(ui.painter(), rect, n_gridlines, margins, max_val, unit);
        self.draw_lap(
            ui.painter(),
            rect,
            cur_lap,
            margins,
            Stroke::new(1.5, color),
        );
        self.draw_lap(
            ui.painter(),
            rect,
            ref_lap,
            margins,
            Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 127),
            ),
        );
    }

    fn draw_lap(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        lap: &Vec<Vec2>,
        margins: f32,
        stroke: Stroke,
    ) {
        let mut size = rect.size();
        size.y -= margins;
        let pos = rect.min;
        let mut points: Vec<Pos2> = vec![];

        for point in lap {
            points.push(pos2(
                point.x * size.x + pos.x,
                (1.0 - point.y) * size.y + pos.y + margins / 2.0,
            ));
        }

        painter.add(egui::Shape::line(points, stroke));
    }

    fn draw_gridlines(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        n_gridlines: i32,
        margins: f32,
        max_val: f64,
        unit: String,
    ) {
        let mut size = rect.size();
        size.y -= margins;
        let pos = rect.min;
        let spacing = size.y / (n_gridlines as f32 - 1.0);
        let width = size.x;

        for i in 0..n_gridlines {
            let y = (i) as f32 * spacing;
            self.dashed_line(
                painter,
                pos2(pos.x, y + pos.y + margins / 2.0),
                pos2(width + pos.x, y + pos.y + margins / 2.0),
                8.0,
                8.0,
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 127)),
            );

            // Overrides for special cases
            if max_val == 1.0 {
                painter.text(
                    pos2(pos.x + 2.0, y + pos.y + margins / 2.0 - 5.0),
                    Align2::LEFT_BOTTOM,
                    format!(
                        "{} {}",
                        max_val * 100.0 * (n_gridlines - 1 - i) as f64 / (n_gridlines - 1) as f64,
                        unit
                    )
                    .to_string(),
                    FontId::new(12.0, FontFamily::default()),
                    Color32::from_white_alpha(127),
                );
            } else {
                painter.text(
                    pos2(pos.x + 2.0, y + pos.y + margins / 2.0 - 5.0),
                    Align2::LEFT_BOTTOM,
                    format!(
                        "{} {}",
                        max_val * (n_gridlines - 1 - i) as f64 / (n_gridlines - 1) as f64,
                        unit
                    )
                    .to_string(),
                    FontId::new(12.0, FontFamily::default()),
                    Color32::from_white_alpha(127),
                );
            }
        }
    }

    fn dashed_line(
        &self,
        painter: &egui::Painter,
        start: egui::Pos2,
        end: egui::Pos2,
        dash: f32,
        gap: f32,
        stroke: egui::Stroke,
    ) {
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
}
