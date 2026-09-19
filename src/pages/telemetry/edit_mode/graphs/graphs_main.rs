use crate::{
    pages::telemetry::{
        edit_mode::graphs::graph_dummy::graph_dummy_main::graph_edit, telemetry_main::TelemetryPage,
    },
    telemetry::TelemetryGraphValueType,
};
use eframe::egui::*;

// TODO: Refactor this

pub(super) enum GraphChange {
    Height(usize, f32), // Height delta in percent (0-1)
    Type(usize, String),
    Color(usize, Color32),
    Gridlines(usize, i32),
    Reference(usize, bool),
    Delete(usize),
}
impl TelemetryPage {
    pub(in crate::pages::telemetry::edit_mode) fn draw_graphs_edit(
        &mut self,
        ui: &mut Ui,
        graphs_rect: Rect,
    ) {
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
                                TelemetryGraphValueType::from_string(&new_type);
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
}
