use crate::components::sidebar::sidebar_main::Sidebar;
use eframe::egui::*;

impl Sidebar {
    pub fn draw_top(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 16.0;

            ui.add_space(16.0);

            ui.add(
                Image::new(include_image!("../../../public/icons/Logo.svg"))
                    .fit_to_exact_size(vec2(48.0, 48.0)),
            );
            ui.add(
                Label::new(
                    RichText::new("Telemetry LMU")
                        .family(FontFamily::Name("RacingSansOne".into()))
                        .size(24.0)
                        .color(Color32::WHITE),
                )
                .selectable(false),
            );
        });
    }
}
