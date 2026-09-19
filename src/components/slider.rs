use eframe::egui::*;

pub fn slider(
    ui: &mut Ui,
    size: Vec2,
    color: Color32,
    value: &mut f32,
    min: f32,
    max: f32,
    slider_width: f32,
) {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click_and_drag());
    let slider_rect = Rect::from_min_max(
        pos2(
            rect.min.x,
            rect.min.y + rect.height() / 2.0 - slider_width / 2.0,
        ),
        pos2(
            rect.max.x,
            rect.max.y - rect.height() / 2.0 + slider_width / 2.0,
        ),
    );
    ui.painter().rect_filled(
        slider_rect,
        slider_rect.height() / 2.0,
        Color32::from_white_alpha(25),
    );
    ui.painter().rect_filled(
        Rect::from_min_size(
            slider_rect.min,
            vec2(
                slider_rect.size().x * ((*value - min) / (max - min)),
                slider_rect.size().y,
            ),
        ),
        slider_rect.height() / 2.0,
        color,
    );

    ui.painter().circle_filled(
        pos2(
            slider_rect.min.x + slider_rect.size().x * ((*value - min) / (max - min)),
            slider_rect.min.y + slider_rect.size().y / 2.0,
        ),
        rect.height() / 2.0,
        color,
    );

    if let Some(pos) = response.interact_pointer_pos() {
        let rel_pos = pos - rect.min;

        let normalised_x_pos = rel_pos.x / rect.size().x;

        *value = (normalised_x_pos * (max - min) + min).clamp(min, max)
    }
}
