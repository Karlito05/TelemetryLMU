use std::{fmt, sync::RwLock};

use eframe::egui::TextureHandle;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct SettingsProvider {
    pub name: RwLock<String>,
    pub in_game_name: RwLock<String>,
    pub record_laps: RwLock<bool>,
    pub record_save_path: RwLock<String>,
    pub pfp_bytes: RwLock<Option<Vec<u8>>>,
    #[serde(skip)] // textures can't be serialized, recreate on load
    pub pfp_texture: RwLock<Option<TextureHandle>>,
    pub log_all_cars: RwLock<bool>,
}

impl fmt::Debug for SettingsProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsProvider")
            .field("name", &self.name)
            .field("in_game_name", &self.in_game_name)
            .field("record_laps", &self.record_laps)
            .field("record_save_path", &self.record_save_path)
            .field("pfp_bytes", &self.pfp_bytes)
            .field("log_all_cars", &self.log_all_cars)
            .finish()
    }
}
