use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls::input_row) fn draw_delta(
        &mut self,
        ui: &mut Ui,
        row_rect: &Rect,
    ) {
        if let Some(delta) = self.get_time_delta() {
            if delta < 0.0 {
                ui.painter().text(
                    row_rect.center(),
                    Align2::CENTER_CENTER,
                    format!("{:.3}", delta),
                    FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                    Color32::GREEN,
                );
            } else {
                ui.painter().text(
                    row_rect.center(),
                    Align2::CENTER_CENTER,
                    format!("+{:.3}", delta),
                    FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                    Color32::RED,
                );
            }
        } else {
            ui.painter().text(
                row_rect.center(),
                Align2::CENTER_CENTER,
                "N/A",
                FontId::new(32.0, FontFamily::Name("JetBrainsMono".into())),
                Color32::RED,
            );
        }
    }
}
