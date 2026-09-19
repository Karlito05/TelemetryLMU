use eframe::{egui::*, epaint::Hsva};

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
