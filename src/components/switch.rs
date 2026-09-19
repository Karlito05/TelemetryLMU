use eframe::egui::*;

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
