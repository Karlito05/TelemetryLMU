use crate::backend::telemetry::GraphViewDataType;
use crate::frontend::components::{
    DropdownItem, GraphChange, GraphInfo, button, graph, graph_edit,
};
use crate::frontend::components::{dropdown, telemetry_not_found};
use crate::frontend::sidebar::Sidebar;
use crate::interface::Telemetry;
use eframe::egui::*;
use egui_phosphor_icons::icons;

pub struct TelemetryPage {
    telemetry: Telemetry,
    cur_driver: (String, i32),
    in_layout_edit_mode: bool,
    cur_layout_index: usize,
    layouts: Vec<LayoutInfo>,
    edit_mode_context: EditModeContext,
    show_delete_layout_dialog: bool,
    show_add_limit_dialog: bool,
    save_as_dialog_info: SaveAsDialogInfo,
    cur_lap: i32,
}

#[derive(Clone, Debug)]
struct EditModeContext {
    layout: EditLayoutInfo,
    #[expect(unused)]
    started_edtiting: bool,
}

#[derive(Clone, Debug)]
struct SaveAsDialogInfo {
    name: String,
    show: bool,
}

#[derive(Clone, Debug)]
struct EditLayoutInfo {
    index: usize,
    graphs: Vec<GraphInfo>,
}

#[derive(Clone, Debug)]
struct LayoutInfo {
    name: String,
    graphs: Vec<GraphInfo>,
}

impl Default for TelemetryPage {
    fn default() -> Self {
        Self {
            telemetry: Telemetry::new("/dev/shm/LMU_Data"),
            cur_driver: ("".to_owned(), 0),
            in_layout_edit_mode: false,
            show_delete_layout_dialog: false,
            show_add_limit_dialog: false,
            cur_layout_index: 0,
            cur_lap: 0,
            save_as_dialog_info: SaveAsDialogInfo {
                name: "".to_owned(),
                show: false,
            },
            edit_mode_context: EditModeContext {
                layout: EditLayoutInfo {
                    index: 0,
                    graphs: vec![],
                },
                started_edtiting: false,
            },
            layouts: vec![LayoutInfo {
                name: "Main".to_owned(),
                graphs: vec![GraphInfo {
                    show_ref: false,
                    cur_lap: vec![],
                    ref_lap: vec![],
                    color: Color32::WHITE,
                    n_gridlines: 3,
                    size_percent: 1.0,
                    graph_type: GraphViewDataType::Rpm(0),
                }],
            }],
        }
    }
}

impl TelemetryPage {
    pub fn draw_telemetry_page(&mut self, ui: &mut Ui, sidebar: &mut Sidebar) {
        if !self.telemetry.full_mode {
            telemetry_not_found(ui);
            return;
        }
        self.process_telemetry_updates(
            self.layouts[self.cur_layout_index]
                .graphs
                .iter()
                .map(|g| g.graph_type)
                .collect(),
        );
        ui.request_repaint_after(std::time::Duration::from_millis(16));
        if !self.in_layout_edit_mode {
            self.draw_normal_mode(ui, sidebar);
        } else {
            self.draw_edit_mode(ui, sidebar);
        }
    }

    fn process_telemetry_updates(&mut self, graph_data_types: Vec<GraphViewDataType>) {
        let t = self.telemetry.update_telemetry().unwrap();

        if self.cur_lap != graph_data_types[0].get_lap(&t) {
            if graph_data_types[0].is_last_best(&t) {
                graph_data_types.iter().enumerate().for_each(|(i, _)| {
                    self.layouts[self.cur_layout_index].graphs[i].ref_lap =
                        self.layouts[self.cur_layout_index].graphs[i]
                            .cur_lap
                            .clone();
                });
            }
            graph_data_types.iter().enumerate().for_each(|(i, _)| {
                self.layouts[self.cur_layout_index].graphs[i].cur_lap = vec![];
            });
            self.cur_lap = graph_data_types[0].get_lap(&t);
        }

        graph_data_types
            .iter()
            .enumerate()
            .for_each(|(i, graph_data_type)| {
                if !self.layouts[self.cur_layout_index].graphs[i]
                    .cur_lap
                    .is_empty()
                    || graph_data_type.get_normalized_distance(&t) < 0.9
                {
                    self.layouts[self.cur_layout_index].graphs[i]
                        .cur_lap
                        .push(vec2(
                            graph_data_type.get_normalized_distance(&t) as f32,
                            graph_data_type.get_normalized_values(&t) as f32,
                        ));
                }
            });
    }

