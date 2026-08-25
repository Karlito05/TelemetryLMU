use eframe::egui::*;

use crate::frontend::{settings::Settings, sidebar::Sidebar};

pub struct CarInfo {}

impl Default for CarInfo {
    fn default() -> Self {
        Self {}
    }
}

impl CarInfo {
    pub fn draw_car_info_page(
        &mut self,
        ui: &mut Ui,
        sidebar: &mut Sidebar,
        settings: &mut Settings,
    ) {
    }
}
