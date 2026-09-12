// NOTE: Problems:
// - Crashes on load of incorrect data
// - Make sure user picks a lap in the same class and on the same track

use std::{f32::consts::PI, fs, sync::Arc, time::Duration};

use eframe::egui::*;
use egui_phosphor_icons::icons;

use crate::{
    frontend::{
        components::{button, slider},
        frontend_main::{SettingsProvider, StateProvider},
    },
    interface::{IPVehicleClass, TelemVect3},
    telemetry::{SaveData, TelemetryGraphValueType},
};

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
#[serde(default)]
pub struct MapPage {
    zoom: f32,
    offset: Vec2,
    time: f32,
    cur_dp_index: Option<usize>,
    car_1: Vec<Dp>,
    car_1_info: Option<CarInfo>,
    car_2: Vec<Dp>,
    car_2_info: Option<CarInfo>,
    track_reference: Option<(Vec<Pos2>, Vec<Pos2>)>,
    replayer_state: ReplayerState,
    #[serde(skip)]
    settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    state_provider: Arc<StateProvider>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
struct CarInfo {
    class: IPVehicleClass,
    car: String,
    driver: String,
    laptime: f32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
struct Track {
    line1: Vec<TelemVect3>,
    line2: Vec<TelemVect3>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
enum ReplayerState {
    #[default]
    Paused,
    Forwards,
    Backwards,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Copy, Clone, Debug)]
#[serde(default)]
struct Dp {
    distance: f32,
    pos: Pos2,
    time_since_lap_start: f32,
    speed: f32,
    gear: i32,
    throttle: f32,
    brake: f32,
    steering: f32,
}

impl MapPage {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
    ) -> Self {
        Self {
            replayer_state: ReplayerState::default(),
            time: 0.0,
            offset: Vec2::ZERO,
            zoom: 1.0,
            cur_dp_index: None,
            car_1: vec![],
            car_1_info: None,
            car_2: vec![],
            car_2_info: None,
            track_reference: None,
            settings_provider,
            state_provider,
        }
    }
}

impl MapPage {
    pub fn draw_map_page(&mut self, ui: &mut Ui) {
        *self.state_provider.sidebar_open.write().unwrap() = ui.viewport_rect().width() > 1500.0;

        let ref_len = self.car_1.len().max(self.car_2.len());
        if ref_len > 0 {
            self.cur_dp_index = Some((self.time * (ref_len - 1) as f32) as usize);
        }

        if let Some(i) = self.cur_dp_index.as_mut() {
            match self.replayer_state {
                ReplayerState::Paused => {}
                ReplayerState::Forwards => {
                    ui.request_repaint_after(Duration::from_millis(16));
                    let dt = ui.input(|inp| inp.stable_dt); // actual elapsed seconds since last frame
                    let total_duration_secs = self
                        .car_1
                        .last()
                        .copied()
                        .unwrap_or_default()
                        .time_since_lap_start
                        .max(
                            self.car_2
                                .last()
                                .copied()
                                .unwrap_or_default()
                                .time_since_lap_start,
                        );
                    self.time = (self.time + dt / total_duration_secs).min(1.0);
                    *i = (self.time * (ref_len - 1) as f32) as usize;
                }
                ReplayerState::Backwards => {
                    ui.request_repaint_after(Duration::from_millis(16));
                    if *i > 0 {
                        *i -= 1;
                    }
                    self.time = *i as f32 / (ref_len - 1).max(1) as f32;
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
            self.draw_map(
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
            self.draw_map(
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
        if let Some(i) = self.cur_dp_index {
            if !self.car_1.is_empty() {
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

            if !self.car_2.is_empty() {
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
        }

        let controls_rect = Rect::from_min_max(
            pos2(map_rect.min.x + 8.0, map_rect.max.y - 200.0),
            map_rect.max - vec2(8.0, 8.0),
        );
        self.draw_controls(ui, controls_rect);
    }

    fn draw_controls(&mut self, ui: &mut Ui, rect: Rect) {
        ui.painter()
            .rect_filled(rect, 16, Color32::from_white_alpha(17));

        let usable_rect =
            Rect::from_min_max(rect.min + vec2(16.0, 16.0), rect.max - vec2(16.0, 16.0));

        ui.put(usable_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                slider(
                    ui,
                    vec2(ui.available_width(), 24.0),
                    Color32::from_rgb(19, 141, 241),
                    &mut self.time,
                    0.0,
                    1.0,
                    8.0,
                );

                let row_rect = ui
                    .add_sized(vec2(ui.available_width(), 48.0), |ui: &mut Ui| {
                        ui.horizontal(|ui| {
                            #[expect(clippy::collapsible_if)]
                            if button(
                                ui,
                                vec2(140.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_rgb(19, 141, 241),
                                "Select Reference",
                                FontId::new(14.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Select a reference file")
                                    .set_directory(
                                        self.settings_provider
                                            .record_save_path
                                            .read()
                                            .unwrap()
                                            .clone(),
                                    )
                                    .add_filter("JSON files", &["json"])
                                    .pick_file()
                                {
                                    let contents = fs::read_to_string(path).unwrap_or_default();
                                    let save_data: SaveData =
                                        serde_json::from_str(&contents).unwrap_or_default();

                                    let mut track: Option<Track> = None;
                                    set_track_reference(&mut track, save_data.track.as_str());

                                    self.track_reference = Some((
                                        track
                                            .as_ref()
                                            .unwrap()
                                            .line1
                                            .iter()
                                            .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                            .collect(),
                                        track
                                            .unwrap()
                                            .line2
                                            .iter()
                                            .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                            .collect(),
                                    ));

                                    self.cur_dp_index = None;
                                    self.car_1.clear();
                                    self.car_1_info = Some(CarInfo {
                                        class: save_data.car_class,
                                        car: save_data.car,
                                        driver: save_data.driver_name,
                                        laptime: save_data.lap_time,
                                    });
                                    self.car_1 = save_data
                                        .positions
                                        .iter()
                                        .enumerate()
                                        .map(|(mut i, pd)| {
                                            if i > 21600 {
                                                i = 21600;
                                            }
                                            Dp {
                                                pos: pos2(pd.x as f32, -pd.z as f32),
                                                distance: save_data.distances[i],
                                                time_since_lap_start: save_data.times[i],
                                                speed: save_data.lap_data
                                                    [TelemetryGraphValueType::Speed as usize][i],
                                                gear: save_data.lap_data
                                                    [TelemetryGraphValueType::Gear as usize][i]
                                                    as i32,
                                                throttle: save_data.lap_data
                                                    [TelemetryGraphValueType::Throttle as usize][i],
                                                brake: save_data.lap_data
                                                    [TelemetryGraphValueType::Brake as usize][i],
                                                steering: save_data.lap_data
                                                    [TelemetryGraphValueType::Steering as usize][i]
                                                    + 0.5,
                                            }
                                        })
                                        .collect();
                                }
                            }
                            if button(
                                ui,
                                vec2(140.0, 32.0),
                                CornerRadius::same(8),
                                Color32::RED,
                                "Clear",
                                FontId::new(14.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                self.car_1_info = None;
                                self.car_1 = vec![];
                            }

                            if let Some(info) = self.car_1_info.clone() {
                                let driver_info_rect =
                                    ui.allocate_exact_size(vec2(140.0, 36.0), Sense::empty()).0;
                                ui.put(driver_info_rect, |ui: &mut Ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(info.driver.clone())
                                                    .size(16.0)
                                                    .color(Color32::WHITE),
                                            );
                                            let badge_rect = ui
                                                .allocate_exact_size(
                                                    vec2(40.0, 16.0),
                                                    Sense::empty(),
                                                )
                                                .0;
                                            draw_badge(info.class, ui, badge_rect);
                                        });
                                        ui.label(format!(
                                            "{}:{:.3}",
                                            (info.laptime / 60.0).trunc(),
                                            info.laptime % 60.0
                                        ));
                                    })
                                    .response
                                });
                            }

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                #[expect(clippy::collapsible_if)]
                                if button(
                                    ui,
                                    vec2(140.0, 32.0),
                                    CornerRadius::same(8),
                                    Color32::from_rgb(255, 107, 53),
                                    "Select Reference",
                                    FontId::new(14.0, FontFamily::Proportional),
                                    Color32::WHITE,
                                )
                                .clicked()
                                {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .set_title("Select a reference file")
                                        .set_directory(
                                            self.settings_provider
                                                .record_save_path
                                                .read()
                                                .unwrap()
                                                .clone(),
                                        )
                                        .add_filter("JSON files", &["json"])
                                        .pick_file()
                                    {
                                        let contents = fs::read_to_string(path).unwrap_or_default();
                                        let save_data: SaveData =
                                            serde_json::from_str(&contents).unwrap_or_default();

                                        let mut track: Option<Track> = None;
                                        set_track_reference(&mut track, save_data.track.as_str());

                                        self.track_reference = Some((
                                            track
                                                .as_ref()
                                                .unwrap()
                                                .line1
                                                .iter()
                                                .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                                .collect(),
                                            track
                                                .unwrap()
                                                .line2
                                                .iter()
                                                .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                                .collect(),
                                        ));

                                        self.cur_dp_index = None;
                                        self.car_2_info = Some(CarInfo {
                                            class: save_data.car_class,
                                            car: save_data.car,
                                            driver: save_data.driver_name,
                                            laptime: save_data.lap_time,
                                        });
                                        self.car_2.clear();
                                        self.car_2 = save_data
                                            .positions
                                            .iter()
                                            .enumerate()
                                            .map(|(i, pd)| Dp {
                                                pos: pos2(pd.x as f32, -pd.z as f32),
                                                distance: save_data.distances[i],
                                                time_since_lap_start: save_data.times[i],
                                                speed: save_data.lap_data
                                                    [TelemetryGraphValueType::Speed as usize][i],
                                                gear: save_data.lap_data
                                                    [TelemetryGraphValueType::Gear as usize][i]
                                                    as i32,
                                                throttle: save_data.lap_data
                                                    [TelemetryGraphValueType::Throttle as usize][i],
                                                brake: save_data.lap_data
                                                    [TelemetryGraphValueType::Brake as usize][i],
                                                steering: save_data.lap_data
                                                    [TelemetryGraphValueType::Steering as usize][i]
                                                    + 0.5,
                                            })
                                            .collect();
                                    }
                                }

                                if button(
                                    ui,
                                    vec2(140.0, 32.0),
                                    CornerRadius::same(8),
                                    Color32::RED,
                                    "Clear",
                                    FontId::new(14.0, FontFamily::Proportional),
                                    Color32::WHITE,
                                )
                                .clicked()
                                {
                                    self.car_2_info = None;
                                    self.car_2 = vec![];
                                }
                                if let Some(info) = self.car_2_info.clone() {
                                    let driver_info_rect =
                                        ui.allocate_exact_size(vec2(140.0, 36.0), Sense::empty()).0;
                                    ui.put(driver_info_rect, |ui: &mut Ui| {
                                        ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                            ui.with_layout(
                                                Layout::right_to_left(Align::Min),
                                                |ui| {
                                                    ui.label(
                                                        RichText::new(info.driver.clone())
                                                            .size(16.0)
                                                            .color(Color32::WHITE),
                                                    );
                                                    let badge_rect = ui
                                                        .allocate_exact_size(
                                                            vec2(40.0, 16.0),
                                                            Sense::empty(),
                                                        )
                                                        .0;
                                                    draw_badge(info.class, ui, badge_rect);
                                                },
                                            );
                                            ui.label(format!(
                                                "{}:{:.3}",
                                                (info.laptime / 60.0).trunc(),
                                                info.laptime % 60.0
                                            ));
                                        })
                                        .response
                                    });
                                }
                            });
                        })
                        .response
                    })
                    .rect;

                let spacing = ui.spacing().item_spacing.x;
                let group_size = vec2(48.0 * 3.0 + spacing * 2.0, 48.0);
                let group_rect = Rect::from_center_size(row_rect.center(), group_size);

                ui.put(group_rect, |ui: &mut Ui| {
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

                let row3_rect = ui
                    .add_sized(
                        vec2(ui.available_width(), ui.available_height()),
                        |ui: &mut Ui| {
                            ui.horizontal_centered(|ui| {
                                let (bars_rect, _) =
                                    ui.allocate_exact_size(vec2(200.0, 40.0), Sense::empty());

                                let throttle_rect =
                                    Rect::from_min_size(bars_rect.min, vec2(200.0, 16.0));
                                let brake_rect = Rect::from_min_max(
                                    bars_rect.max - vec2(200.0, 16.0),
                                    bars_rect.max,
                                );

                                ui.painter().rect_filled(
                                    throttle_rect,
                                    8,
                                    Color32::from_white_alpha(25),
                                );
                                ui.painter().rect_filled(
                                    Rect::from_min_size(
                                        throttle_rect.min,
                                        throttle_rect.size()
                                            * vec2(
                                                if let Some(i) = self.cur_dp_index {
                                                    if i < self.car_1.len() {
                                                        self.car_1[i].throttle
                                                    } else {
                                                        if let Some(last) = self.car_1.last() {
                                                            last.throttle
                                                        } else {
                                                            0.0
                                                        }
                                                    }
                                                } else {
                                                    0.0
                                                },
                                                1.0,
                                            ),
                                    ),
                                    8,
                                    Color32::from_rgb(0, 255, 0),
                                );

                                ui.painter().rect_filled(
                                    brake_rect,
                                    8,
                                    Color32::from_white_alpha(25),
                                );
                                ui.painter().rect_filled(
                                    Rect::from_min_size(
                                        brake_rect.min,
                                        brake_rect.size()
                                            * vec2(
                                                if let Some(i) = self.cur_dp_index {
                                                    if i < self.car_1.len() {
                                                        self.car_1[i].brake
                                                    } else {
                                                        if let Some(last) = self.car_1.last() {
                                                            last.brake
                                                        } else {
                                                            0.0
                                                        }
                                                    }
                                                } else {
                                                    0.0
                                                },
                                                1.0,
                                            ),
                                    ),
                                    8,
                                    Color32::from_rgb(255, 0, 0),
                                );

                                ui.add_space(8.0);

                                ui.add(
                                    Image::new(include_image!(
                                        "../../public/icons/steering-wheel-blue.svg"
                                    ))
                                    .rotate(
                                        if let Some(i) = self.cur_dp_index {
                                            if i < self.car_1.len() {
                                                (self.car_1[i].steering - 0.5)
                                                    * 360.0
                                                    * (PI / 180.0)
                                            } else {
                                                if let Some(last) = self.car_1.last() {
                                                    (last.steering - 0.5) * 360.0 * (PI / 180.0)
                                                } else {
                                                    0.0
                                                }
                                            }
                                        } else {
                                            0.0
                                        },
                                        Vec2::splat(0.5),
                                    ),
                                );

                                ui.separator();

                                let rect =
                                    ui.allocate_exact_size(vec2(100.0, 55.0), Sense::empty()).0;
                                ui.put(rect, |ui: &mut Ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new("Speed")
                                                .size(12.0)
                                                .color(Color32::from_white_alpha(64)),
                                        );
                                        ui.label(
                                            RichText::new(if let Some(i) = self.cur_dp_index {
                                                format!(
                                                    "{}km/h",
                                                    if i < self.car_1.len() {
                                                        self.car_1[i].speed.round()
                                                    } else {
                                                        if let Some(last) = self.car_1.last() {
                                                            last.speed.round()
                                                        } else {
                                                            0.0
                                                        }
                                                    }
                                                )
                                            } else {
                                                "N/A".to_owned()
                                            })
                                            .size(24.0)
                                            .color(Color32::WHITE)
                                            .family(FontFamily::Name("JetBrainsMono".into())),
                                        );
                                    })
                                    .response
                                });

                                ui.separator();

                                let rect =
                                    ui.allocate_exact_size(vec2(50.0, 55.0), Sense::empty()).0;
                                ui.put(rect, |ui: &mut Ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            RichText::new("Gear")
                                                .size(12.0)
                                                .color(Color32::from_white_alpha(64)),
                                        );
                                        ui.label(
                                            RichText::new(if let Some(i) = self.cur_dp_index {
                                                format!(
                                                    "{}",
                                                    if i < self.car_1.len() {
                                                        self.car_1[i].gear
                                                    } else {
                                                        if let Some(last) = self.car_1.last() {
                                                            last.gear
                                                        } else {
                                                            0
                                                        }
                                                    }
                                                )
                                            } else {
                                                "N/A".to_owned()
                                            })
                                            .size(24.0)
                                            .color(Color32::WHITE)
                                            .family(FontFamily::Name("JetBrainsMono".into())),
                                        );
                                    })
                                    .response
                                });

                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let (bars_rect, _) =
                                        ui.allocate_exact_size(vec2(200.0, 40.0), Sense::empty());

