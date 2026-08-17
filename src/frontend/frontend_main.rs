use eframe::egui::{self, Button, Color32, FontData, FontDefinitions, Id, RichText};

use crate::frontend::{sidebar::Sidebar, telemetry_page};

pub struct App {
    sidebar: Sidebar,
    telemetry: telemetry_page::TelemetryPage,
}

#[derive(Debug, PartialEq)]
pub enum Page {
    Telemetry,
    Info,
    Map,
    Settings,
}

// Default init
impl Default for App {
    fn default() -> Self {
        Self {
            sidebar: Sidebar::default(),
            telemetry: telemetry_page::TelemetryPage::default(),
        }
    }
}

// Main GUI entrypoint
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.sidebar.draw_sidebar(ui);

        egui::CentralPanel::default().show(ui, |ui| match self.sidebar.active_page {
            Page::Telemetry => self.telemetry.draw_telemetry_page(ui),
            _ => todo!(),
        });
    }
}
