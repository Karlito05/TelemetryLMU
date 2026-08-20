use eframe::egui::*;
use egui_material_icons::icons::ICON_ARROW_DROP_DOWN;

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
) {
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

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
            }
        }
    });
}
