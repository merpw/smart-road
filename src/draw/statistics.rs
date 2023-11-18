use crate::config::WINDOW_SIZE;
use crate::STATS;
use macroquad::prelude::*;

fn draw_centered_text(text: &str, y: f32, size: f32, color: Color) {
    let text_width = measure_text(text, None, size as u16, 1.0).width;
    let x = WINDOW_SIZE as f32 / 2.0 - text_width / 2.0;
    draw_text(text, x, y, size, color);
}

pub fn draw_statistics() {
    clear_background(BLACK);
    let header_text = "Statistics".to_string();

    let statistics = [
        format!("Car count: {}", unsafe { STATS.car_count }),
        format!("Max speed: {}", unsafe { STATS.max_speed }),
        format!("Min speed: {}", unsafe { STATS.min_speed }),
        format!("Max time: {:.0} sec.", unsafe { STATS.max_time }),
        format!("Min time: {:.0} sec.", unsafe { STATS.min_time }),
        format!("Close calls: {}", unsafe { STATS.close_calls }),
        format!("Collisions: {}", unsafe { STATS.collision_count }),
    ];

    let text_size = 24.0;
    let text_color = WHITE;
    let text_y_start = WINDOW_SIZE as f32 / 2.5;
    let line_height = 30.0;

    draw_centered_text(&header_text, text_y_start, 30.0, RED);

    for (index, stat) in statistics.iter().enumerate() {
        let y = text_y_start + (index as f32 + 1.0) * line_height;
        draw_centered_text(stat, y, text_size, text_color);
    }
}
