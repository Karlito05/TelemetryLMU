use crate::pages::settings::settings_main::SettingsPage;
use eframe::egui::*;

impl SettingsPage {
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
        *self.settings_provider.pfp_bytes.write().unwrap() = Some(png.into_inner());

        // build gpu texture
        let size = [img.width() as usize, img.height() as usize];
        let color = ColorImage::from_rgba_unmultiplied(size, img.as_raw());
        *self.settings_provider.pfp_texture.write().unwrap() =
            Some(ctx.load_texture("pfp", color, TextureOptions::LINEAR));
    }

    /// Rebuild texture from persisted bytes (call once on startup).
    pub fn restore_pfp(&mut self, ctx: &Context) {
        if self.settings_provider.pfp_texture.read().unwrap().is_none() {
            let bytes = self.settings_provider.pfp_bytes.read().unwrap().clone();

            if let Some(bytes) = bytes {
                self.load_pfp(ctx, &bytes);
            }
        }
    }
}
