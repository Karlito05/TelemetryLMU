use crate::pages::map::map_main::MapPage;
use eframe::egui::*;

impl MapPage {
    pub(in crate::pages::map::controls) fn draw_control_row(&mut self, ui: &mut Ui) {
        let row_rect = ui
            .add_sized(vec2(ui.available_width(), 48.0), |ui: &mut Ui| {
                ui.horizontal(|ui| {
                    Self::draw_select_reference_button(
                        ui,
                        &mut self.cur_dp_1,
                        &mut self.car_1,
                        &mut self.car_1_info,
                        &self.car_2_info,
                        self.settings_provider.clone(),
                        &mut self.track_reference,
                        &mut self.show_track_not_same_popup,
                    );

                    Self::draw_clear_button(
                        ui,
                        &mut self.car_1_info,
                        &mut self.car_1,
                        &mut self.cur_dp_1,
                    );

                    if let Some(info) = self.car_1_info.clone() {
                        let driver_info_rect =
                            ui.allocate_exact_size(vec2(140.0, 36.0), Sense::empty()).0;
                        self.draw_lap_info(ui, driver_info_rect, info);
                    }

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        Self::draw_select_reference_button(
                            ui,
                            &mut self.cur_dp_2,
                            &mut self.car_2,
                            &mut self.car_2_info,
                            &self.car_1_info,
                            self.settings_provider.clone(),
                            &mut self.track_reference,
                            &mut self.show_track_not_same_popup,
                        );

                        Self::draw_clear_button(
                            ui,
                            &mut self.car_2_info,
                            &mut self.car_2,
                            &mut self.cur_dp_2,
                        );

                        if let Some(info) = self.car_2_info.clone() {
                            let driver_info_rect =
                                ui.allocate_exact_size(vec2(140.0, 36.0), Sense::empty()).0;
                            self.draw_lap_info(ui, driver_info_rect, info);
                        }
                    });
                })
                .response
            })
            .rect;

        let spacing = ui.spacing().item_spacing.x;
        let group_size = vec2(48.0 * 3.0 + spacing * 2.0, 48.0);
        let group_rect = Rect::from_center_size(row_rect.center(), group_size);

        self.draw_playback_controls(ui, group_rect);
    }
}