    fn draw_edit_mode(&mut self, ui: &mut Ui, sidebar: &mut Sidebar) {
        let top_bar_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 0.0 } else { 16.0 },
                48.0,
            ),
        );
        self.draw_top_bar_edit(ui, top_bar_rect, sidebar);

        let graphs_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 80.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
                ui.available_height() - 20.0,
            ),
        );
        self.draw_graphs_edit(ui, graphs_rect);
    }

    fn draw_normal_mode(&mut self, ui: &mut Ui, sidebar: &mut Sidebar) {
        let top_bar_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 16.0),
            vec2(
                ui.available_width() - if sidebar.open { 0.0 } else { 16.0 },
                48.0,
            ),
        );

        self.draw_top_bar_normal(ui, top_bar_rect, sidebar);

        let graphs_rect = Rect::from_min_size(
            pos2(if sidebar.open { 300.0 } else { 16.0 }, 80.0),
            vec2(
                ui.available_width() - if sidebar.open { 8.0 } else { 16.0 },
                ui.available_height() - 20.0,
            ),
        );

        self.draw_graphs_normal(ui, graphs_rect);
    }

    fn draw_top_bar_edit(&mut self, ui: &mut Ui, top_bar_rect: Rect, sidebar: &mut Sidebar) {
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

                self.draw_editing_layout_label(ui);

                ui.separator();

                self.draw_save_button(ui);

                ui.separator();

                self.draw_save_as_button(ui);

                ui.separator();

                self.draw_discard_button(ui);

                ui.separator();

                self.draw_add_graph_button(ui);

                ui.separator();

                self.draw_delete_button(ui);
            })
            .response
        });
    }

    fn draw_editing_layout_label(&mut self, ui: &mut Ui) {
        ui.add(
            Label::new(
                RichText::new(format!(
                    "Editing: {}",
                    self.layouts[self.edit_mode_context.layout.index].name
                ))
                .size(16.0)
                .color(Color32::WHITE),
            )
            .selectable(false),
        );
    }

    fn draw_save_button(&mut self, ui: &mut Ui) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgb(19, 141, 241),
            "Save",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.layouts[self.edit_mode_context.layout.index].graphs =
                self.edit_mode_context.layout.graphs.clone();
            for graph in &mut self.layouts[self.edit_mode_context.layout.index].graphs {
                graph.cur_lap = vec![];
                graph.ref_lap = vec![];
            }
            self.edit_mode_context = EditModeContext {
                layout: EditLayoutInfo {
                    graphs: vec![],
                    index: 0,
                },
                started_edtiting: false,
            };
            self.in_layout_edit_mode = false;
        }
    }

    fn draw_save_as_button(&mut self, ui: &mut Ui) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgb(19, 141, 241),
            "Save as",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.save_as_dialog_info.show = true
        }
        if self.save_as_dialog_info.show {
            Window::new("Name?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("Enter the name for your new layout: ");

                    ui.text_edit_singleline(&mut self.save_as_dialog_info.name);

                    ui.horizontal(|ui: &mut Ui| {
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_rgb(19, 141, 241),
                            "Confirm",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.layouts.push(LayoutInfo {
                                graphs: self.edit_mode_context.layout.graphs.clone(),
                                name: self.save_as_dialog_info.name.clone(),
                            });
                            self.cur_layout_index = self.layouts.len() - 1;
                            self.edit_mode_context = EditModeContext {
                                layout: EditLayoutInfo {
                                    graphs: vec![],
                                    index: 0,
                                },
                                started_edtiting: false,
                            };
                            self.save_as_dialog_info.show = false;
                            self.in_layout_edit_mode = false;
                        }
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Cancel",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.save_as_dialog_info.show = false;
                        }
                    })
                });
        }
    }

    fn draw_discard_button(&mut self, ui: &mut Ui) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            "Discard",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.edit_mode_context = EditModeContext {
                layout: EditLayoutInfo {
                    graphs: vec![],
                    index: 0,
                },
                started_edtiting: false,
            };
            self.in_layout_edit_mode = false;
        }
    }

    fn draw_add_graph_button(&mut self, ui: &mut Ui) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_white_alpha(25),
            "Add",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            if self.edit_mode_context.layout.graphs.len() < 10 {
                self.edit_mode_context.layout.graphs.push(GraphInfo {
                    color: Color32::WHITE,
                    show_ref: true,
                    n_gridlines: 3,
                    size_percent: 0.0,
                    graph_type: GraphViewDataType::Rpm(self.cur_driver.1 as usize),
                    cur_lap: vec![],
                    ref_lap: vec![],
                });

                let new_num_graphs = self.edit_mode_context.layout.graphs.len();

                self.edit_mode_context
                    .layout
                    .graphs
                    .iter_mut()
                    .for_each(|g| {
                        g.size_percent = 1.0 / new_num_graphs as f32;
                    });
            } else {
                self.show_add_limit_dialog = true
            }
        }
        if self.show_add_limit_dialog {
            Window::new("Couldn't Add Graph")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("You can't have more than 10 graphs!");

                    ui.horizontal(|ui| {
                        if button(
                            ui,
                            vec2(64.0, 32.0),
                            CornerRadius::same(8),
                            Color32::from_white_alpha(25),
                            "Ok",
                            FontId::new(16.0, FontFamily::Proportional),
                            Color32::WHITE,
                        )
                        .clicked()
                        {
                            self.show_add_limit_dialog = false;
                        }
                    });
                });
        }
    }

    fn draw_delete_button(&mut self, ui: &mut Ui) {
        if button(
            ui,
            vec2(64.0, 32.0),
            CornerRadius::same(8),
            Color32::from_rgb(255, 0, 0),
            "Delete",
            FontId::new(14.0, FontFamily::Proportional),
            Color32::WHITE,
        )
        .clicked()
        {
            self.show_delete_layout_dialog = true;
        }

        if self.show_delete_layout_dialog {
            if self.layouts.len() >= 2 {
                Window::new("Delete Layout?")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.label("Are you sure you want to delete this layout?");

                        ui.horizontal(|ui| {
                            if button(
                                ui,
                                vec2(64.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_rgb(255, 0, 0),
                                "Delete",
                                FontId::new(16.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                self.layouts.remove(self.edit_mode_context.layout.index);
                                self.cur_layout_index = 0;
                                self.show_delete_layout_dialog = false;
                                self.edit_mode_context = EditModeContext {
                                    layout: EditLayoutInfo {
                                        graphs: vec![],
                                        index: 0,
                                    },
                                    started_edtiting: false,
                                };
                                self.in_layout_edit_mode = false;
                            }
                            if button(
                                ui,
                                vec2(64.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_white_alpha(25),
                                "Cancel",
                                FontId::new(16.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                self.show_delete_layout_dialog = false;
                            }
                        });
                    });
            } else {
                Window::new("Couldn't Delete Layout")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                    .show(ui.ctx(), |ui| {
                        ui.label("You must have at least 1 layout!");

                        ui.horizontal(|ui| {
                            if button(
                                ui,
                                vec2(64.0, 32.0),
                                CornerRadius::same(8),
                                Color32::from_white_alpha(25),
                                "Ok",
                                FontId::new(16.0, FontFamily::Proportional),
                                Color32::WHITE,
                            )
                            .clicked()
                            {
                                self.show_delete_layout_dialog = false;
                            }
                        });
                    });
            }
        }
    }

    fn draw_top_bar_normal(&mut self, ui: &mut Ui, top_bar_rect: Rect, sidebar: &mut Sidebar) {
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

        if dropdown(
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
        ) {
            for layout in &mut self.layouts {
                for graph in &mut layout.graphs {
                    graph.graph_type = GraphViewDataType::from_string(
                        &graph.graph_type.to_string(),
                        self.cur_driver.1 as usize,
                    );
                    graph.cur_lap = vec![];
                    graph.ref_lap = vec![];
                }
            }
        }
    }

    fn draw_layout_select(&mut self, ui: &mut Ui) {
        ui.add(
            Label::new(RichText::new("Layout:").size(16.0).color(Color32::WHITE)).selectable(false),
        );

        ui.add_space(2.0);

        if dropdown(
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
        ) {
            for layout in &mut self.layouts {
                for graph in &mut layout.graphs {
                    graph.cur_lap = vec![];
                    graph.ref_lap = vec![];
                }
            }
        }
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

    fn draw_edit_layout_button(&mut self, ui: &mut Ui) {
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
            self.in_layout_edit_mode = true;
            self.edit_mode_context = EditModeContext {
                layout: EditLayoutInfo {
                    graphs: self.layouts[self.cur_layout_index].graphs.clone(),
                    index: self.cur_layout_index,
                },
                started_edtiting: false,
            };
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

    fn draw_graphs_edit(&mut self, ui: &mut Ui, graphs_rect: Rect) {
        ui.put(graphs_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                let mut changes: Vec<GraphChange> = vec![];
                for (i, graph_info) in self.edit_mode_context.layout.graphs.iter().enumerate() {
                    if i == self.edit_mode_context.layout.graphs.len() - 1 {
                        if let Some(gc) = graph_edit(
                            ui,
                            i,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            false,
                            CornerRadius {
                                ne: 0,
                                nw: 0,
                                sw: 24,
                                se: 24,
                            },
                        ) {
                            changes.push(gc);
                        }
                    } else if i == 0 {
                        if let Some(gc) = graph_edit(
                            ui,
                            i,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            true,
                            CornerRadius {
                                ne: 24,
                                nw: 24,
                                sw: 0,
                                se: 0,
                            },
                        ) {
                            changes.push(gc);
                        }
                    } else if self.edit_mode_context.layout.graphs.len() == 1 {
                        if let Some(gc) = graph_edit(
                            ui,
                            i,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            false,
                            CornerRadius::same(24),
                        ) {
                            changes.push(gc);
                        }
                    } else {
                        if let Some(gc) = graph_edit(
                            ui,
                            i,
                            graph_info,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            true,
                            CornerRadius::same(0),
                        ) {
                            changes.push(gc);
                        }
                    }
                }

                for change in changes {
                    match change {
                        GraphChange::Height(i, delta) => {
                            const MIN_SIZE: f32 = 0.10;

                            let graphs = &mut self.edit_mode_context.layout.graphs;

                            if delta > 0.0 {
                                let mut remaining = delta;

                                for graph in graphs.iter_mut().skip(i + 1) {
                                    let available = (graph.size_percent - MIN_SIZE).max(0.0);
                                    let taken = remaining.min(available);

                                    graph.size_percent -= taken;
                                    remaining -= taken;

                                    if remaining <= 0.0 {
                                        break;
                                    }
                                }

                                graphs[i].size_percent += delta - remaining;
                            } else {
                                let mut remaining = -delta;

                                let available = (graphs[i].size_percent - MIN_SIZE).max(0.0);
                                let taken = remaining.min(available);

                                graphs[i].size_percent -= taken;
                                remaining -= taken;

                                let mut j = i;

                                while remaining > 0.0 && j > 0 {
                                    j -= 1;

                                    let available = (graphs[j].size_percent - MIN_SIZE).max(0.0);
                                    let taken = remaining.min(available);

                                    graphs[j].size_percent -= taken;
                                    remaining -= taken;
                                }

                                graphs[i + 1].size_percent += -delta - remaining;
                            }
                        }
                        GraphChange::Type(i, new_type) => {
                            self.edit_mode_context.layout.graphs[i].graph_type =
                                GraphViewDataType::from_string(
                                    &new_type,
                                    self.cur_driver.1 as usize,
                                );
                        }
                        GraphChange::Color(i, new_color) => {
                            self.edit_mode_context.layout.graphs[i].color = new_color;
                        }
                        GraphChange::Gridlines(i, new_gridlines) => {
                            self.edit_mode_context.layout.graphs[i].n_gridlines = new_gridlines;
                        }
                        GraphChange::Reference(i, new_reference) => {
                            self.edit_mode_context.layout.graphs[i].show_ref = new_reference;
                        }
                        GraphChange::Delete(i) => {
                            self.edit_mode_context.layout.graphs.remove(i);
                            let new_num_graphs = self.edit_mode_context.layout.graphs.len();
                            self.edit_mode_context
                                .layout
                                .graphs
                                .iter_mut()
                                .for_each(|g| {
                                    g.size_percent = 1.0 / new_num_graphs as f32;
                                });
                        }
                    }
                }
            })
            .response
        });
    }

    fn draw_graphs_normal(&self, ui: &mut Ui, graphs_rect: Rect) {
        ui.put(graphs_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                let margins = 64.0;
                for (i, graph_info) in self.layouts[self.cur_layout_index]
                    .graphs
                    .iter()
                    .enumerate()
                {
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
                            &self.telemetry.update_telemetry().unwrap(),
                        );
                        continue;
                    }
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
                            &self.telemetry.update_telemetry().unwrap(),
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
                            &self.telemetry.update_telemetry().unwrap(),
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
                        &self.telemetry.update_telemetry().unwrap(),
                    );
                }
            })
            .response
        });
    }
}
