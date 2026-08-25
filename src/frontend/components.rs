use eframe::{egui::*, epaint::Hsva};
use egui_material_icons::icons::ICON_ARROW_DROP_DOWN;
use egui_phosphor_icons::icons;

use crate::{graph_view::GraphViewDataType, telemetry_back::SharedMemoryObjectOut};

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
) -> bool {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let mut ret = false;

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
                ret = true;
            }
        }
    });
    ret
}

#[expect(clippy::too_many_arguments)]
pub fn number_input(
    ui: &mut Ui,
    size: Vec2,
    corner_radius: CornerRadius,
    min: i32,
    max: i32,
    step: i32,
    current: i32,
    color: Color32,
    font: FontId,
    text_color: Color32,
) -> i32 {
    let (rect, _) = ui.allocate_exact_size(size, Sense::empty());

    ui.painter().rect_filled(rect, corner_radius, color);

    ui.painter().text(
        pos2(4.0, size.y / 2.0) + rect.min.to_vec2(),
        Align2::LEFT_CENTER,
        current,
        font,
        text_color,
    );

    let top_button_rect = Rect::from_min_size(
        pos2(size.x - size.y / 2.0, 0.0) + rect.min.to_vec2(),
        vec2(size.y / 2.0, size.y / 2.0),
    );

    let top_button_response = ui.allocate_rect(top_button_rect, Sense::click());

    if top_button_response.hovered() {
        ui.painter().rect(
            top_button_rect,
            CornerRadius {
                nw: 0,
                ne: corner_radius.ne,
                sw: 0,
                se: 0,
            },
            Color32::from_white_alpha(25),
            Stroke::new(1.0, Color32::from_white_alpha(64)),
            StrokeKind::Inside,
        );
    } else {
        ui.painter().rect(
            top_button_rect,
            CornerRadius {
                nw: 0,
                ne: corner_radius.ne,
                sw: 0,
                se: 0,
            },
            Color32::from_white_alpha(0),
            Stroke::new(1.0, Color32::from_white_alpha(64)),
            StrokeKind::Inside,
        );
    }

    ui.put(
        top_button_rect,
        Label::new(icons::CARET_UP.light().size(size.y / 2.0)).selectable(false),
    );

    let bot_button_rect = Rect::from_min_size(
        pos2(size.x - size.y / 2.0, size.y / 2.0) + rect.min.to_vec2(),
        vec2(size.y, size.y) / 2.0,
    );

    let bot_button_response = ui.allocate_rect(bot_button_rect, Sense::click());

    if bot_button_response.hovered() {
        ui.painter().rect(
            bot_button_rect,
            CornerRadius {
                nw: 0,
                ne: 0,
                sw: 0,
                se: corner_radius.se,
            },
            Color32::from_white_alpha(25),
            Stroke::new(1.0, Color32::from_white_alpha(64)),
            StrokeKind::Inside,
        );
    } else {
        ui.painter().rect(
            bot_button_rect,
            CornerRadius {
                nw: 0,
                ne: 0,
                sw: 0,
                se: corner_radius.se,
            },
            Color32::from_white_alpha(0),
            Stroke::new(1.0, Color32::from_white_alpha(64)),
            StrokeKind::Inside,
        );
    }

    ui.put(
        bot_button_rect,
        Label::new(icons::CARET_DOWN.light().size(size.y / 2.0)).selectable(false),
    );

    if top_button_response.clicked() && current + step <= max {
        return current + step;
    }
    if bot_button_response.clicked() && current - step >= min {
        return current - step;
    }
    current
}

pub fn switch(
    ui: &mut Ui,
    size: Vec2,
    corner_radius: CornerRadius,
    bg_color: Color32,
    active_color: Color32,
    current: bool,
) -> bool {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    ui.painter().rect_filled(rect, corner_radius, bg_color);

    if current {
        ui.painter().rect_filled(
            Rect::from_min_size(
                pos2(rect.size().x / 2.0, 0.0) + rect.min.to_vec2(),
                vec2(rect.size().x / 2.0, rect.size().y),
            ),
            corner_radius,
            active_color,
        );
    } else {
        ui.painter().rect_filled(
            Rect::from_min_size(
                pos2(0.0, 0.0) + rect.min.to_vec2(),
                vec2(rect.size().x / 2.0, rect.size().y),
            ),
            corner_radius,
            Color32::from_white_alpha(127),
        );
    }

    if response.clicked() {
        return !current;
    }

    current
}

