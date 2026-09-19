use crate::{components::sidebar::sidebar_main::Sidebar, pages::Page};
use eframe::egui::*;
use egui_phosphor_icons::{Icon, icons};

impl Sidebar {
    pub fn draw_buttons(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.vertical(|ui| {
                ui.add(Label::new(
                    RichText::new("Live Analysis")
                        .family(FontFamily::Name("DaysOne".into()))
                        .size(12.0)
                        .color(Color32::from_white_alpha(127)),
                ));

                ui.add_space(4.0);

                if self
                    .draw_button(
                        ui,
                        "Telemetry".to_owned(),
                        icons::CHART_LINE,
                        *self.state_provider.page.read().unwrap() == Page::Telemetry,
                    )
                    .clicked()
                {
                    *self.state_provider.page.write().unwrap() = Page::Telemetry;
                };

                if self
                    .draw_button(
                        ui,
                        "Car Info".to_owned(),
                        icons::INFO,
                        *self.state_provider.page.read().unwrap() == Page::Info,
                    )
                    .clicked()
                {
                    *self.state_provider.page.write().unwrap() = Page::Info;
                };

                ui.add_space(32.0);

                ui.add(Label::new(
                    RichText::new("Reflect")
                        .family(FontFamily::Name("DaysOne".into()))
                        .size(12.0)
                        .color(Color32::from_white_alpha(127)),
                ));

                ui.add_space(4.0);

                if self
                    .draw_button(
                        ui,
                        "Map".to_owned(),
                        icons::MAP_TRIFOLD,
                        *self.state_provider.page.read().unwrap() == Page::Map,
                    )
                    .clicked()
                {
                    *self.state_provider.page.write().unwrap() = Page::Map;
                };
            });
        });
    }

    fn draw_button(&self, ui: &mut Ui, text: String, icon: Icon, active: bool) -> Response {
        let desired_size = vec2(234.0, 36.0);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        let bg_color = if active || response.hovered() {
            Color32::from_rgb(22, 23, 28).blend(Color32::from_white_alpha(255 / 20)) //5% white
        } else {
            Color32::from_rgb(22, 23, 28)
        };

        ui.painter()
            .rect_filled(rect, CornerRadius::same(8), bg_color);

        ui.put(rect, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.add_space(2.0);
                ui.label(
                    icon.regular()
                        .size(24.0)
                        .color(Color32::from_rgb(19, 141, 241)),
                );
                ui.label(RichText::new(text).size(16.0).color(Color32::WHITE));
            })
            .response
        });

        response
    }
}
