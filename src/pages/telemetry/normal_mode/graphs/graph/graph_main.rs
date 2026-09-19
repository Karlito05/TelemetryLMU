use eframe::egui::*;

use crate::{
    interface::SharedMemoryObjectOut,
    pages::telemetry::{
        normal_mode::graphs::graph::{gridlines::draw_gridlines, lap::draw_lap, title::draw_title},
        telemetry_main::{DynGraphData, GraphInfo},
    },
    utils::capitalize_first::capitalize_first,
};

#[expect(clippy::too_many_arguments)]
pub(in crate::pages::telemetry::normal_mode::graphs) fn graph(
    ui: &mut Ui,
    graph_info: &GraphInfo,
    dyn_graph_data: &DynGraphData,
    car_num: usize,
    size: Vec2,
    corner_radius: CornerRadius,
    margins: f32, // Total so val/2 on each side
    telemetry: &SharedMemoryObjectOut,
) {
    // Allocate the rect
    let painter = ui.allocate_painter(size, Sense::empty()).1;

    // Background
    painter.rect_filled(
        painter.clip_rect(),
        corner_radius,
        Color32::from_rgb(22, 23, 28),
    );

    // TODO: Cache this doesn't need to be refetched every render
    let labels =
        graph_info
            .ref_val_type
            .get_unit_labels(telemetry, graph_info.n_gridlines, car_num);

    draw_gridlines(
        &painter,
        painter.clip_rect(),
        graph_info.n_gridlines,
        margins,
        &labels,
    );

    draw_lap(
        &painter,
        painter.clip_rect(),
        &dyn_graph_data.cur_lap,
        graph_info,
        telemetry,
        margins,
        Stroke::new(1.5, graph_info.color),
        car_num,
    );

    if graph_info.show_ref {
        draw_lap(
            &painter,
            painter.clip_rect(),
            &dyn_graph_data.ref_lap,
            graph_info,
            telemetry,
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
            car_num,
        );
    }
    draw_title(
        &painter,
        painter.clip_rect(),
        &capitalize_first(&graph_info.ref_val_type.to_string()),
        margins,
        graph_info.color,
    );
}
