use crate::graph_view::GraphViewDataType;
use crate::telemetry_back::Telemetry;
use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontFamily, FontId, Pos2, Rect, Stroke, Vec2, pos2, vec2,
};

pub struct TelemetryPage {
    telemetry: Telemetry,
    graphs: Vec<GraphInfo>,
}

struct GraphInfo {
    cur_lap: Vec<Vec2>,
    ref_lap: Vec<Vec2>,
    max_val: f64,
    color: Color32,
    n_gridlines: i32,
    unit: String,
    size_percent: f32,
    graph_type: GraphViewDataType,
}

impl Default for TelemetryPage {
    fn default() -> Self {
        Self {
            telemetry: Telemetry::new("LMU_Data"),
            graphs: vec![
                GraphInfo {
                    cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                    ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                    max_val: 1.0,
                    color: Color32::from_rgb(20, 10, 200),
                    n_gridlines: 3,
                    unit: "%".to_owned(),
                    size_percent: 0.5,
                    graph_type: GraphViewDataType::from_string("rpm", 0),
                },
                GraphInfo {
                    cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                    ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                    max_val: 1.0,
                    color: Color32::from_rgb(20, 200, 20),
                    n_gridlines: 3,
                    unit: "%".to_owned(),
                    size_percent: 0.5,
                    graph_type: GraphViewDataType::from_string("speed", 0),
                },
            ],
        }
    }
}

impl TelemetryPage {
    pub fn draw_telemetry_page(&mut self, ui: &mut egui::Ui) {
        let top_bar_rect =
            egui::Rect::from_min_size(pos2(308.0, 16.0), vec2(ui.available_width() - 8.0, 48.0));

        ui.painter().rect_filled(
            top_bar_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        let graphs_rect = egui::Rect::from_min_size(
            pos2(308.0, 80.0),
            vec2(ui.available_width() - 8.0, ui.available_height() - 80.0),
        );

        ui.put(graphs_rect, |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                let margins = 64.0;
                for (i, graph_info) in self.graphs.iter().enumerate() {
                    if i == 0 {
                        self.graph(
                            ui,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius {
                                nw: 24,
                                ne: 24,
                                sw: 0,
                                se: 0,
                            },
                            margins,
                        );
                        continue;
                    }

                    if i == self.graphs.len() - 1 {
                        self.graph(
                            ui,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius {
                                nw: 0,
                                ne: 0,
                                sw: 24,
                                se: 24,
                            },
                            margins,
                        );
                        continue;
                    }

                    if self.graphs.len() == 1 {
                        self.graph(
                            ui,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius::same(24),
                            margins,
                        );
                        continue;
                    }

                    self.graph(
                        ui,
                        graph_info,
                        vec2(
                            graphs_rect.width(),
                            graph_info.size_percent * graphs_rect.height(),
                        ),
                        CornerRadius::same(0),
                        margins,
                    );
                }
            })
            .response
        });
    }

    fn graph(
        &self,
        ui: &mut egui::Ui,
        graph_info: &GraphInfo,
        size: Vec2,
        corner_radius: CornerRadius,
        margins: f32, // Total so val/2 on each side
    ) {
        // Allocate the rect
        let rect = ui.allocate_space(size).1;

        // Background
        ui.painter()
            .rect_filled(rect, corner_radius, Color32::from_rgb(22, 23, 28));

        self.draw_gridlines(
            ui.painter(),
            rect,
            graph_info.n_gridlines,
            margins,
            graph_info.max_val,
            &graph_info.unit,
        );
        self.draw_lap(
            ui.painter(),
            rect,
            &graph_info.cur_lap,
            margins,
            Stroke::new(1.5, graph_info.color),
        );
        self.draw_lap(
            ui.painter(),
            rect,
            &graph_info.ref_lap,
            margins,
            Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(
                    graph_info.color.r(),
                    graph_info.color.g(),
                    graph_info.color.b(),
                    127,
                ),
            ),
        );
        self.draw_title(
            ui.painter(),
            rect,
            &capitalize_first(&graph_info.graph_type.to_string()),
            margins,
            graph_info.color,
        );
    }

    fn draw_title(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        text: &str,
        margins: f32,
        color: Color32,
    ) {
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
        unit: &String,
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

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
