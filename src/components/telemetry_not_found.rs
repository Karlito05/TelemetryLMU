use eframe::egui::*;

pub fn telemetry_not_found(ui: &mut Ui) {
    Window::new("Error")
        .collapsible(false)
        .resizable(false)
        .min_width(800.0)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.label(
                RichText::new(
                    "To use this page, you need to have the game open when launching this app.",
                )
                .size(24.0)
                .strong(),
            );
            ui.separator();
            ui.label(
                RichText::new("Please restart this app or switch to another page!").size(16.0),
            );
        });
}
