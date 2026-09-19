use crate::{
    components::{
        button::button,
        color_picker::color_picker,
        dropdown::{DropdownItem, dropdown},
        number_input::number_input,
        switch::switch,
    },
    pages::telemetry::{edit_mode::graphs::graphs_main::GraphChange, telemetry_main::GraphInfo},
    telemetry::TelemetryGraphValueType,
};
use eframe::egui::*;

// TODO: Refactor this

pub(in crate::pages::telemetry::edit_mode::graphs) fn graph_edit(
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

    let old_graph_type = graph.ref_val_type.to_string();
    let mut new_graph_type = graph.ref_val_type.to_string();
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
                            TelemetryGraphValueType::get_all_string()
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
                    TelemetryGraphValueType::get_all_string()
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
