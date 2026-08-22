use eframe::egui::*;
use egui_material_icons::icons::ICON_ARROW_DROP_DOWN;

use crate::graph_view::GraphViewDataType;

pub struct DropdownItem<T: PartialEq> {
    pub value: T,
    pub display_value: String,
}

#[expect(clippy::too_many_arguments)]
pub fn dropdown<T: PartialEq>(
    ui: &mut Ui,
    size: Vec2,
    corner_radius: CornerRadius,
    fill_color: Color32,
    active: &mut T,
    placeholder: &str,
    font_id: FontId,
    items: Vec<DropdownItem<T>>,
) {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if response.hovered() {
        ui.painter().rect_filled(
            rect,
            corner_radius,
            fill_color.blend(Color32::from_white_alpha(25)),
        );
    } else {
        ui.painter().rect_filled(rect, corner_radius, fill_color);
    }

    let active_display_text = match items.iter().find(|di| di.value == *active) {
        Some(val) => &val.display_value,
        None => placeholder,
    };

    ui.put(rect, |ui: &mut Ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            // TODO: Fix overflowing text issue
            ui.add(
                Label::new(
                    RichText::new(active_display_text)
                        .font(font_id)
                        .color(Color32::WHITE),
                )
                .selectable(false),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(
                    Label::new(
                        RichText::new(ICON_ARROW_DROP_DOWN)
                            .size(24.0)
                            .color(Color32::WHITE),
                    )
                    .selectable(false),
                );
            });
        })
        .response
    });

    Popup::menu(&response).width(rect.width()).show(|ui| {
        for item in items {
            if ui
                .selectable_label(*active == item.value, item.display_value)
                .clicked()
            {
                *active = item.value;
                ui.close();
            }
        }
    });
}

pub struct GraphInfo {
    pub cur_lap: Vec<Vec2>,
    pub ref_lap: Vec<Vec2>,
    pub max_val: f64,
    pub color: Color32,
    pub n_gridlines: i32,
    pub unit: String,
    pub size_percent: f32,
    pub graph_type: GraphViewDataType,
}

pub fn graph(
    ui: &mut Ui,
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

    draw_gridlines(
        ui.painter(),
        rect,
        graph_info.n_gridlines,
        margins,
        graph_info.max_val,
        &graph_info.unit,
    );
    draw_lap(
        ui.painter(),
        rect,
        &graph_info.cur_lap,
        margins,
        Stroke::new(1.5, graph_info.color),
    );
    draw_lap(
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
    draw_title(
        ui.painter(),
        rect,
        &capitalize_first(&graph_info.graph_type.to_string()),
        margins,
        graph_info.color,
    );
}

fn draw_title(painter: &Painter, rect: Rect, text: &str, margins: f32, color: Color32) {
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

fn draw_lap(painter: &Painter, rect: Rect, lap: &Vec<Vec2>, margins: f32, stroke: Stroke) {
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

    painter.add(Shape::line(points, stroke));
}

fn draw_gridlines(
    painter: &Painter,
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
        dashed_line(
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

fn dashed_line(painter: &Painter, start: Pos2, end: Pos2, dash: f32, gap: f32, stroke: Stroke) {
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

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn button(
    ui: &mut Ui,
    size: Vec2,
    corner_radius: CornerRadius,
    fill_color: Color32,
    text: &str,
    font_id: FontId,
    text_color: Color32,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    ui.painter().rect_filled(rect, corner_radius, fill_color);

    if response.hovered() {
        ui.painter()
            .rect_filled(rect, corner_radius, Color32::from_white_alpha(25));
    }

    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        font_id,
        text_color,
    );

    response
}
