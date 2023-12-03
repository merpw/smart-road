use crate::app::Statistics;
use crate::config::WINDOW_SIZE;
use crate::draw::background_statistics::draw_statistics_background;
use macroquad::prelude::*;

fn draw_centered_text(text: &str, y: f32, size: f32, color: Color, font: Option<&Font>) {
    let text_width = measure_text(text, font, size as u16, 1.0).width;
    let x = WINDOW_SIZE as f32 / 2.0 - text_width / 2.0;
    let params = TextParams {
        font_size: size as u16,
        font,
        color,
        ..Default::default()
    };

    draw_text_ex(text, x, y, params);
}

pub fn draw_statistics(statistics: &Statistics, background: &Texture2D, font: Option<&Font>) {
    draw_statistics_background(background);
    let header_text = "Statistics".to_string();

    let messages = [
        format!("Car count: {}", statistics.car_count),
        format!("Max speed: {}", statistics.max_speed),
        format!("Min speed: {}", statistics.min_speed),
        format!("Max time: {:.0} sec.", statistics.max_time),
        format!("Min time: {:.0} sec.", statistics.min_time),
        format!("Close calls: {}", statistics.close_calls.len()),
        format!("Collisions: {}", statistics.collisions.len()),
    ];

    let christmas = format!("Days until Christmas: {:.0}", statistics.christmas);

    let text_size = 17.0;
    let text_color = WHITE;
    let text_y_start = WINDOW_SIZE as f32 / 2.5;
    let line_height = 30.0;

    draw_centered_text(
        &header_text,
        text_y_start,
        30.0,
        Color::from_rgba(163, 180, 203, 225),
        font,
    );

    for (index, stat) in messages.iter().enumerate() {
        let y = text_y_start + (index as f32 + 1.0) * line_height;
        draw_centered_text(stat, y, text_size, text_color, font);
    }

    draw_centered_text(
        &christmas,
        text_y_start + (messages.len() as f32 + 1.0) * line_height,
        text_size,
        Color::from_rgba(205, 255, 244, 255),
        font,
    );
}
