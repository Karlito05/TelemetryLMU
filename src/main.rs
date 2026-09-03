#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod backend;
mod frontend;
mod interface;
mod telemetry;

use std::sync::OnceLock;

use eframe::egui::*;
use egui_phosphor_icons::add_fonts;

use crate::frontend::frontend_main::App;

pub static TOKIO: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    let _ = TOKIO.set(rt);

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1920.0, 1080.0])
            .with_min_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Telemetry LMU",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            cc.egui_ctx
                .all_styles_mut(|style| style.visuals.panel_fill = Color32::from_rgb(15, 15, 15));
            cc.egui_ctx
                .all_styles_mut(|s| s.visuals.window_fill = Color32::from_rgb(22, 23, 28));

            let mut fonts = FontDefinitions::default();
            add_font(
                &mut fonts,
                "RacingSansOne",
                include_bytes!("../public/RacingSansOne-Regular.ttf"),
            );
            add_font(
                &mut fonts,
                "JetBrainsMono",
                include_bytes!("../public/JetBrainsMono-VariableFont_wght.ttf"),
            );
            add_font(
                &mut fonts,
                "RethinkSans",
                include_bytes!("../public/RethinkSans-Bold.ttf"),
            );
            add_font(
                &mut fonts,
                "BarlowCondensed",
                include_bytes!("../public/BarlowCondensed-SemiBold.ttf"),
            );
            add_font(
                &mut fonts,
                "DaysOne",
                include_bytes!("../public/DaysOne-Regular.ttf"),
            );

            //Icon fonts
            add_fonts(&mut fonts);

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(App::new(cc)))
        }),
    )
}

fn add_font(fonts: &mut FontDefinitions, name: &str, data: &'static [u8]) {
    fonts.font_data.insert(
        name.to_owned(),
        std::sync::Arc::new(FontData::from_static(data)),
    );
    fonts
        .families
        .entry(FontFamily::Name(name.into()))
        .or_default()
        .insert(0, name.to_owned());
}
