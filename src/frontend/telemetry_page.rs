use crate::frontend::components::DropdownItem;
use crate::frontend::sidebar::Sidebar;
use crate::telemetry_back::Telemetry;
use crate::{frontend::components::dropdown, graph_view::GraphViewDataType};
use eframe::egui::{self, *};
use egui_material_icons::icons::{ICON_MENU, ICON_VIEW_SIDEBAR};

pub struct TelemetryPage {
    telemetry: Telemetry,
    cur_driver: (String, i32),
    cur_layout_index: usize,
    layouts: Vec<LayoutInfo>,
}

struct LayoutInfo {
    name: String,
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
            telemetry: Telemetry::new("/dev/shm/LMU_Data"),
            cur_driver: ("".to_owned(), 0),
            cur_layout_index: 0,
            layouts: vec![
                LayoutInfo {
                    name: "Main test".to_owned(),
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
                },
                LayoutInfo {
                    name: "Main test 2".to_owned(),
                    graphs: vec![
                        GraphInfo {
                            cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                            ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                            max_val: 1.0,
                            color: Color32::from_rgb(20, 150, 200),
                            n_gridlines: 3,
                            unit: "%".to_owned(),
                            size_percent: 0.5,
                            graph_type: GraphViewDataType::from_string("rpm", 0),
                        },
                        GraphInfo {
                            cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                            ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                            max_val: 1.0,
                            color: Color32::from_rgb(200, 0, 20),
                            n_gridlines: 3,
                            unit: "%".to_owned(),
                            size_percent: 0.5,
                            graph_type: GraphViewDataType::from_string("speed", 0),
                        },
                    ],
                },
            ],
        }
    }
}

impl TelemetryPage {
    pub fn draw_telemetry_page(&mut self, ui: &mut egui::Ui, sidebar: &mut Sidebar) {
        let top_bar_rect = egui::Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 0.0 } else { 16.0 },
                48.0,
            ),
        );

        ui.painter().rect_filled(
            top_bar_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        ui.put(top_bar_rect, |ui: &mut egui::Ui| {
            ui.horizontal(|ui| {
                let (sidebar_icon_rect, response) = ui.allocate_exact_size(
                    vec2(ui.available_height() - 8.0, ui.available_height() - 8.0),
                    Sense::click(),
                );
                if response.hovered() {
                    ui.painter().rect_filled(
                        sidebar_icon_rect,
                        CornerRadius::same(8),
                        Color32::from_white_alpha(25),
                    );
                }
                if response.clicked() {
                    ui.painter().rect_filled(
                        sidebar_icon_rect,
                        CornerRadius::same(8),
                        Color32::from_white_alpha(25),
                    );
                    sidebar.open = !sidebar.open;
                }
                // TODO: Add an icon to this mess right here (sidebar)
                // TODO: Refactor this into functions ig

                ui.add(
                    egui::Label::new(
                        egui::RichText::new("Driver:")
                            .size(16.0)
                            .color(Color32::WHITE),
                    )
                    .selectable(false),
                );

                ui.add_space(2.0);

                dropdown(
                    ui,
                    vec2(140.0, 32.0),
                    CornerRadius::same(8),
                    Color32::from_white_alpha(25),
                    &mut self.cur_driver,
                    "Select a driver",
                    FontId {
                        size: 14.0,
                        family: FontFamily::Proportional,
                    },
                    self.telemetry
                        .get_drivers()
                        .iter()
                        .map(|driver| DropdownItem {
                            value: driver.clone(),
                            display_value: driver.0.clone(),
                        })
                        .collect(),
                );

                ui.separator();

                ui.add(
                    egui::Label::new(
                        egui::RichText::new("Layout:")
                            .size(16.0)
                            .color(Color32::WHITE),
                    )
                    .selectable(false),
                );

                ui.add_space(2.0);

                dropdown(
                    ui,
                    vec2(140.0, 32.0),
                    CornerRadius::same(8),
                    Color32::from_white_alpha(25),
                    &mut self.cur_layout_index,
                    "",
                    FontId {
                        size: 14.0,
                        family: FontFamily::Proportional,
                    },
                    self.layouts
                        .iter()
                        .enumerate()
                        .map(|(i, l)| DropdownItem {
                            value: i,
                            display_value: l.name.clone(),
                        })
                        .collect(),
                );
            })
            .response
        });

        let graphs_rect = egui::Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 80.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
                ui.available_height() - 24.0,
            ),
        );

        ui.put(graphs_rect, |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                let margins = 64.0;
                for (i, graph_info) in self.layouts[self.cur_layout_index]
                    .graphs
                    .iter()
                    .enumerate()
                {
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

                    if i == self.layouts[self.cur_layout_index].graphs.len() - 1 {
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

                    if self.layouts[self.cur_layout_index].graphs.len() == 1 {
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