                                    let throttle_rect =
                                        Rect::from_min_size(bars_rect.min, vec2(200.0, 16.0));
                                    let brake_rect = Rect::from_min_max(
                                        bars_rect.max - vec2(200.0, 16.0),
                                        bars_rect.max,
                                    );

                                    ui.painter().rect_filled(
                                        throttle_rect,
                                        8,
                                        Color32::from_white_alpha(25),
                                    );
                                    ui.painter().rect_filled(
                                        Rect::from_min_size(
                                            throttle_rect.min,
                                            throttle_rect.size()
                                                * vec2(
                                                    if let Some(i) = self.cur_dp_index {
                                                        if i < self.car_2.len() {
                                                            self.car_2[i].throttle
                                                        } else {
                                                            if let Some(v) = self.car_2.last() {
                                                                v.throttle
                                                            } else {
                                                                0.0
                                                            }
                                                        }
                                                    } else {
                                                        0.0
                                                    },
                                                    1.0,
                                                ),
                                        ),
                                        8,
                                        Color32::from_rgb(0, 255, 0),
                                    );

                                    ui.painter().rect_filled(
                                        brake_rect,
                                        8,
                                        Color32::from_white_alpha(25),
                                    );
                                    ui.painter().rect_filled(
                                        Rect::from_min_size(
                                            brake_rect.min,
                                            brake_rect.size()
                                                * vec2(
                                                    if let Some(i) = self.cur_dp_index {
                                                        if i < self.car_2.len() {
                                                            self.car_2[i].brake
                                                        } else {
                                                            if let Some(v) = self.car_2.last() {
                                                                v.brake
                                                            } else {
                                                                0.0
                                                            }
                                                        }
                                                    } else {
                                                        0.0
                                                    },
                                                    1.0,
                                                ),
                                        ),
                                        8,
                                        Color32::from_rgb(255, 0, 0),
                                    );

