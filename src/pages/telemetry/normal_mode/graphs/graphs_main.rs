use eframe::egui::*;

use crate::{
    pages::telemetry::{
        normal_mode::graphs::graph::graph_main::graph,
        telemetry_main::{DynGraphData, Lap, TelemetryPage},
    },
    providers::telemetry_provider::telemetry_provider_main,
};

impl TelemetryPage {
    pub(in crate::pages::telemetry::normal_mode) fn draw_graphs_normal(
        &self,
        ui: &mut Ui,
        graphs_rect: Rect,
    ) {
        ui.put(graphs_rect, |ui: &mut Ui| {
            ui.vertical(|ui| {
                let margins = 64.0;
                for (i, graph_info) in self.layouts[self.cur_layout_index]
                    .graphs
                    .iter()
                    .enumerate()
                {
                    let cur_lap_guard = self
                        .telemetry_provider
                        .as_ref()
                        .as_ref()
                        .unwrap()
                        .cur_lap
                        .lock()
                        .unwrap();
                    let best_lap_guard = self
                        .telemetry_provider
                        .as_ref()
                        .as_ref()
                        .unwrap()
                        .best_lap
                        .lock()
                        .unwrap();
                    let cur = &cur_lap_guard[self.cur_driver.1 as usize];

                    let best: &telemetry_provider_main::Lap;
                    if let Some(best_lap) = &self.ref_lap_override {
                        best = best_lap;
                    } else {
                        best = &best_lap_guard[self.cur_driver.1 as usize];
                    }

                    let dyn_graph_data = DynGraphData {
                        cur_lap: Lap {
                            values: &cur.datapoints[graph_info.ref_val_type.clone() as usize],
                            distances: &cur.distances,
                        },
                        ref_lap: Lap {
                            values: &best.datapoints[graph_info.ref_val_type.clone() as usize],
                            distances: &best.distances,
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
                            &self
                                .telemetry_provider
                                .as_ref()
                                .as_ref()
                                .unwrap()
                                .get_telemetry_object(),
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
                            &self
                                .telemetry_provider
                                .as_ref()
                                .as_ref()
                                .unwrap()
                                .get_telemetry_object(),
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
                            &self
                                .telemetry_provider
                                .as_ref()
                                .as_ref()
                                .unwrap()
                                .get_telemetry_object(),
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
                        &self
                            .telemetry_provider
                            .as_ref()
                            .as_ref()
                            .unwrap()
                            .get_telemetry_object(),
                    );
                }
            })
            .response
        });
    }
}
