use eframe::egui::*;

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
