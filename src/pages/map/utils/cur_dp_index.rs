use crate::pages::map::map_main::{Dp, MapPage};

impl MapPage {
    pub(in crate::pages::map) fn get_current_dp_index(
        &self,
        data: &[Dp],
        lap_time: f32,
        offset: f32,
    ) -> Option<usize> {
        if data.is_empty() || lap_time <= 0.0 {
            return None;
        }
        let mut target_time = (self.time * lap_time) / offset;

        if target_time > lap_time {
            target_time = lap_time;
        }

        let lower_index = data
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let a_diff = (a.time_since_lap_start - target_time).abs();
                let b_diff = (b.time_since_lap_start - target_time).abs();

                a_diff.partial_cmp(&b_diff).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or_default();

        if lower_index == data.len() - 1 {
            return Some(lower_index);
        }

        let mut upper_index = lower_index;

        for i in upper_index..data.len() {
            if data[lower_index].time_since_lap_start != data[i].time_since_lap_start {
                upper_index = i;
                break;
            }
        }

        let lower_time = data[lower_index].time_since_lap_start;
        let mut upper_time = data[upper_index].time_since_lap_start;

        if upper_index == lower_index {
            upper_index = data.len() - 1;
            upper_time = lap_time;
        }

        let t = (target_time - lower_time) / (upper_time - lower_time);

        Some((lower_index as f32 + t * (upper_index - lower_index) as f32) as usize)
    }
}
