use crate::frontend::components::{DropdownItem, GraphInfo, button, graph};
use crate::frontend::sidebar::Sidebar;
use crate::telemetry_back::Telemetry;
use crate::{frontend::components::dropdown, graph_view::GraphViewDataType};
use eframe::egui::Direction::RightToLeft;
use eframe::egui::*;
use egui_phosphor_icons::icons;

pub struct TelemetryPage {
    telemetry: Telemetry,
    cur_driver: (String, i32),
    cur_layout_index: usize,
    layouts: Vec<LayoutInfo>,
}

struct LayoutInfo {
    name: String,
    graphs: Vec<GraphInfo>,
}

impl Default for TelemetryPage {
    fn default() -> Self {
        Self {
            telemetry: Telemetry::new("/dev/shm/LMU_Data"),
            cur_driver: ("".to_owned(), 0),
            cur_layout_index: 0,
            layouts: vec![
                LayoutInfo {
                    name: "Main test".to_owned(),
                    graphs: vec![
                        GraphInfo {
                            cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                            ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                            max_val: 1.0,
                            color: Color32::from_rgb(20, 10, 200),
                            n_gridlines: 3,
                            unit: "%".to_owned(),
                            size_percent: 0.5,
                            graph_type: GraphViewDataType::from_string("rpm", 0),
                        },
                        GraphInfo {
                            cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                            ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                            max_val: 1.0,
                            color: Color32::from_rgb(20, 200, 20),
                            n_gridlines: 3,
                            unit: "%".to_owned(),
                            size_percent: 0.5,
                            graph_type: GraphViewDataType::from_string("speed", 0),
                        },
                    ],
                },
                LayoutInfo {
                    name: "Main test 2".to_owned(),
                    graphs: vec![
                        GraphInfo {
                            cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                            ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                            max_val: 1.0,
                            color: Color32::from_rgb(20, 150, 200),
                            n_gridlines: 3,
                            unit: "%".to_owned(),
                            size_percent: 0.5,
                            graph_type: GraphViewDataType::from_string("rpm", 0),
                        },
                        GraphInfo {
                            cur_lap: vec![vec2(0.0, 0.4), vec2(0.4, 0.2)],
                            ref_lap: vec![vec2(0.0, 1.0), vec2(0.6, 0.3)],
                            max_val: 1.0,
                            color: Color32::from_rgb(200, 0, 20),
                            n_gridlines: 3,
                            unit: "%".to_owned(),
                            size_percent: 0.5,
                            graph_type: GraphViewDataType::from_string("speed", 0),
                        },
                    ],
                },
            ],
        }
    }
}