pub fn color_picker(
    ui: &mut Ui,
    size: Vec2,
    corner_radius: CornerRadius,
    current: Color32,
    bg_color: Color32,
    font: FontId,
    text_color: Color32,
) -> Color32 {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let mut new = Hsva::from(current);

    ui.painter().rect_filled(rect, corner_radius, bg_color);
    ui.painter().rect_filled(
        Rect::from_min_size(rect.min, vec2(rect.size().y, rect.size().y)),
        corner_radius,
        new,
    );

    let popup_id = ui.auto_id_with("popup");
    const COLOR_SLIDER_WIDTH: f32 = 275.0;

    Popup::menu(&response)
        .id(popup_id)
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.spacing_mut().slider_width = COLOR_SLIDER_WIDTH;
            color_picker::color_picker_hsva_2d(ui, &mut new, color_picker::Alpha::Opaque);
        });

    ui.painter().text(
        pos2(
            (rect.size().x - rect.size().y) / 2.0 + rect.size().y,
            rect.size().y / 2.0,
        ) + rect.min.to_vec2(),
        Align2::CENTER_CENTER,
        Color32::from(new)
            .to_hex()
            .to_string()
            .chars()
            .rev()
            .skip(2)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>(), // This whole ordeal just removes the last 2 chars from the
        // string. AKA the FF for alpha which is not needed
        font,
        text_color,
    );

    Color32::from(new)
}

pub enum GraphChange {
    Height(usize, f32), // Height delta in percent (0-1)
    Type(usize, String),
    Color(usize, Color32),
    Gridlines(usize, i32),
    Reference(usize, bool),
    Delete(usize),
}

