#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod backend;
mod frontend;
mod interface;

use eframe::egui::{self, Color32, FontData, FontDefinitions};
use egui_phosphor_icons::add_fonts;

use crate::frontend::frontend_main::{self, App};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

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

            cc.egui_ctx
                .all_styles_mut(|style| style.visuals.panel_fill = Color32::from_rgb(15, 15, 15));

            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "RacingSansOne".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../public/RacingSansOne-Regular.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Name("RacingSansOne".into()))
                .or_default()
                .insert(0, "RacingSansOne".to_owned());

            fonts.font_data.insert(
                "JetBrainsMono".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../public/JetBrainsMono-VariableFont_wght.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Name("JetBrainsMono".into()))
                .or_default()
                .insert(0, "JetBrainsMono".to_owned());

            fonts.font_data.insert(
                "RethinkSans".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../public/RethinkSans-Bold.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Name("RethinkSans".into()))
                .or_default()
                .insert(0, "RethinkSans".to_owned());

            fonts.font_data.insert(
                "BarlowCondensed".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../public/BarlowCondensed-SemiBold.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Name("BarlowCondensed".into()))
                .or_default()
                .insert(0, "BarlowCondensed".to_owned());

            fonts.font_data.insert(
                "DaysOne".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../public/DaysOne-Regular.ttf"
                ))),
            );
            fonts
                .families
                .entry(egui::FontFamily::Name("DaysOne".into()))
                .or_default()
                .insert(0, "DaysOne".to_owned());

            add_fonts(&mut fonts);

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(App::new(cc)))
        }),
    )
}
