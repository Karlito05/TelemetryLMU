use std::sync::RwLock;

use crate::pages::Page;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub struct StateProvider {
    pub global_first_launch: RwLock<bool>,
    #[serde(skip)]
    pub page: RwLock<Page>,
    #[serde(skip)]
    pub sidebar_open: RwLock<bool>,
    #[serde(skip)]
    pub settings_open: RwLock<bool>,
}
impl Default for StateProvider {
    fn default() -> Self {
        Self {
            global_first_launch: RwLock::new(true),
            page: RwLock::new(Page::Telemetry),
            sidebar_open: RwLock::new(true),
            settings_open: RwLock::new(false),
        }
    }
}
