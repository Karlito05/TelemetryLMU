use crate::pages::map::map_main::MapPage;

impl MapPage {
    pub(in crate::pages::map) fn get_time_delta(&self) -> Option<f32> {
        let i_1 = self.cur_dp_1?;
        let i_2 = self.cur_dp_2?;
        let car1_cur = self.car_1.get(i_1)?.distance;
        let car2_cur = self.car_2.get(i_2)?.distance;

        let mut closest_index = i_2;
        let mut best_diff = (car2_cur - car1_cur).abs();

        if car1_cur > car2_cur {
            #[expect(clippy::needless_range_loop)]
            for j in (i_2 + 1)..self.car_2.len() {
                let diff = (self.car_2[j].distance - car1_cur).abs();
                if diff < best_diff {
                    best_diff = diff;
                    closest_index = j;
                }
                if self.car_2[j].distance > car1_cur && diff > best_diff {
                    break;
                }
            }
        } else {
            for j in (0..i_2).rev() {
                let diff = (self.car_2[j].distance - car1_cur).abs();
                if diff < best_diff {
                    best_diff = diff;
                    closest_index = j;
                }
                if self.car_2[j].distance < car1_cur && diff > best_diff {
                    break;
                }
            }
        }

        Some(self.car_2[closest_index].time_since_lap_start - self.car_1[i_1].time_since_lap_start)
    }
}
