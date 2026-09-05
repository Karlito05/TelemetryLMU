use crate::frontend::components::dropdown;
use crate::frontend::components::{
    DropdownItem, DynGraphData, GraphChange, GraphInfo, Lap, button, graph, graph_edit,
};
use crate::frontend::frontend_main::{SettingsProvider, StateProvider};
use crate::telemetry::{Telemetry, TelemetryValueType};
use eframe::egui::*;
use egui_phosphor_icons::icons;
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TelemetryPage {
    cur_layout_index: usize,
    layouts: Vec<LayoutInfo>,
    #[serde(skip)]
    cur_driver: (String, i32),
    #[serde(skip)]
    in_layout_edit_mode: bool,
    #[serde(skip)]
    edit_mode_context: EditModeContext,
    #[serde(skip)]
    show_delete_layout_dialog: bool,
    #[serde(skip)]
    show_add_limit_dialog: bool,
    #[serde(skip)]
    cur_ref_path: Option<String>,
    #[serde(skip)]
    save_as_dialog_info: SaveAsDialogInfo,
    #[serde(skip)]
    cur_lap: i32,
    #[serde(skip)]
    settings_provider: Arc<SettingsProvider>,
    #[serde(skip)]
    state_provider: Arc<StateProvider>,
}

#[derive(Clone, Debug, Default)]
struct EditModeContext {
    layout: EditLayoutInfo,
    #[expect(unused)]
    started_edtiting: bool,
}

#[derive(Clone, Debug, Default)]
struct SaveAsDialogInfo {
    name: String,
    show: bool,
}

#[derive(Clone, Debug, Default)]
struct EditLayoutInfo {
    index: usize,
    graphs: Vec<GraphInfo>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
#[serde(default)]
struct LayoutInfo {
    name: String,
    graphs: Vec<GraphInfo>,
}

impl TelemetryPage {
    pub fn new(
        settings_provider: Arc<SettingsProvider>,
        state_provider: Arc<StateProvider>,
    ) -> Self {
        Self {
            settings_provider,
            state_provider,
            cur_driver: ("".to_owned(), 0),
            in_layout_edit_mode: false,
            show_delete_layout_dialog: false,
            show_add_limit_dialog: false,
            cur_ref_path: None,
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
                    ref_val_type: TelemetryValueType::Rpm,
                    color: Color32::WHITE,
                    n_gridlines: 3,
                    size_percent: 1.0,
                }],
            }],
        }
    }
}

impl TelemetryPage {
    pub fn draw_telemetry_page(&mut self, ui: &mut Ui, telemetry: &Telemetry) {
        // TODO: Refactor this into frontend_main!
        // if !self.telemetry.full_mode {
        //     telemetry_not_found(ui);
        //     return;
        // }
        //
        // self.process_telemetry_updates(
        //     self.layouts[self.cur_layout_index]
        //         .graphs
        //         .iter()
        //         .map(|g| g.graph_type)
        //         .collect(),
        // );
        if !self.in_layout_edit_mode {
            self.draw_normal_mode(ui, telemetry);
        } else {
            self.draw_edit_mode(ui);
        }
    }

    // fn process_telemetry_updates(&mut self, graph_data_types: Vec<GraphViewDataType>) {
    //     let t = self.telemetry.update_telemetry().unwrap();
    //
    //     if self.cur_lap != graph_data_types[0].get_lap(&t) {
    //         if graph_data_types[0].is_last_best(&t) && self.cur_ref_path.is_none() {
    //             graph_data_types.iter().enumerate().for_each(|(i, _)| {
    //                 self.layouts[self.cur_layout_index].graphs[i].ref_lap =
    //                     self.layouts[self.cur_layout_index].graphs[i]
    //                         .cur_lap
    //                         .clone();
    //             });
    //         }
    //         graph_data_types.iter().enumerate().for_each(|(i, _)| {
    //             self.layouts[self.cur_layout_index].graphs[i].cur_lap = vec![];
    //         });
    //         self.cur_lap = graph_data_types[0].get_lap(&t);
    //     }
    //
    //     graph_data_types
    //         .iter()
    //         .enumerate()
    //         .for_each(|(i, graph_data_type)| {
    //             if !self.layouts[self.cur_layout_index].graphs[i]
    //                 .cur_lap
    //                 .is_empty()
    //                 || graph_data_type.get_normalized_distance(&t) < 0.9
    //             {
    //                 let new_y = graph_data_type.get_normalized_values(&t) as f32;
    //                 // match self.layouts[self.cur_layout_index].graphs[i].cur_lap.last() {
    //                 //     Some(last) => {
    //                 //         if (last.y - new_y).abs() > 0.005 {
    //                 //             self.layouts[self.cur_layout_index].graphs[i]
    //                 //                 .cur_lap
    //                 //                 .push(vec2(
    //                 //                     graph_data_type.get_normalized_distance(&t) as f32,
    //                 //                     new_y,
    //                 //                 ))
    //                 //         }
    //                 //     }
    //                 //     None => self.layouts[self.cur_layout_index].graphs[i]
    //                 //         .cur_lap
    //                 //         .push(vec2(
    //                 //             graph_data_type.get_normalized_distance(&t) as f32,
    //                 //             new_y,
    //                 //         )),
    //                 // }
    //                 self.layouts[self.cur_layout_index].graphs[i]
    //                     .cur_lap
    //                     .push(vec2(
    //                         graph_data_type.get_normalized_distance(&t) as f32,
    //                         new_y,
    //                     ));
    //             }
    //         });
    // }

