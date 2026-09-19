use eframe::egui::Color32;

pub fn interpolate_color(
    value: f32,
    min: f32,
    mid: f32,
    max: f32,
    start_color: Color32,
    mid_color: Color32,
    end_color: Color32,
) -> Color32 {
    let value = min.max(max.min(value));

    if value <= mid {
        let t = (value - min) / (mid - min);
        return interpolate_between(start_color, mid_color, t);
    }

    let t = (value - mid) / (max - mid);
    interpolate_between(mid_color, end_color, t)
}

fn interpolate_between(a: Color32, b: Color32, t: f32) -> Color32 {
    let r_val = (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t).round() as u8;
    let g_val = (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t).round() as u8;
    let b_val = (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t).round() as u8;

    Color32::from_rgb(r_val, g_val, b_val)
}
