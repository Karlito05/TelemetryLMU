use crate::{components::button::button, pages::map::map_main::MapPage};
use eframe::egui::*;

impl MapPage {
    pub(super) fn first_entry(&mut self, ui: &mut Ui) {
        Window::new("Hello")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.set_min_size(vec2(500.0, 0.0));
                ui.vertical(|ui| {
                    ui.label(RichText::new("Welcome to the map page, where you can review your laps and see exactly where you drove. Selecting any lap recorded by the app will bring it up on the map for review.").size(18.0));
                    ui.label(RichText::new("
The bottom control bar gives you a closer look at that lap: your inputs (throttle in green, brakes in red, and steering wheel rotation), along with details like the driver, lap time, and car class. Clicking the play button lets you watch the lap unfold in real time. You can also bring in a second lap - useful for comparing against a reference - and the app will show you the live delta between the two as they play out.").size(18.0)
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