    fn draw_edit_mode(&mut self, ui: &mut Ui) {
        let top_bar_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                16.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        0.0
                    } else {
                        16.0
                    },
                48.0,
            ),
        );
        self.draw_top_bar_edit(ui, top_bar_rect);

        let graphs_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                80.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        8.0
                    } else {
                        16.0
                    },
                ui.viewport_rect().size().y - (32.0 + 64.0),
            ),
        );
        self.draw_graphs_edit(ui, graphs_rect);
    }

    fn draw_normal_mode(&mut self, ui: &mut Ui, telemetry: &Telemetry) {
        let top_bar_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                16.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        0.0
                    } else {
                        16.0
                    },
                48.0,
            ),
        );

        self.draw_top_bar_normal(ui, top_bar_rect, telemetry);

        let graphs_rect = Rect::from_min_size(
            pos2(
                if *self.state_provider.sidebar_open.read().unwrap() {
                    300.0
                } else {
                    16.0
                },
                80.0,
            ),
            vec2(
                ui.available_width()
                    - if *self.state_provider.sidebar_open.read().unwrap() {
                        8.0
                    } else {
                        16.0
                    },
                ui.viewport_rect().size().y - (32.0 + 64.0),
            ),
        );

        self.draw_graphs_normal(ui, graphs_rect, telemetry);
    }

    fn draw_top_bar_edit(&mut self, ui: &mut Ui, top_bar_rect: Rect) {
        ui.painter().rect_filled(
            top_bar_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        ui.put(top_bar_rect, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);

                self.draw_sidebar_button(ui);

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
            // for graph in &mut self.layouts[self.edit_mode_context.layout.index].graphs {
            //     graph.cur_lap = Lap::default();
            //     graph.ref_lap = Lap::default();
            // }
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
                    ref_val_type: TelemetryValueType::Rpm,
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

    fn draw_top_bar_normal(&mut self, ui: &mut Ui, top_bar_rect: Rect, telemetry: &Telemetry) {
        ui.painter().rect_filled(
            top_bar_rect,
            CornerRadius::same(24),
            Color32::from_rgb(22, 23, 28),
        );

        ui.put(top_bar_rect, |ui: &mut Ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);

                self.draw_sidebar_button(ui);

                ui.separator();

                self.draw_driver_select(ui, telemetry);

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

    fn draw_sidebar_button(&self, ui: &mut Ui) {
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
            self.state_provider.sidebar_open.write().unwrap().toggle();
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

    fn draw_driver_select(&mut self, ui: &mut Ui, telemetry: &Telemetry) {
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
            telemetry
                .get_drivers()
                .iter()
                .map(|driver| DropdownItem {
                    value: driver.clone(),
                    display_value: driver.0.clone(),
                })
                .collect(),
        ) {
            for layout in &mut self.layouts {
                for _graph in &mut layout.graphs {
                    // TODO: Fix this
                    // graph. = GraphViewDataType::from_string(
                    //     &graph.ref_val_type.to_string(),
                    //     self.cur_driver.1 as usize,
                    // );
                    // graph.cur_lap = Lap::default();
                    // graph.ref_lap = Lap::default();
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
            for _layout in &mut self.layouts {
                // for graph in &mut layout.graphs {
                //     graph.cur_lap = Lap::default();
                //     graph.ref_lap = Lap::default();
                // }
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
            // TODO: When I reimplement the lap Stores reimplement this
            //
            // if let Some(path) = rfd::FileDialog::new()
            //     .set_title("Select a reference file")
            //     .set_directory(settings.record_save_path.clone())
            //     .add_filter("JSON files", &["json"])
            //     .pick_file()
            // {
            //     self.cur_ref_path = Some(path.display().to_string());
            //
            //     let contents = fs::read_to_string(path).unwrap_or_default();
            //     let save_data: SaveData = serde_json::from_str(&contents).unwrap_or_default();
            //
            //     for graph in self.layouts[self.cur_layout_index].graphs.iter_mut() {
            //         let gt = graph.graph_type.to_string();
            //
            //         graph.ref_lap = save_data
            //             .lap_data
            //             .iter()
            //             .find(|d| d.data_type == gt)
            //             .unwrap()
            //             .values
            //             .clone()
            //     }
            // }
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
            self.cur_ref_path = None;
            // for graph in &mut self.layouts[self.cur_layout_index].graphs {
            //     graph.ref_lap = Lap::default();
            // }
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
                                graph_info.size_percent * graphs_rect.height() - 3.0,
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
                                graph_info.size_percent * graphs_rect.height() - 3.0,
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
                            self.edit_mode_context.layout.graphs[i].ref_val_type =
                                TelemetryValueType::from_string(&new_type);
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

    fn draw_graphs_normal(&self, ui: &mut Ui, graphs_rect: Rect, telemetry: &Telemetry) {
        ui.put(graphs_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                let margins = 64.0;
                for (i, graph_info) in self.layouts[self.cur_layout_index]
                    .graphs
                    .iter()
                    .enumerate()
                {
                    let cur_lap_guard = telemetry.cur_lap.lock().unwrap();
                    let last_lap_guard = telemetry.last_lap.lock().unwrap();

                    let cur = &cur_lap_guard[self.cur_driver.1 as usize];
                    let last = &last_lap_guard[self.cur_driver.1 as usize];

                    let dyn_graph_data = DynGraphData {
                        cur_lap: Lap {
                            values: &cur.datapoints[graph_info.ref_val_type.clone() as usize],
                            distances: &cur.datapoints
                                [TelemetryValueType::DistanceIntoLap as usize],
                        },
                        ref_lap: Lap {
                            values: &last.datapoints[graph_info.ref_val_type.clone() as usize],
                            distances: &last.datapoints
                                [TelemetryValueType::DistanceIntoLap as usize],
                        },
                    };

                    if self.layouts[self.cur_layout_index].graphs.len() == 1 {
                        graph(
                            ui,
                            graph_info,
                            &dyn_graph_data,
                            self.cur_driver.1 as usize,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height(),
                            ),
                            CornerRadius::same(24),
                            margins,
                            &telemetry.get_telemetry_object(),
                        );
                        continue;
                    }

                    if i == 0 {
                        graph(
                            ui,
                            graph_info,
                            &dyn_graph_data,
                            self.cur_driver.1 as usize,
                            vec2(
                                graphs_rect.width(),
                                graph_info.size_percent * graphs_rect.height() - 3.0, // - 3.0 to
                                                                                      // adjust for spacing because why tf not make it 3 px :D
                            ),
                            CornerRadius {
                                nw: 24,
                                ne: 24,
                                sw: 0,
                                se: 0,
                            },
                            margins,
                            &telemetry.get_telemetry_object(),
                        );
                        continue;
                    }

                    if i == self.layouts[self.cur_layout_index].graphs.len() - 1 {
                        graph(
                            ui,
                            graph_info,
                            &dyn_graph_data,
                            self.cur_driver.1 as usize,
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
                            &telemetry.get_telemetry_object(),
                        );
                        continue;
                    }

                    graph(
                        ui,
                        graph_info,
                        &dyn_graph_data,
                        self.cur_driver.1 as usize,
                        vec2(
                            graphs_rect.width(),
                            graph_info.size_percent * graphs_rect.height() - 3.0,
                        ),
                        CornerRadius::same(0),
                        margins,
                        &telemetry.get_telemetry_object(),
                    );
                }
            })
            .response
        });
    }
}