                                    ui.add_space(8.0);

                                    ui.add(
                                        Image::new(include_image!(
                                            "../../public/icons/steering-wheel-orange.svg"
                                        ))
                                        .rotate(
                                            if let Some(i) = self.cur_dp_index {
                                                if i < self.car_2.len() {
                                                    (self.car_2[i].steering - 0.5)
                                                        * 360.0
                                                        * (PI / 180.0)
                                                } else {
                                                    if let Some(last) = self.car_2.last() {
                                                        (last.steering - 0.5) * 360.0 * (PI / 180.0)
                                                    } else {
                                                        0.0
                                                    }
                                                }
                                            } else {
                                                0.0
                                            },
                                            Vec2::splat(0.5),
                                        ),
                                    );

                                    ui.separator();

                                    let rect =
                                        ui.allocate_exact_size(vec2(100.0, 55.0), Sense::empty()).0;
                                    ui.put(rect, |ui: &mut Ui| {
                                        ui.with_layout(Layout::top_down(Align::Max), |ui| {
                                            ui.label(
                                                RichText::new("Speed")
                                                    .size(12.0)
                                                    .color(Color32::from_white_alpha(64)),
                                            );
                                            ui.label(
                                                RichText::new(if let Some(i) = self.cur_dp_index {
                                                    format!(
                                                        "{}km/h",
                                                        if i < self.car_2.len() {
                                                            self.car_2[i].speed.round()
                                                        } else {
                                                            if let Some(v) = self.car_2.last() {
                                                                v.speed.round()
                                                            } else {
                                                                0.0
                                                            }
                                                        }
                                                    )
                                                } else {
                                                    "N/A".to_owned()
                                                })
                                                .size(24.0)
                                                .color(Color32::WHITE)
                                                .family(FontFamily::Name("JetBrainsMono".into())),
                                            );
                                        })
                                        .response
                                    });

