use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(super) fn draw_map_view(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        lines: &[(&Vec<Pos2>, Color32)],
    ) {
        ui.painter()
            .rect_filled(rect, CornerRadius::same(24), Color32::from_rgb(22, 23, 28));
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter().with_clip_rect(rect);

        if response.dragged() {
            self.offset += response.drag_delta();
        }

        if response.dragged_by(PointerButton::Secondary) {
            self.offset += response.drag_delta();
        }

        if response.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let mouse = response.hover_pos().unwrap_or(rect.center());
                let world = self.to_world(rect, mouse);

                self.zoom = (self.zoom * (1.0 + scroll * 0.01)).clamp(0.1, 10.0);

                self.offset = mouse - rect.center() - world * self.zoom;
            }
        }

        for line in lines {
            let points: Vec<Pos2> = line
                .0
                .iter()
                .map(|p| self.to_screen(rect, p.to_vec2()))
                .collect();
            painter.add(Shape::line(points, Stroke::new(0.5 * self.zoom, line.1)));
        }
    }
}
