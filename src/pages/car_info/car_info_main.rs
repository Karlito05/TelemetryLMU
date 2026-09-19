use std::{sync::Arc, time::Duration};

use eframe::egui::*;

use crate::{
    components::telemetry_not_found::telemetry_not_found,
    providers::{
        settings_provider::SettingsProvider,
        state_provider::StateProvider,
        telemetry_provider::{interface::IPVehicleClass, telemetry_provider_main::Telemetry},
    },
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct CarInfo {
    pub first_entry: bool,
    #[serde(skip)]
    pub(super) inputs: Inputs,
    #[serde(skip)]
    pub(super) name: String,
    #[serde(skip)]
    pub(super) car: String,
    #[serde(skip)]
    pub(super) car_class: IPVehicleClass,
    #[serde(skip)]
    pub(super) driver_index: usize,
    #[serde(skip)]
    pub(super) fuel_info: FuelInfo,
    #[serde(skip)]
    pub(super) tires: [TireInfo; 4],
    #[serde(skip)]
    pub(super) telemetry_provider: Arc<Option<Telemetry>>,
    #[serde(skip)]
    pub(super) settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    pub(super) state_provider: Arc<StateProvider>,
}

#[derive(Debug, Default)]
pub(super) struct FuelInfo {
    pub(super) fuel_percent: f32,
    pub(super) virt_eng_percent: f32,
    pub(super) fuel_liters: f32,
}

#[derive(Debug, Default)]
pub(super) struct Inputs {
    pub(super) brake: f32,
    pub(super) throttle: f32,
    pub(super) steering: f32,
}

#[derive(Default, Debug)]
pub(super) struct TireInfo {
    pub(super) inside_temp: f32,
    pub(super) outside_temp: f32,
    pub(super) brake_temp: f32,
    pub(super) health_percent: f32,
}

impl CarInfo {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
        telemetry_provider: Arc<Option<Telemetry>>,
    ) -> Self {
        Self {
            first_entry: true,
            telemetry_provider,
            driver_index: 0,
            name: "".to_string(),
            car: "".to_string(),
            car_class: IPVehicleClass::Unknown,
            fuel_info: FuelInfo {
                fuel_percent: 0.0,
                virt_eng_percent: 0.0,
                fuel_liters: 0.0,
            },
            inputs: Inputs::default(),
            tires: [
                TireInfo::default(),
                TireInfo::default(),
                TireInfo::default(),
                TireInfo::default(),
            ],
            state_provider,
            settings_provider,
        }
    }
}

impl CarInfo {
    pub fn draw_car_info_page(&mut self, ui: &mut Ui) {
        ui.request_repaint_after(Duration::from_millis(16));

        if self.first_entry && !*self.state_provider.global_first_launch.read().unwrap() {
            self.draw_onboarding_dialog(ui);
        }

        if self.telemetry_provider.is_none() {
            if !self.first_entry {
                telemetry_not_found(ui);
            }
            return;
        }

        self.update();

        let rect = Rect::from_min_size(
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

        ui.painter()
            .rect_filled(rect, CornerRadius::same(24), Color32::from_rgb(22, 23, 28));

        let title_box = self.draw_title(ui, rect);

        let fuel_rect = Rect::from_min_size(
            title_box.max + vec2(-title_box.size().x, 16.0),
            vec2(rect.size().x - 32.0, 172.0),
        );
        self.draw_fuel_panel(ui, fuel_rect);

        let tires_rect = Rect::from_min_size(
            pos2(fuel_rect.min.x, fuel_rect.max.y + 16.0),
            vec2(fuel_rect.size().x / 2.0 - 8.0, 250.0),
        );
        self.draw_tires_panel(ui, tires_rect);

        let input_rect = Rect::from_min_size(
            pos2(
                fuel_rect.min.x + fuel_rect.size().x / 2.0 + 8.0,
                fuel_rect.max.y + 16.0,
            ),
            vec2(fuel_rect.size().x / 2.0 - 8.0, 250.0),
        );
        self.draw_input_panel(ui, input_rect);
    }
}
