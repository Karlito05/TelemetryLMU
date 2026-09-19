// TODO: Take a look at this and refactor it a bit more mby

use crate::{
    components::button::button,
    frontend::frontend_main::{SettingsProvider, StateProvider},
    interface::{IPVehicleClass, TelemVect3},
};
use eframe::egui::*;
use std::{sync::Arc, time::Duration};

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
#[serde(default)]
pub struct MapPage {
    pub first_entry: bool,
    #[serde(skip)]
    pub(super) zoom: f32,
    #[serde(skip)]
    pub(super) offset: Vec2,
    #[serde(skip)]
    pub(super) time: f32,
    #[serde(skip)]
    pub(super) cur_dp_1: Option<usize>,
    #[serde(skip)]
    pub(super) car_1: Vec<Dp>,
    #[serde(skip)]
    pub(super) car_1_info: Option<CarInfo>,
    #[serde(skip)]
    pub(super) cur_dp_2: Option<usize>,
    #[serde(skip)]
    pub(super) car_2: Vec<Dp>,
    #[serde(skip)]
    pub(super) car_2_info: Option<CarInfo>,
    #[serde(skip)]
    pub(super) track_reference: Option<(Vec<Pos2>, Vec<Pos2>)>,
    #[serde(skip)]
    pub(super) replayer_state: ReplayerState,
    #[serde(skip)]
    pub(super) settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    pub(super) state_provider: Arc<StateProvider>,
    #[serde(skip)]
    pub(super) show_track_not_same_popup: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub(super) struct CarInfo {
    pub(super) class: IPVehicleClass,
    pub(super) car: String,
    pub(super) driver: String,
    pub(super) laptime: f32,
    pub(super) track: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub(super) struct Track {
    pub(super) line1: Vec<TelemVect3>,
    pub(super) line2: Vec<TelemVect3>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub(super) enum ReplayerState {
    #[default]
    Paused,
    Forwards,
    Backwards,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Copy, Clone, Debug)]
#[serde(default)]
pub(super) struct Dp {
    pub(super) distance: f32,
    pub(super) pos: Pos2,
    pub(super) time_since_lap_start: f32,
    pub(super) speed: f32,
    pub(super) gear: i32,
    pub(super) throttle: f32,
    pub(super) brake: f32,
    pub(super) steering: f32,
}

impl MapPage {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
    ) -> Self {
        Self {
            replayer_state: ReplayerState::default(),
            first_entry: true,
            time: 0.0,
            offset: Vec2::ZERO,
            zoom: 1.0,
            cur_dp_1: None,
            car_1: vec![],
            car_1_info: None,
            cur_dp_2: None,
            car_2: vec![],
            car_2_info: None,
            track_reference: None,
            settings_provider,
            state_provider,
            show_track_not_same_popup: false,
        }
    }
}

impl MapPage {
    pub fn draw_map_page(&mut self, ui: &mut Ui) {
        if self.first_entry && !*self.state_provider.global_first_launch.read().unwrap() {
            self.first_entry(ui);
        }
        {
            *self.state_provider.sidebar_open.write().unwrap() =
                ui.viewport_rect().width() > 1500.0;
        }
        let max_time = self
            .car_1_info
            .as_ref()
            .unwrap_or(&CarInfo::default())
            .laptime
            .max(
                self.car_2_info
                    .as_ref()
                    .unwrap_or(&CarInfo::default())
                    .laptime,
            );

        let time_1 = self.car_1_info.as_ref().map_or(0.0, |x| x.laptime);
        let time_2 = self.car_2_info.as_ref().map_or(0.0, |x| x.laptime);

        let offset = time_1.min(time_2) / time_1.max(time_2);

        let ref_len = self.car_1.len().max(self.car_2.len());
        if ref_len > 0 {
            if time_1 > time_2 {
                self.cur_dp_1 = self.get_current_dp_index(&self.car_1, time_1, 1.0);
                self.cur_dp_2 = self.get_current_dp_index(&self.car_2, time_2, offset);
            } else {
                self.cur_dp_1 = self.get_current_dp_index(&self.car_1, time_1, offset);
                self.cur_dp_2 = self.get_current_dp_index(&self.car_2, time_2, 1.0);
            }
        } else {
            self.cur_dp_1 = self.get_current_dp_index(&self.car_1, time_1, 1.0);
            self.cur_dp_2 = self.get_current_dp_index(&self.car_2, time_2, 1.0);
        }

        if self.show_track_not_same_popup {
            Window::new("Couldn't show lap")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("You can't compare 2 laps that are on a different track!");
                    ui.label("If you want to compare this lap with some other lap on the same track, please clear the pervious before proceeding.");

                    ui.horizontal(|ui| {
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Ok",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.show_track_not_same_popup = false;
                        }
                    });
                });
        }

        if max_time > 0.0 {
            match self.replayer_state {
                ReplayerState::Paused => {}
                ReplayerState::Forwards => {
                    ui.request_repaint_after(Duration::from_millis(16));
                    let dt = ui.input(|inp| inp.stable_dt); // actual elapsed seconds since last frame
                    self.time = (self.time + dt / max_time).clamp(0.0, 1.0);
                }
                ReplayerState::Backwards => {
                    ui.request_repaint_after(Duration::from_millis(16));
                    let dt = ui.input(|inp| inp.stable_dt); // actual elapsed seconds since last frame
                    self.time = (self.time - dt / max_time).clamp(0.0, 1.0);
                }
            }
        }
        let map_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                16.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        8.0
                    } else {
                        16.0
                    },
                ui.available_height() - 16.0,
            ),
        );

        if let Some(tr) = self.track_reference.clone() {
            self.draw_map_view(
                ui,
                map_rect,
                &[
                    (
                        &self.car_1.iter().map(|dp| dp.pos).collect(),
                        Color32::from_rgb(19, 141, 241),
                    ),
                    (
                        &self.car_2.iter().map(|dp| dp.pos).collect(),
                        Color32::from_rgb(255, 107, 53),
                    ),
                    (&tr.0, Color32::WHITE),
                    (&tr.1, Color32::WHITE),
                ],
            );
        } else {
            self.draw_map_view(
                ui,
                map_rect,
                &[
                    (
                        &self.car_1.iter().map(|dp| dp.pos).collect(),
                        Color32::from_rgb(19, 141, 241),
                    ),
                    (
                        &self.car_2.iter().map(|dp| dp.pos).collect(),
                        Color32::from_rgb(255, 107, 53),
                    ),
                ],
            );
        }
        if let Some(i) = self.cur_dp_1 {
            let car_1_pos = if i < self.car_1.len() {
                self.car_1[i].pos
            } else {
                if let Some(v) = self.car_1.last() {
                    v.pos
                } else {
                    pos2(0.0, 0.0)
                }
            };

            ui.painter().circle_filled(
                self.to_screen(map_rect, car_1_pos.to_vec2()),
                2.0 * self.zoom,
                Color32::from_rgb(19, 141, 241),
            );
        }

        if let Some(i) = self.cur_dp_2 {
            let car_2_pos = if i < self.car_2.len() {
                self.car_2[i].pos
            } else {
                if let Some(v) = self.car_2.last() {
                    v.pos
                } else {
                    pos2(0.0, 0.0)
                }
            };

            ui.painter().circle_filled(
                self.to_screen(map_rect, car_2_pos.to_vec2()),
                2.0 * self.zoom,
                Color32::from_rgb(255, 107, 53),
            );
        }

        let controls_rect = Rect::from_min_max(
            pos2(map_rect.min.x + 8.0, map_rect.max.y - 200.0),
            map_rect.max - vec2(8.0, 8.0),
        );
        self.draw_controls(ui, controls_rect);
    }
}
