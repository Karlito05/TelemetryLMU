use eframe::egui::*;

use crate::frontend::sidebar::Sidebar;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MapPage {
    zoom: f32,
    offset: Vec2,
}

impl Default for MapPage {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl MapPage {
    pub fn draw_map_page(&mut self, ui: &mut Ui, sidebar: &Sidebar) {
        let map_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
                ui.available_height() - 16.0,
            ),
        );

        ui.painter().rect_filled(
            map_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        self.draw_map(
            ui,
            map_rect,
            &[
                vec![pos2(0.0, 0.0), pos2(20.0, 15.0)],
                vec![pos2(12.3, 23.0), pos2(33.0, 45.0)],
            ],
        );

        let controls_rect = Rect::from_min_max(
            pos2(map_rect.min.x + 8.0, map_rect.max.y - 200.0),
            map_rect.max - vec2(8.0, 8.0),
        );

        ui.painter()
            .rect_filled(controls_rect, 16, Color32::from_white_alpha(17));
    }

    fn to_screen(&self, rect: Rect, p: Vec2) -> Pos2 {
        rect.center() + (p * self.zoom) + self.offset
    }

    fn to_world(&self, rect: Rect, p: Pos2) -> Vec2 {
        (p - rect.center() - self.offset) / self.zoom
    }

    pub fn draw_map(&mut self, ui: &mut Ui, rect: Rect, lines: &[Vec<Pos2>]) {
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
                self.zoom = (self.zoom * (1.0 + scroll * 0.01)).clamp(0.1, 10.0);

                let mouse = response.hover_pos().unwrap_or(rect.center());
                let world = self.to_world(rect, mouse);
                self.offset = mouse - rect.center() - world * self.zoom;
            }
        }

        for line in lines {
            let points: Vec<Pos2> = line
                .iter()
                .map(|p| self.to_screen(rect, p.to_vec2()))
                .collect();
            painter.add(Shape::line(
                points,
                Stroke::new(0.5 * self.zoom, Color32::from_rgb(19, 141, 241)),
            ));
        }
    }
}