impl TelemetryPage {
    pub fn draw_telemetry_page(&mut self, ui: &mut Ui, sidebar: &mut Sidebar) {
        let top_bar_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 0.0 } else { 16.0 },
                48.0,
            ),
        );

        self.draw_top_bar(ui, top_bar_rect, sidebar);

        let graphs_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 80.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
                ui.available_height() - 24.0,
            ),
        );

        self.draw_graphs(ui, graphs_rect);
    }

    fn draw_top_bar(&mut self, ui: &mut Ui, top_bar_rect: Rect, sidebar: &mut Sidebar) {
        ui.painter().rect_filled(
            top_bar_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        ui.put(top_bar_rect, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);

                self.draw_sidebar_button(ui, sidebar);

                ui.separator();

                self.draw_driver_select(ui);

                ui.separator();

                self.draw_layout_select(ui);

                ui.separator();

                self.draw_reference_controls(ui);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(4.0);

                    self.draw_edit_layout_button(ui);
                })
            })
            .response
        });
    }

    fn draw_sidebar_button(&self, ui: &mut Ui, sidebar: &mut Sidebar) {
        let (sidebar_icon_rect, response) = ui.allocate_exact_size(
            vec2(ui.available_height() - 8.0, ui.available_height() - 8.0),
            Sense::click(),
        );
        if response.hovered() {
            ui.painter().rect_filled(
                sidebar_icon_rect,
                CornerRadius::same(40),
                Color32::from_white_alpha(25),
            );
        }
        if response.clicked() {
            ui.painter().rect_filled(
                sidebar_icon_rect,
                CornerRadius::same(40),
                Color32::from_white_alpha(25),
            );
            sidebar.open = !sidebar.open;
        }

        ui.put(
            sidebar_icon_rect,
            Label::new(
                icons::SIDEBAR_SIMPLE
                    .regular()
                    .size(32.0)
                    .color(Color32::from_rgb(19, 141, 241)),
            )
            .selectable(false),
        );
    }

    fn draw_driver_select(&mut self, ui: &mut Ui) {
        ui.add(
            Label::new(RichText::new("Driver:").size(16.0).color(Color32::WHITE)).selectable(false),
        );

        ui.add_space(2.0);

        dropdown(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            &mut self.cur_driver,
            "Select a driver",
            FontId {
                size: 14.0,
                family: FontFamily::Proportional,
            },
            self.telemetry
                .get_drivers()
                .iter()
                .map(|driver| DropdownItem {
                    value: driver.clone(),
                    display_value: driver.0.clone(),
                })
                .collect(),
        );
    }

    fn draw_layout_select(&mut self, ui: &mut Ui) {
        ui.add(
            Label::new(RichText::new("Layout:").size(16.0).color(Color32::WHITE)).selectable(false),
        );

        ui.add_space(2.0);

        dropdown(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            &mut self.cur_layout_index,
            "",
            FontId {
                size: 14.0,
                family: FontFamily::Proportional,
            },
            self.layouts
                .iter()
                .enumerate()
                .map(|(i, l)| DropdownItem {
                    value: i,
                    display_value: l.name.clone(),
                })
                .collect(),
        );
    }

    fn draw_reference_controls(&mut self, ui: &mut Ui) {
        ui.add(
            Label::new(RichText::new("Reference:").size(16.0).color(Color32::WHITE))
                .selectable(false),
        );

        if button(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            "Select ref from file",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            // TODO: Logic
        }
        if button(
            ui,
            vec2(140.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgba_unmultiplied(255, 0, 0, 127),
            "Clear",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            // TODO: Logic
        }
    }

    fn draw_edit_layout_button(&self, ui: &mut Ui) {
        let (sidebar_icon_rect, response) = ui.allocate_exact_size(
            vec2(ui.available_height() - 8.0, ui.available_height() - 8.0),
            Sense::click(),
        );
        if response.hovered() {
            ui.painter().rect_filled(
                sidebar_icon_rect,
                CornerRadius::same(40),
                Color32::from_white_alpha(25),
            );
        }
        if response.clicked() {
            ui.painter().rect_filled(
                sidebar_icon_rect,
                CornerRadius::same(40),
                Color32::from_white_alpha(25),
            );
            // TODO: Logic
        }

        ui.put(
            sidebar_icon_rect,
            Label::new(
                icons::PENCIL
                    .regular()
                    .size(32.0)
                    .color(Color32::from_rgb(19, 141, 241)),
            )
            .selectable(false),
        );
    }

    fn draw_graphs(&self, ui: &mut Ui, graphs_rect: Rect) {
        ui.put(graphs_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                let margins = 64.0;
                for (i, graph_info) in self.layouts[self.cur_layout_index]
                    .graphs
                    .iter()
                    .enumerate()
                {
                    if i == 0 {
                        graph(
                            ui,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius {
                                nw: 24,
                                ne: 24,
                                sw: 0,
                                se: 0,
                            },
                            margins,
                        );
                        continue;
                    }

                    if i == self.layouts[self.cur_layout_index].graphs.len() - 1 {
                        graph(
                            ui,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius {
                                nw: 0,
                                ne: 0,
                                sw: 24,
                                se: 24,
                            },
                            margins,
                        );
                        continue;
                    }

                    if self.layouts[self.cur_layout_index].graphs.len() == 1 {
                        graph(
                            ui,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius::same(24),
                            margins,
                        );
                        continue;
                    }

                    graph(
                        ui,
                        graph_info,
                        vec2(
                            graphs_rect.width(),
                            graph_info.size_percent * graphs_rect.height(),
                        ),
                        CornerRadius::same(0),
                        margins,
                    );
                }
            })
            .response
        });
    }
}