pub fn graph_edit(
    ui: &mut Ui,
    index: usize,
    graph: &GraphInfo,
    size: Vec2,
    resizable: bool,
    corner_radius: CornerRadius,
) -> Option<GraphChange> {
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());

    if resizable {
        let handle = Rect::from_min_max(
            pos2(rect.left(), rect.bottom() - 4.0),
            pos2(rect.right(), rect.bottom() + 4.0),
        );

        let response = ui.interact(handle, ui.id().with(index), Sense::drag());

        if response.dragged() {
            let new_height = graph.size_percent * (size.y + response.drag_delta().y) / size.y;

            return Some(GraphChange::Height(index, new_height - graph.size_percent));
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
        }
    }

    ui.painter()
        .rect_filled(rect, corner_radius, Color32::from_rgb(22, 23, 28));

    let old_graph_type = graph.graph_type.to_string();
    let mut new_graph_type = graph.graph_type.to_string();
    let mut change = None;

    ui.place(rect, |ui: &mut Ui| {
        if rect.size().y > 200.0 {
            ui.vertical_centered_justified(|ui: &mut Ui| {
                Grid::new(index)
                    .num_columns(2)
                    .spacing([8.0, 8.0])
                    .show(ui, |ui| {
                        ui.add(
                            Label::new(
                                RichText::new("Graph Type:")
                                    .size(16.0)
                                    .color(Color32::WHITE),
                            )
                            .selectable(false),
                        );
                        dropdown(
                            ui,
                            vec2(140.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            &mut new_graph_type,
                            "Select a type",
                            FontId::new(14.0, FontFamily::Proportional),
                            GraphViewDataType::get_all_string()
                                .iter()
                                .map(|s| DropdownItem {
                                    value: s.clone(),
                                    display_value: s.clone(),
                                })
                                .collect(),
                        );
                        ui.end_row();

                        ui.add(
                            Label::new(
                                RichText::new("Gridlines:").size(16.0).color(Color32::WHITE),
                            )
                            .selectable(false),
                        );
                        let new = number_input(
                            ui,
                            vec2(140.0, 32.0),
                            CornerRadius::same(8),
                            3,
                            10,
                            1,
                            graph.n_gridlines,
                            Color32::from_white_alpha(25),
                            FontId::new(14.0, FontFamily::Proportional),
                            Color32::WHITE,
                        );

                        if new != graph.n_gridlines {
                            change = Some(GraphChange::Gridlines(index, new));
                        }
                        ui.end_row();

                        ui.add(
                            Label::new(
                                RichText::new("Show Reference:")
                                    .size(16.0)
                                    .color(Color32::WHITE),
                            )
                            .selectable(false),
                        );
                        let new = switch(
                            ui,
                            vec2(48.0, 24.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            Color32::from_rgb(19, 141, 241),
                            graph.show_ref,
                        );

                        if new != graph.show_ref {
                            change = Some(GraphChange::Reference(index, new));
                        }
                        ui.end_row();
                        ui.add(
                            Label::new(RichText::new("Color:").size(16.0).color(Color32::WHITE))
                                .selectable(false),
                        );
                        let new = color_picker(
                            ui,
                            vec2(110.0, 32.0),
                            CornerRadius::same(8),
                            graph.color,
                            Color32::from_white_alpha(25),
                            FontId::new(14.0, FontFamily::Proportional),
                            Color32::WHITE,
                        );
                        if new != graph.color {
                            change = Some(GraphChange::Color(index, new));
                        }
                        ui.end_row();

                        if button(
                            ui,
                            vec2(96.0, 32.0),
                            CornerRadius::same(8),
                            Color32::RED,
                            "Delete",
                            FontId {
                                size: 12.0,
                                family: FontFamily::Proportional,
                            },
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            change = Some(GraphChange::Delete(index));
                        }
                        ui.end_row();
                    });
            })
            .response
        } else {
            ui.horizontal_centered(|ui: &mut Ui| {
                ui.add(
                    Label::new(
                        RichText::new("Graph Type:")
                            .size(16.0)
                            .color(Color32::WHITE),
                    )
                    .selectable(false),
                );
                dropdown(
                    ui,
                    vec2(140.0, 32.0),
                    CornerRadius::same(8),
                    Color32::from_white_alpha(25),
                    &mut new_graph_type,
                    "Select a type",
                    FontId::new(14.0, FontFamily::Proportional),
                    GraphViewDataType::get_all_string()
                        .iter()
                        .map(|s| DropdownItem {
                            value: s.clone(),
                            display_value: s.clone(),
                        })
                        .collect(),
                );
                ui.separator();

                ui.add(
                    Label::new(RichText::new("Gridlines:").size(16.0).color(Color32::WHITE))
                        .selectable(false),
                );
                let new = number_input(
                    ui,
                    vec2(140.0, 32.0),
                    CornerRadius::same(8),
                    3,
                    10,
                    1,
                    graph.n_gridlines,
                    Color32::from_white_alpha(25),
                    FontId::new(14.0, FontFamily::Proportional),
                    Color32::WHITE,
                );

                if new != graph.n_gridlines {
                    change = Some(GraphChange::Gridlines(index, new));
                }
                ui.separator();

                ui.add(
                    Label::new(
                        RichText::new("Show Reference:")
                            .size(16.0)
                            .color(Color32::WHITE),
                    )
                    .selectable(false),
                );
                let new = switch(
                    ui,
                    vec2(48.0, 24.0),
                    CornerRadius::same(8),
                    Color32::from_white_alpha(25),
                    Color32::from_rgb(19, 141, 241),
                    graph.show_ref,
                );

                if new != graph.show_ref {
                    change = Some(GraphChange::Reference(index, new));
                }
                ui.separator();

                ui.add(
                    Label::new(RichText::new("Color:").size(16.0).color(Color32::WHITE))
                        .selectable(false),
                );
                let new = color_picker(
                    ui,
                    vec2(110.0, 32.0),
                    CornerRadius::same(8),
                    graph.color,
                    Color32::from_white_alpha(25),
                    FontId::new(14.0, FontFamily::Proportional),
                    Color32::WHITE,
                );
                if new != graph.color {
                    change = Some(GraphChange::Color(index, new));
                }
                ui.separator();

                if button(
                    ui,
                    vec2(96.0, 32.0),
                    CornerRadius::same(8),
                    Color32::RED,
                    "Delete",
                    FontId {
                        size: 12.0,
                        family: FontFamily::Proportional,
                    },
                    Color32::WHITE,
                )
                .clicked()
                {
                    change = Some(GraphChange::Delete(index));
                }
            })
            .response
        }
    });
    if new_graph_type != old_graph_type {
        change = Some(GraphChange::Type(index, new_graph_type));
    }

    change
}

#[derive(Clone, Debug)]
pub struct GraphInfo {
    pub cur_lap: Vec<Vec2>,
    pub ref_lap: Vec<Vec2>,
    pub color: Color32,
    pub show_ref: bool,
    pub n_gridlines: i32,
    pub size_percent: f32,
    pub graph_type: GraphViewDataType,
}

pub fn graph(
    ui: &mut Ui,
    graph_info: &GraphInfo,
    size: Vec2,
    corner_radius: CornerRadius,
    margins: f32, // Total so val/2 on each side
    telemetry: &SharedMemoryObjectOut,
) {
    // Allocate the rect
    let rect = ui.allocate_space(size).1;

    // Background
    ui.painter()
        .rect_filled(rect, corner_radius, Color32::from_rgb(22, 23, 28));

    let labels = graph_info
        .graph_type
        .get_unit_labels(telemetry, graph_info.n_gridlines);

    draw_gridlines(ui.painter(), rect, graph_info.n_gridlines, margins, &labels);
    draw_lap(
        ui.painter(),
        rect,
        &graph_info.cur_lap,
        margins,
        Stroke::new(1.5, graph_info.color),
    );
    if graph_info.show_ref {
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
    }
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
