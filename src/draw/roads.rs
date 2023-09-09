use crate::config::{
    ROAD_COLOR, ROAD_LINE_COLOR, ROAD_LINE_WIDTH, ROAD_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use macroquad::prelude::*;

pub fn draw_roads() {
    // Vertical road
    draw_rectangle(
        (WINDOW_WIDTH as f32 - ROAD_WIDTH) / 2.0,
        0.0,
        ROAD_WIDTH,
        WINDOW_WIDTH as f32,
        ROAD_COLOR,
    );

    // Horizontal road
    draw_rectangle(
        0.0,
        (WINDOW_WIDTH as f32 - ROAD_WIDTH) / 2.0,
        WINDOW_WIDTH as f32,
        ROAD_WIDTH,
        ROAD_COLOR,
    );

    for i in 0..(WINDOW_WIDTH / 100) {
        // vertical road line
        draw_rectangle(
            (WINDOW_WIDTH as f32 - ROAD_LINE_WIDTH) / 2.0,
            25.0 + i as f32 * 100.0,
            ROAD_LINE_WIDTH,
            50.0,
            ROAD_LINE_COLOR,
        );

        // horizontal road line
        draw_rectangle(
            25.0 + i as f32 * 100.0,
            (WINDOW_WIDTH as f32 - ROAD_LINE_WIDTH) / 2.0,
            50.0,
            ROAD_LINE_WIDTH,
            ROAD_LINE_COLOR,
        );
    }

    // remove lines in the center
    draw_rectangle(
        (WINDOW_WIDTH as f32 - ROAD_WIDTH) / 2.0,
        (WINDOW_HEIGHT as f32 - ROAD_WIDTH) / 2.0,
        ROAD_WIDTH,
        ROAD_WIDTH,
        ROAD_COLOR,
    );
}