                                    ui.separator();

                                    let rect =
                                        ui.allocate_exact_size(vec2(50.0, 55.0), Sense::empty()).0;
                                    ui.put(rect, |ui: &mut Ui| {
                                        ui.vertical_centered(|ui| {
                                            ui.label(
                                                RichText::new("Gear")
                                                    .size(12.0)
                                                    .color(Color32::from_white_alpha(64)),
                                            );
                                            ui.label(
                                                RichText::new(if let Some(i) = self.cur_dp_index {
                                                    format!(
                                                        "{}",
                                                        if i < self.car_2.len() {
                                                            self.car_2[i].gear
                                                        } else {
                                                            if let Some(v) = self.car_2.last() {
                                                                v.gear
                                                            } else {
                                                                0
                                                            }
                                                        }
                                                    )
                                                } else {
                                                    "N/A".to_owned()
                                                })
                                                .size(24.0)
                                                .color(Color32::WHITE)
                                                .family(FontFamily::Name("JetBrainsMono".into())),
                                            );
                                        })
                                        .response
                                    });
                                })
                            })
                            .response
                        },
                    )
                    .rect;

                if let Some(delta) = self.get_time_delta() {
                    if delta < 0.0 {
                        ui.painter().text(
                            row3_rect.center(),
                            Align2::CENTER_CENTER,
                            format!("{:.3}", delta),
                            FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                            Color32::GREEN,
                        )
                    } else {
                        ui.painter().text(
                            row3_rect.center(),
                            Align2::CENTER_CENTER,
                            format!("+{:.3}", delta),
                            FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                            Color32::RED,
                        )
                    }
                } else {
                    ui.painter().text(
                        row3_rect.center(),
                        Align2::CENTER_CENTER,
                        "N/A",
                        FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                        Color32::RED,
                    )
                }
            })
            .response
        });
    }
    fn to_screen(&self, rect: Rect, p: Vec2) -> Pos2 {
        rect.center() + (p * self.zoom) + self.offset
    }

    fn to_world(&self, rect: Rect, p: Pos2) -> Vec2 {
        (p - rect.center() - self.offset) / self.zoom
    }

    fn draw_map(&mut self, ui: &mut Ui, rect: Rect, lines: &[(&Vec<Pos2>, Color32)]) {
        ui.painter()
            .rect_filled(rect, CornerRadius::same(24), Color32::from_rgb(22, 23, 28));
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter().with_clip_rect(rect);

        if response.dragged() {
            self.offset += response.drag_delta();
        }

        if response.dragged_by(PointerButton::Secondary) {
            self.offset += response.drag_delta();
        }

        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let mouse = response.hover_pos().unwrap_or(rect.center());
                let world = self.to_world(rect, mouse);

                self.zoom = (self.zoom * (1.0 + scroll * 0.01)).clamp(0.1, 10.0);

                self.offset = mouse - rect.center() - world * self.zoom;
            }
        }

        for line in lines {
            let points: Vec<Pos2> = line
                .0
                .iter()
                .map(|p| self.to_screen(rect, p.to_vec2()))
                .collect();
            painter.add(Shape::line(points, Stroke::new(0.5 * self.zoom, line.1)));
        }
    }
    fn get_time_delta(&self) -> Option<f32> {
        let i = self.cur_dp_index?;
        let car1_cur = self.car_1.get(i)?.distance;
        let car2_cur = self.car_2.get(i)?.distance;

        if car1_cur > car2_cur {
            let mut last_diff = car1_cur - car2_cur;
            let mut closest_index = i;
            for j in i..self.car_2.len() {
                if (self.car_2[j].distance - car1_cur).abs() < last_diff {
                    last_diff = self.car_2[j].distance - car1_cur;
                    closest_index = j;
                } else {
                    break;
                }
            }
            return Some(
                self.car_2[closest_index].time_since_lap_start - self.car_1[i].time_since_lap_start,
            );
        }
        if car1_cur <= car2_cur {
            let mut last_diff = car1_cur - car2_cur;
            let mut closest_index = i;
            for j in (0..i).rev() {
                if (self.car_2[j].distance - car1_cur).abs() < last_diff {
                    last_diff = self.car_2[j].distance - car1_cur;
                    closest_index = j;
                } else {
                    break;
                }
            }
            return Some(
                self.car_2[closest_index].time_since_lap_start - self.car_1[i].time_since_lap_start,
            );
        }

        None
    }
}
fn draw_badge(class: IPVehicleClass, ui: &mut Ui, badge_rect: Rect) {
    let draw_badge = |badge_rect: Rect, color: Color32, name: &str| {
        ui.painter().rect(
            badge_rect,
            CornerRadius::same(4),
            color,
            Stroke::new(2.0, color.to_opaque()),
            StrokeKind::Inside,
        );
        ui.painter().text(
            badge_rect.center(),
            Align2::CENTER_CENTER,
            name,
            FontId::new(14.0, FontFamily::Name("RethinkSans".into())),
            color.to_opaque(),
        );
    };

    match class {
        IPVehicleClass::Gt3 => draw_badge(
            badge_rect,
            Color32::from_rgba_unmultiplied(13, 157, 0, 64),
            "GT3",
        ),

        IPVehicleClass::Gte => draw_badge(
            badge_rect,
            Color32::from_rgba_unmultiplied(255, 204, 0, 64),
            "GTE",
        ),
        IPVehicleClass::Lmp3 => draw_badge(
            badge_rect,
            Color32::from_rgba_unmultiplied(123, 0, 255, 64),
            "LMP3",
        ),
        IPVehicleClass::Lmp2 | IPVehicleClass::Lmp2Elms => draw_badge(
            badge_rect,
            Color32::from_rgba_unmultiplied(0, 127, 221, 64),
            "LMP2",
        ),
        IPVehicleClass::Hypercar => draw_badge(
            badge_rect,
            Color32::from_rgba_unmultiplied(223, 39, 28, 64),
            "HY",
        ),
        _ => {}
    };
}

