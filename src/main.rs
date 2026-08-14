#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod graph_view;
mod telemetry;
mod telemetry_back;

use eframe::egui::{self, Button, Color32, Id, RichText};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_min_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Telemetry LMU",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<App>::default())
        }),
    )
}

struct App {
    active_page: Page,
    telemetry: telemetry::TelemetryPage,
}

enum Page {
    Telemetry,
    Info,
    Map,
    Profile,
    Settings,
}

// Default init
impl Default for App {
    fn default() -> Self {
        Self {
            active_page: Page::Telemetry,
            telemetry: telemetry::TelemetryPage::default(),
        }
    }
}

// Main GUI entrypoint
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.all_styles_mut(|style| style.visuals.panel_fill = Color32::from_rgb(15, 15, 15));

        self.e_sidebar(ui);

        egui::CentralPanel::default().show(ui, |ui| match self.active_page {
            Page::Telemetry => self.telemetry.main(ui),
            _ => todo!(),
        });
    }
}

impl App {
    fn e_sidebar(&mut self, ui: &mut egui::Ui) {
        let screen_width = ui.ctx().viewport_rect().width();
        let sidebar_width = screen_width * 0.15;

        egui::Panel::left(Id::new("e_sidebar"))
            .resizable(false)
            .exact_size(if sidebar_width > 200.0 {
                sidebar_width
            } else {
                200.0
            })
            .show(ui, |ui| {
                if ui.add(Button::new(RichText::new("Telemetry"))).clicked() {
                    self.active_page = Page::Telemetry
                }

                if ui.add(Button::new(RichText::new("Info"))).clicked() {
                    self.active_page = Page::Info
                }

                if ui.add(Button::new(RichText::new("Map"))).clicked() {
                    self.active_page = Page::Map
                }

                if ui.add(Button::new(RichText::new("Profile"))).clicked() {
                    self.active_page = Page::Profile
                }

                if ui.add(Button::new(RichText::new("Settings"))).clicked() {
                    self.active_page = Page::Settings
                }
            });
    }
}
