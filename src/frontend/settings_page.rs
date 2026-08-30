use eframe::egui::*;
use egui_phosphor_icons::icons;

use crate::frontend::{
    components::{button, input, switch},
    sidebar::Sidebar,
};
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub name: String,
    pub in_game_name: String,
    pub record_laps: bool,
    pub record_save_path: String,
    pfp_bytes: Option<Vec<u8>>,
    #[serde(skip)] // textures can't be serialized, recreate on load
    pub pfp_texture: Option<TextureHandle>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            in_game_name: "".to_owned(),
            record_laps: true,
            record_save_path: "".to_owned(),
            pfp_bytes: None,
            pfp_texture: None,
        }
    }
}

impl Settings {
    pub fn draw_settings_page(&mut self, ui: &mut Ui, sidebar: &mut Sidebar) {
        Window::new("Settings")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .title_bar(false)
            .show(ui, |ui| {
                ui.set_min_size(vec2(500.0, 600.0));
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Settings")
                            .size(32.0)
                            .color(Color32::WHITE)
                            .family(FontFamily::Name("RacingSansOne".into())),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .add(
                                Button::new(icons::X.regular().size(32.0))
                                    .fill(Color32::TRANSPARENT)
                                    .stroke(Stroke::NONE),
                            )
                            .clicked()
                        {
                            sidebar.settings_open = false
                        }
                    });
                });

                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Name").size(20.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        input(
                            ui,
                            vec2(200.0, 32.0),
                            CornerRadius::same(8),
                            Stroke::new(1.0, Color32::from_gray(127)),
                            Color32::from_white_alpha(17),
                            &mut self.name,
                            FontSelection::FontId(FontId::proportional(16.0)),
                            32,
                        )
                    })
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("In Game Name").size(20.0));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        input(
                            ui,
                            vec2(200.0, 32.0),
                            CornerRadius::same(8),
                            Stroke::new(1.0, Color32::from_gray(127)),
                            Color32::from_white_alpha(17),
                            &mut self.in_game_name,
                            FontSelection::FontId(FontId::proportional(16.0)),
                            32,
                        )
                    })
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Record Laps").size(20.0));

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        self.record_laps = switch(
                            ui,
                            vec2(48.0, 24.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            Color32::from_rgb(19, 141, 241),
                            self.record_laps,
                        )
                    })
                });
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Lap Save Path").size(20.0));
                        ui.label(RichText::new(&self.record_save_path).size(12.0));
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        #[expect(clippy::collapsible_if)]
                        if button(
                            ui,
                            vec2(140.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Browse",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Select folder")
                                .set_directory(self.record_save_path.clone())
                                .add_filter("All files", &["*"])
                                .set_can_create_directories(true)
                                .pick_folder()
                            {
                                self.record_save_path = path.display().to_string()
                            }
                        }
                    })
                });

                ui.horizontal(|ui| {
                    // preview image, or a placeholder circle
                    // match &self.pfp_texture {
                    //     Some(tex) => {
                    //         ui.add(Image::new(tex).max_size(vec2(64.0, 64.0)));
                    //     }
                    //     None => {
                    //         let (rect, _) =
                    //             ui.allocate_exact_size(vec2(64.0, 64.0), Sense::hover());
                    //         ui.painter().circle_filled(
                    //             rect.center(),
                    //             32.0,
                    //             Color32::from_white_alpha(25),
                    //         );
                    //         // ui.painter().text(
                    //         //     rect.center(),
                    //         //     Align2::CENTER_CENTER,
                    //         //     icons::USER.regular().size(32.0),
                    //         //     FontId::proportional(32.0),
                    //         //     Color32::from_gray(127),
                    //         // );
                    //     }
                    // }
                    //
                    ui.label(RichText::new("Change Profile Picture").size(20.0));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        #[expect(clippy::collapsible_if)]
                        if button(
                            ui,
                            vec2(140.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Browse",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_title("Select a file")
                                .add_filter("Images", &["png", "jpg", "jpeg"])
                                .pick_file()
                            {
                                if let Ok(bytes) = std::fs::read(&path) {
                                    self.load_pfp(ui.ctx(), &bytes);
                                }
                            }
                        }
                    });
                });
            });
    }
    pub fn load_pfp(&mut self, ctx: &Context, bytes: &[u8]) {
        // decode + downscale to keep the save file small
        let img = match image::load_from_memory(bytes)
            .map(|img| img.resize(256, 256, image::imageops::FilterType::Lanczos3))
        {
            Ok(img) => img.to_rgba8(),
            Err(e) => {
                eprintln!("Failed to load pfp: {e}");
                return;
            }
        };

        // encode as png for compact storage
        let mut png = std::io::Cursor::new(Vec::new());
        if img.write_to(&mut png, image::ImageFormat::Png).is_err() {
            eprintln!("Failed to encode pfp");
            return;
        }
        self.pfp_bytes = Some(png.into_inner());

        // build gpu texture
        let size = [img.width() as usize, img.height() as usize];
        let color = ColorImage::from_rgba_unmultiplied(size, img.as_raw());
        self.pfp_texture = Some(ctx.load_texture("pfp", color, TextureOptions::LINEAR));
    }

    /// Rebuild texture from persisted bytes (call once on startup).
    pub fn restore_pfp(&mut self, ctx: &Context) {
        if self.pfp_texture.is_none() {
            if let Some(bytes) = self.pfp_bytes.clone() {
                self.load_pfp(ctx, &bytes);
            }
        }
    }
}