fn set_track_reference(track_reference: &mut Option<Track>, track: &str) {
    match track {
        "Algrave International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Algarve International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Autodromo Enzo e Dino Ferrari" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Autodromo Enzo e Dino Ferrari.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Autodromo Nazionale Monza" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Autodromo Nazionale Monza.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Autodrómo José Carlos Pace" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Autódromo José Carlos Pace.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain Endurance Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Bahrain Endurance Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Bahrain International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain Outer Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Bahrain Outer Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Bahrain Paddock Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Bahrain Paddock Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "COTA National Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/COTA National Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de Barcelona" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Circuit de Barcelona.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de Spa-Francorchamps" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Circuit de Spa-Francorchamps.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de la Sarthe Mulsanne" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Circuit de la Sarthe Mulsanne.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit de la Sarthe" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Circuit de la Sarthe.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Circuit of the Americas" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Circuit of the Americas.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Daytona International Speedway Road Course" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Daytona International Speedway Road Course.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Fuji Speedway" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!("../../public/tracks/Fuji Speedway.json"))
                    .unwrap_or_default(),
            )
        }
        "Fuji Speedway Classic" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Fuji Speedway Classic.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Lusail International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Lusail International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Lusail Short Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Lusail Short Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Monza Curva Grande Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Monza Curva Grande Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 1A-V2-Short" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Paul Ricard - 1A-V2-Short.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 1A-V2" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Paul Ricard - 1A-V2.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Paul Ricard - 1A" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!("../../public/tracks/Paul Ricard - 1A.json"))
                    .unwrap_or_default(),
            )
        }
        "Paul Ricard - 3A" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!("../../public/tracks/Paul Ricard - 3A.json"))
                    .unwrap_or_default(),
            )
        }
        "Paul Ricard - ELMS" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Paul Ricard - ELMS.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Sebring International Raceway" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Sebring International Raceway.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Sebring School Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Sebring School Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Silverstone Grand Prix Circuit - WEC" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Silverstone Grand Prix Circuit - WEC.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Silverstone International Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Silverstone International Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "Silverstone National Circuit" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/Silverstone National Circuit.json"
                ))
                .unwrap_or_default(),
            )
        }
        "WeatherTech Raceway Laguna Seca" => {
            *track_reference = Some(
                serde_json::from_slice(include_bytes!(
                    "../../public/tracks/WeatherTech Raceway Laguna Seca.json"
                ))
                .unwrap_or_default(),
            )
        }
        &_ => println!("Unknown track"),
    }
}
