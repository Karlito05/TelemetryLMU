use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls) fn draw_input_row(&mut self, ui: &mut Ui) {
        let row_3_rect = ui
            .add_sized(
                vec2(ui.available_width(), ui.available_height()),
                |ui: &mut Ui| {
                    ui.horizontal_centered(|ui| {
                        if let Some(i) = self.cur_dp_1 {
                            self.draw_pedal_inputs(ui, self.car_1[i].throttle, self.car_1[i].brake);
                        } else {
                            self.draw_pedal_inputs(ui, 0.0, 0.0);
                        }

                        ui.add_space(8.0);

                        if let Some(i) = self.cur_dp_1 {
                            self.draw_wheel_input(ui, self.car_1[i].steering);
                        } else {
                            self.draw_wheel_input(ui, 0.0);
                        }

                        ui.separator();

                        if let Some(i) = self.cur_dp_1 {
                            self.draw_speed(ui, self.car_1[i].speed);
                        } else {
                            self.draw_speed(ui, 0.0);
                        }

                        ui.separator();

                        if let Some(i) = self.cur_dp_1 {
                            self.draw_gear(ui, self.car_1[i].gear);
                        } else {
                            self.draw_gear(ui, 0);
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if let Some(i) = self.cur_dp_2 {
                                self.draw_pedal_inputs(
                                    ui,
                                    self.car_2[i].throttle,
                                    self.car_2[i].brake,
                                );
                            } else {
                                self.draw_pedal_inputs(ui, 0.0, 0.0);
                            }

                            ui.add_space(8.0);

                            if let Some(i) = self.cur_dp_2 {
                                self.draw_wheel_input(ui, self.car_2[i].steering);
                            } else {
                                self.draw_wheel_input(ui, 0.0);
                            }

                            ui.separator();

                            if let Some(i) = self.cur_dp_2 {
                                self.draw_speed(ui, self.car_2[i].speed);
                            } else {
                                self.draw_speed(ui, 0.0);
                            }

                            ui.separator();

                            if let Some(i) = self.cur_dp_2 {
                                self.draw_gear(ui, self.car_2[i].gear);
                            } else {
                                self.draw_gear(ui, 0);
                            }
                        })
                    })
                    .response
                },
            )
            .rect;

        self.draw_delta(ui, &row_3_rect);
    }
}
