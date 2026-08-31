// NOTE: Problems:
// - Gears are wrong for hypers
// - No track
// - Crashes on load of incorrect data
// - Playback controls not working
// - Time delta not working
// - Redisign the pick to include a clear button and some info about the lap (car time)
// - Make sure user picks a lap in the same class and on the same track

use std::{f32::consts::PI, fs};

use eframe::egui::*;
use egui_phosphor_icons::icons;

use crate::{
    backend::lap_stores::SaveData,
    frontend::{
        components::{button, slider},
        settings_page::Settings,
        sidebar::Sidebar,
    },
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MapPage {
    zoom: f32,
    offset: Vec2,
    time: f32,
    cur_dp_index: Option<usize>,
    car_1: Vec<Dp>,
    car_2: Vec<Dp>,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
struct Dp {
    pos: Pos2,
    time_since_lap_start: f64,
    speed: f32,
    gear: i32,
    throttle: f32,
    brake: f32,
    steering: f32,
}

impl Default for MapPage {
    fn default() -> Self {
        Self {
            time: 0.0,
            offset: Vec2::ZERO,
            zoom: 1.0,
            cur_dp_index: None,
            car_1: vec![],
            car_2: vec![],
        }
    }
}

impl MapPage {
    pub fn draw_map_page(&mut self, ui: &mut Ui, sidebar: &Sidebar, settings: &Settings) {
        let ref_len = self.car_1.len().max(self.car_2.len());
        self.cur_dp_index = Some((self.time * (ref_len - 1) as f32) as usize);

        let map_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
                ui.available_height() - 16.0,
            ),
        );

        self.draw_map(
            ui,
            map_rect,
            &[
                (
                    self.car_1.iter().map(|dp| dp.pos).collect(),
                    Color32::from_rgb(19, 141, 241),
                ),
                (
                    self.car_2.iter().map(|dp| dp.pos).collect(),
                    Color32::from_rgb(255, 107, 53),
                ),
            ],
        );
        if let Some(i) = self.cur_dp_index {
            let car_1_pos = if i < self.car_1.len() {
                self.car_1[i].pos
            } else {
                self.car_1.last().unwrap().pos
            };

            let car_2_pos = if i < self.car_2.len() {
                self.car_2[i].pos
            } else {
                self.car_2.last().unwrap().pos
            };

            ui.painter().circle_filled(
                self.to_screen(map_rect, car_1_pos.to_vec2()),
                2.0 * self.zoom,
                Color32::from_rgb(19, 141, 241),
            );
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
        self.draw_controls(ui, controls_rect, settings);
    }

    fn draw_controls(&mut self, ui: &mut Ui, rect: Rect, settings: &Settings) {
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
                            ui.label(
                                RichText::new("Blue:")
                                    .color(Color32::from_rgb(19, 141, 241))
                                    .size(16.0),
                            );
                            #[expect(clippy::collapsible_if)]
                            if button(
                                ui,
                                vec2(140.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_white_alpha(25),
                                "Select ref from file",
                                FontId::new(14.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_title("Select a reference file")
                                    .set_directory(settings.record_save_path.clone())
                                    .add_filter("JSON files", &["json"])
                                    .pick_file()
                                {
                                    let contents = fs::read_to_string(path).unwrap_or_default();
                                    let save_data: SaveData =
                                        serde_json::from_str(&contents).unwrap_or_default();

                                    self.cur_dp_index = None;
                                    self.car_1.clear();
                                    self.car_1 = save_data
                                        .pos_data
                                        .iter()
                                        .enumerate()
                                        .map(|(i, pd)| Dp {
                                            pos: pos2(pd.pos.x as f32, -pd.pos.z as f32),
                                            time_since_lap_start: pd.time_since_lap_start,
                                            speed: save_data
                                                .lap_data
                                                .iter()
                                                .find(|ld| ld.data_type == "speed")
                                                .unwrap()
                                                .values[i]
                                                .y
                                                * 350.0, // 350 is hardcoded max speed on the backend
                                            gear: (save_data
                                                .lap_data
                                                .iter()
                                                .find(|ld| ld.data_type == "gear")
                                                .unwrap()
                                                .values[i]
                                                .y
                                                * 6.0) // Do this based on car class
                                                as i32,
                                            throttle: save_data
                                                .lap_data
                                                .iter()
                                                .find(|ld| ld.data_type == "throttle")
                                                .unwrap()
                                                .values[i]
                                                .y,
                                            brake: save_data
                                                .lap_data
                                                .iter()
                                                .find(|ld| ld.data_type == "brake")
                                                .unwrap()
                                                .values[i]
                                                .y,
                                            steering: save_data
                                                .lap_data
                                                .iter()
                                                .find(|ld| ld.data_type == "steering")
                                                .unwrap()
                                                .values[i]
                                                .y,
                                        })
                                        .collect();
                                }
                            }

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                #[expect(clippy::collapsible_if)]
                                if button(
                                    ui,
                                    vec2(140.0, 32.0),
                                    CornerRadius::same(8),
                                    Color32::from_white_alpha(25),
                                    "Select ref from file",
                                    FontId::new(14.0, FontFamily::Proportional),
                                    Color32::WHITE,
                                )
                                .clicked()
                                {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .set_title("Select a reference file")
                                        .set_directory(settings.record_save_path.clone())
                                        .add_filter("JSON files", &["json"])
                                        .pick_file()
                                    {
                                        let contents = fs::read_to_string(path).unwrap_or_default();
                                        let save_data: SaveData =
                                            serde_json::from_str(&contents).unwrap_or_default();

                                        self.cur_dp_index = None;
                                        self.car_2.clear();
                                        self.car_2 = save_data
                                            .pos_data
                                            .iter()
                                            .enumerate()
                                            .map(|(i, pd)| Dp {
                                                pos: pos2(pd.pos.x as f32, -pd.pos.z as f32),
                                                time_since_lap_start: pd.time_since_lap_start,
                                                speed: save_data
                                                    .lap_data
                                                    .iter()
                                                    .find(|ld| ld.data_type == "speed")
                                                    .unwrap()
                                                    .values[i]
                                                    .y
                                                    * 350.0, // 350 is hardcoded max speed on the backend
                                                gear: (save_data
                                                    .lap_data
                                                    .iter()
                                                    .find(|ld| ld.data_type == "gear")
                                                    .unwrap()
                                                    .values[i]
                                                    .y
                                                    * 6.0) // Do this based on car class
                                                    as i32,
                                                throttle: save_data
                                                    .lap_data
                                                    .iter()
                                                    .find(|ld| ld.data_type == "throttle")
                                                    .unwrap()
                                                    .values[i]
                                                    .y,
                                                brake: save_data
                                                    .lap_data
                                                    .iter()
                                                    .find(|ld| ld.data_type == "brake")
                                                    .unwrap()
                                                    .values[i]
                                                    .y,
                                                steering: save_data
                                                    .lap_data
                                                    .iter()
                                                    .find(|ld| ld.data_type == "steering")
                                                    .unwrap()
                                                    .values[i]
                                                    .y,
                                            })
                                            .collect();
                                    }
                                }
                                ui.label(
                                    RichText::new("Orange:")
                                        .color(Color32::from_rgb(255, 107, 53))
                                        .size(16.0),
                                );
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
                        if resp.clicked() {}

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
                        if resp.clicked() {}

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
                        if resp.clicked() {}
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
                                                        self.car_1.last().unwrap().throttle
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
                                                        self.car_1.last().unwrap().brake
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
                                        "../../public/steering-wheel-blue.svg"
                                    ))
                                    .rotate(
                                        if let Some(i) = self.cur_dp_index {
                                            if i < self.car_1.len() {
                                                (self.car_1[i].steering - 0.5)
                                                    * 360.0
                                                    * (PI / 180.0)
                                            } else {
                                                (self.car_1.last().unwrap().steering - 0.5)
                                                    * 360.0
                                                    * (PI / 180.0)
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
                                                        self.car_1.last().unwrap().speed.round()
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
                                    ui.allocate_exact_size(vec2(30.0, 55.0), Sense::empty()).0;
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
                                                        self.car_1.last().unwrap().gear
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
                                                            self.car_2.last().unwrap().throttle
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
                                                            self.car_2.last().unwrap().brake
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
                                            "../../public/steering-wheel-orange.svg"
                                        ))
                                        .rotate(
                                            if let Some(i) = self.cur_dp_index {
                                                if i < self.car_1.len() {
                                                    (self.car_1[i].steering - 0.5)
                                                        * 360.0
                                                        * (PI / 180.0)
                                                } else {
                                                    (self.car_1.last().unwrap().steering - 0.5)
                                                        * 360.0
                                                        * (PI / 180.0)
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
                                                        if i < self.car_2.len() {
                                                            self.car_2[i].speed.round()
                                                        } else {
                                                            self.car_2.last().unwrap().speed.round()
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
                                        ui.allocate_exact_size(vec2(30.0, 55.0), Sense::empty()).0;
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
                                                            self.car_2.last().unwrap().gear
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

                ui.painter().text(
                    row3_rect.center(),
                    Align2::CENTER_CENTER,
                    "+0.345",
                    FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                    Color32::RED,
                )
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

    fn draw_map(&mut self, ui: &mut Ui, rect: Rect, lines: &[(Vec<Pos2>, Color32)]) {
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
}
