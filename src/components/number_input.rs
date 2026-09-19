use eframe::egui::*;
use egui_phosphor_icons::icons;

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
