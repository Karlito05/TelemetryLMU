use eframe::egui::*;

use crate::{
    interface::SharedMemoryObjectOut,
    pages::telemetry::telemetry_main::{GraphInfo, Lap},
    telemetry::TelemetryGraphValueType,
};

#[expect(clippy::too_many_arguments)]
pub(super) fn draw_lap(
    painter: &Painter,
    rect: Rect,
    lap: &Lap,
    graph_info: &GraphInfo,
    telemetry: &SharedMemoryObjectOut,
    margins: f32,
    stroke: Stroke,
    car_num: usize,
) {
    let mut size = rect.size();
    size.y -= margins;
    let pos = rect.min;
    let mut points: Vec<Pos2> = vec![];

    for i in 0..lap.values.len().min(lap.distances.len()) {
        points.push(pos2(
            TelemetryGraphValueType::normalize_distance_into_lap(telemetry, lap.distances[i] as f64)
                as f32
                * size.x
                + pos.x,
            (1.0 - graph_info
                .ref_val_type
                .normalize(lap.values[i] as f64, telemetry, car_num) as f32)
                * size.y
                + pos.y
                + margins / 2.0,
        ));
    }

    painter.add(Shape::line(points, stroke));
}
