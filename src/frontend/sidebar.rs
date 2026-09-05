use std::sync::{Arc, Mutex};

use eframe::egui::*;
use egui_phosphor_icons::{Icon, icons};

use crate::frontend::{
    frontend_main::{Page, SettingsProvider},
    settings_page::SettingsPage,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Sidebar {
    pub open: bool,
    pub active_page: Page,
    #[serde(skip)]
    pub settings_open: bool,
    #[serde(skip)]
    settings_provider: Arc<SettingsProvider>,
}

impl Sidebar {
    pub fn new(settings_provider: Arc<SettingsProvider>) -> Self {
        Self {
            open: false,
            active_page: Page::Telemetry,
            settings_open: false,
            settings_provider,
        }
    }
}

impl Sidebar {
    pub fn draw_sidebar(&mut self, ui: &mut Ui) {
        // let screen_width = ui.ctx().viewport_rect().width();
        let sidebar_width = 300.0;
        let sidebar_height = ui.ctx().viewport_rect().height();

        if !self.open {
            return;
        }

        Panel::left(Id::new("e_sidebar"))
            .resizable(false)
            .exact_size(sidebar_width)
            .show_separator_line(false)
            .show(ui, |ui| {
                let sidebar_back_rect = Rect::from_min_size(
                    pos2(16.0, 16.0),
                    vec2(sidebar_width - 32.0, sidebar_height - 32.0),
                );
                ui.painter().rect_filled(
                    sidebar_back_rect,
                    CornerRadius::same(24),
                    Color32::from_rgb(23, 23, 28),
                );

                ui.put(sidebar_back_rect, |ui: &mut Ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add_space(16.0);

                        self.draw_top(ui);

                        ui.add_space(32.0);

                        self.draw_buttons(ui);

                        ui.add_space(ui.available_height() - 70.0);

                        if self.draw_profile(ui).clicked() {
                            self.settings_open = true;
                        }
                    })
                    .response
                })
            });
    }

    fn draw_top(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 16.0;

            ui.add_space(16.0);

            ui.add(
                Image::new(include_image!("../../public/Logo.svg"))
                    .fit_to_exact_size(vec2(48.0, 48.0)),
            );
            ui.add(
                Label::new(
                    RichText::new("Telemetry LMU")
                        .family(FontFamily::Name("RacingSansOne".into()))
                        .size(24.0)
                        .color(Color32::WHITE),
                )
                .selectable(false),
            );
        });
    }

    fn draw_buttons(&mut self, ui: &mut Ui) {
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
                        self.active_page == Page::Telemetry,
                    )
                    .clicked()
                {
                    self.active_page = Page::Telemetry;
                };

                if self
                    .draw_button(
                        ui,
                        "Car Info".to_owned(),
                        icons::INFO,
                        self.active_page == Page::Info,
                    )
                    .clicked()
                {
                    self.active_page = Page::Info;
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
                        self.active_page == Page::Map,
                    )
                    .clicked()
                {
                    self.active_page = Page::Map;
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

    fn draw_profile(&self, ui: &mut Ui) -> Response {
        let desired_size = vec2(ui.available_width() - 16.0, 64.0);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // Background
        let bg_color = if response.hovered() {
            Color32::from_rgb(45, 46, 50)
        } else {
            Color32::from_rgb(22, 23, 28)
        };

        ui.painter()
            .rect_filled(rect, CornerRadius::same(16), bg_color);

        // Profile image
        let image_rect = Rect::from_min_size(rect.min + vec2(8.0, 8.0), vec2(48.0, 48.0));

        if let Some(img) = &*self.settings_provider.pfp_texture.read().unwrap() {
            ui.put(
                image_rect,
                Image::new(img)
                    .fit_to_exact_size(vec2(48.0, 48.0))
                    .corner_radius(CornerRadius::same(8)),
            );
        }

        // Name
        ui.painter().text(
            pos2(rect.min.x + 80.0, rect.center().y),
            Align2::LEFT_CENTER,
            self.settings_provider.name.read().unwrap().clone(),
            FontId::proportional(24.0),
            Color32::WHITE,
        );

        // More icon
        ui.put(
            Rect::from_min_size(
                pos2(rect.max.x - 40.0, rect.center().y - 16.0),
                vec2(24.0, 32.0),
            ),
            Label::new(
                icons::DOTS_THREE_VERTICAL
                    .bold()
                    .size(24.0)
                    .color(Color32::WHITE),
            )
            .selectable(false),
        );

        response
    }
}
