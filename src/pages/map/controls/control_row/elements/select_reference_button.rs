use std::{fs, sync::Arc};

use crate::{
    components::button::button,
    frontend::frontend_main::SettingsProvider,
    pages::map::{
        map_main::{CarInfo, Dp, MapPage, Track},
        utils::track_reference::set_track_reference,
    },
    telemetry::{SaveData, TelemetryGraphValueType},
};
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::control_row) fn draw_select_reference_button(
        ui: &mut Ui,
        cur_dp: &mut Option<usize>,
        car: &mut Vec<Dp>,
        car_info: &mut Option<CarInfo>,
        car_info_other: &Option<CarInfo>,
        settings: Arc<SettingsProvider>,
        track_reference: &mut Option<(Vec<Pos2>, Vec<Pos2>)>,
        show_track_not_same_popup: &mut bool,
    ) {
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
                .set_directory(settings.record_save_path.read().unwrap().clone())
                .add_filter("JSON files", &["json"])
                .pick_file()
            {
                let contents = fs::read_to_string(path).unwrap_or_default();
                let save_data: SaveData = serde_json::from_str(&contents).unwrap_or_default();

                let mut track: Option<Track> = None;
                set_track_reference(&mut track, save_data.track.as_str());

                if let Some(info) = car_info_other {
                    if info.track == save_data.track {
                        if let Some(track) = track {
                            *track_reference = Some((
                                track
                                    .line1
                                    .iter()
                                    .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                    .collect(),
                                track
                                    .line2
                                    .iter()
                                    .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                    .collect(),
                            ));
                        }
                        *cur_dp = None;
                        car.clear();
                        *car_info = Some(CarInfo {
                            class: save_data.car_class,
                            car: save_data.car,
                            driver: save_data.driver_name,
                            laptime: save_data.lap_time,
                            track: save_data.track,
                        });
                        *car = save_data
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
                                    gear: save_data.lap_data[TelemetryGraphValueType::Gear as usize]
                                        [i] as i32,
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
                    } else {
                        *show_track_not_same_popup = true;
                    }
                } else {
                    if let Some(track) = track {
                        *track_reference = Some((
                            track
                                .line1
                                .iter()
                                .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                .collect(),
                            track
                                .line2
                                .iter()
                                .map(|tv| pos2(tv.x as f32, -tv.z as f32))
                                .collect(),
                        ));
                    }
                    *cur_dp = None;
                    car.clear();
                    *car_info = Some(CarInfo {
                        class: save_data.car_class,
                        car: save_data.car,
                        driver: save_data.driver_name,
                        laptime: save_data.lap_time,
                        track: save_data.track,
                    });
                    *car = save_data
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
                                speed: save_data.lap_data[TelemetryGraphValueType::Speed as usize]
                                    [i],
                                gear: save_data.lap_data[TelemetryGraphValueType::Gear as usize][i]
                                    as i32,
                                throttle: save_data.lap_data
                                    [TelemetryGraphValueType::Throttle as usize][i],
                                brake: save_data.lap_data[TelemetryGraphValueType::Brake as usize]
                                    [i],
                                steering: save_data.lap_data
                                    [TelemetryGraphValueType::Steering as usize][i]
                                    + 0.5,
                            }
                        })
                        .collect();
                }
            }
        }
    }
}
