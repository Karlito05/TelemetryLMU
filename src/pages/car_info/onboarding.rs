use crate::{components::button::button, pages::car_info::car_info_main::CarInfo};
use eframe::egui::*;

impl CarInfo {
    pub(super) fn draw_onboarding_dialog(&mut self, ui: &mut Ui) {
        Window::new("Hello")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.set_min_size(vec2(500.0, 0.0));
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(
                            "Welcome to the car info page, where you can see the current state of your car. Because of a game limitation, this only works for your own car - the app identifies it automatically based on the in-game name you've set in your settings. Beyond that, the page is fairly self-explanatory.",
                        )
                        .size(18.0),
                    );
                    if button(
                        ui,
                        vec2(64.0, 32.0),
                        CornerRadius::same(8),
                        Color32::from_white_alpha(25),
                        "Close",
                        FontId::new(16.0, FontFamily::Proportional),
                        Color32::WHITE,
                    )
                    .clicked()
                    {
                        self.first_entry = false;
                    }
                });
            });
    }
}
