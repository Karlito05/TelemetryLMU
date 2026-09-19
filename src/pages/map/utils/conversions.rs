use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map) fn to_screen(&self, rect: Rect, p: Vec2) -> Pos2 {
        rect.center() + (p * self.zoom) + self.offset
    }

    pub(in crate::pages::map) fn to_world(&self, rect: Rect, p: Pos2) -> Vec2 {
        (p - rect.center() - self.offset) / self.zoom
    }
}
