use crate::components::sidebar::sidebar_main::Sidebar;
use eframe::egui::*;
use egui_phosphor_icons::icons;

impl Sidebar {
    pub fn draw_profile(&self, ui: &mut Ui) -> Response {
        let desired_size = vec2(ui.available_width() - 16.0, 64.0);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // Background
        let bg_color = if response.hovered() {
            Color32::from_rgb(45, 46, 50)
        } else {
            Color32::from_rgb(22, 23, 28)
        };

        ui.painter()
            .rect_filled(rect, CornerRadius::same(16), bg_color);

        // Profile image
        let image_rect = Rect::from_min_size(rect.min + vec2(8.0, 8.0), vec2(48.0, 48.0));

        if let Some(img) = &*self.settings_provider.pfp_texture.read().unwrap() {
            ui.put(
                image_rect,
                Image::new(img)
                    .fit_to_exact_size(vec2(48.0, 48.0))
                    .corner_radius(CornerRadius::same(8)),
            );
        } else {
            ui.put(
                image_rect,
                Image::new(include_image!("../../../public/icons/pfp_default.svg"))
                    .fit_to_exact_size(vec2(48.0, 48.0))
                    .corner_radius(CornerRadius::same(8)),
            );
        }

        // Name
        ui.painter().text(
            pos2(rect.min.x + 68.0, rect.center().y),
            Align2::LEFT_CENTER,
            self.settings_provider.name.read().unwrap().clone(),
            FontId::proportional(20.0),
            Color32::from_white_alpha(200),
        );

        // More icon
        ui.put(
            Rect::from_min_size(
                pos2(rect.max.x - 40.0, rect.center().y - 16.0),
                vec2(24.0, 32.0),
            ),
            Label::new(
                icons::GEAR_SIX
                    .light()
                    .size(32.0)
                    .color(Color32::from_rgb(19, 141, 241)),
            )
            .selectable(false),
        );

        response
    }
}
