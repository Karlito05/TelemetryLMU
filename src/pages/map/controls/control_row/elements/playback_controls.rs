use crate::pages::map::map_main::{MapPage, ReplayerState};
use eframe::egui::*;
use egui_phosphor_icons::icons;

impl MapPage {
    pub(in crate::pages::map::controls::control_row) fn draw_playback_controls(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
    ) {
        ui.put(rect, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                let resp = ui.add_sized(
                    vec2(48.0, 48.0),
                    Button::new(
                        icons::SKIP_BACK
                            .regular()
                            .size(32.0)
                            .color(Color32::from_rgb(19, 141, 241)),
                    )
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE)
                    .small(),
                );
                if resp.hovered() {
                    ui.painter()
                        .rect_filled(resp.rect, 24, Color32::from_white_alpha(25));
                }
                if resp.clicked() {
                    self.replayer_state = ReplayerState::Backwards;
                }

                let resp = ui.add_sized(
                    vec2(48.0, 48.0),
                    Button::new(
                        icons::PAUSE
                            .regular()
                            .size(32.0)
                            .color(Color32::from_rgb(19, 141, 241)),
                    )
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE)
                    .small(),
                );
                if resp.hovered() {
                    ui.painter()
                        .rect_filled(resp.rect, 24, Color32::from_white_alpha(25));
                }
                if resp.clicked() {
                    self.replayer_state = ReplayerState::Paused;
                }

                let resp = ui.add_sized(
                    vec2(48.0, 48.0),
                    Button::new(
                        icons::SKIP_FORWARD
                            .regular()
                            .size(32.0)
                            .color(Color32::from_rgb(19, 141, 241)),
                    )
                    .fill(Color32::TRANSPARENT)
                    .stroke(Stroke::NONE)
                    .small(),
                );
                if resp.hovered() {
                    ui.painter()
                        .rect_filled(resp.rect, 24, Color32::from_white_alpha(25));
                }
                if resp.clicked() {
                    self.replayer_state = ReplayerState::Forwards;
                }
            })
            .response
        });
    }
}
