use eframe::egui::*;

#[expect(clippy::too_many_arguments)]
pub fn input(
    ui: &mut Ui,
    size: Vec2,
    corner_radius: CornerRadius,
    stroke: Stroke,
    bg_color: Color32,
    text: &mut String,
    font: FontSelection,
    max: usize,
) -> Response {
    let (rect, _) = ui.allocate_exact_size(size, Sense::empty());

    ui.painter()
        .rect(rect, corner_radius, bg_color, stroke, StrokeKind::Inside);

    let usable_rect = Rect::from_min_size(rect.min + vec2(4.0, 4.0), rect.size() - vec2(8.0, 8.0));

    let response = ui.put(
        usable_rect,
        TextEdit::singleline(text) // ← pass it straight through
            .font(font)
            .frame(Frame::NONE)
            .min_size(usable_rect.size())
            .horizontal_align(Align::LEFT)
            .vertical_align(Align::Center)
            .margin(Margin::same(2)),
    );

    // I mean not good but what you gonna do about it
    if text.len() > max {
        text.pop();
    }

    response
}
