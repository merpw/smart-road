use crate::config::{LIGHTS_SIZE, ROAD_WIDTH, WINDOW_SIZE};
use crate::traffic::{Direction, Light, Line};
use macroquad::prelude::*;
use std::ops::Sub;

const STRAIGHT_LENGTH: f32 = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0;

// center square corners
const TOP_LEFT: Vec2 = Vec2::new(STRAIGHT_LENGTH, STRAIGHT_LENGTH);

const TOP_RIGHT: Vec2 = Vec2::new(WINDOW_SIZE as f32 - STRAIGHT_LENGTH, STRAIGHT_LENGTH);

const BOTTOM_LEFT: Vec2 = Vec2::new(STRAIGHT_LENGTH, WINDOW_SIZE as f32 - STRAIGHT_LENGTH);

const BOTTOM_RIGHT: Vec2 = Vec2::new(
    WINDOW_SIZE as f32 - STRAIGHT_LENGTH,
    WINDOW_SIZE as f32 - STRAIGHT_LENGTH,
);

pub fn draw_light(line: &Line) {
    let pos = match line.coming_from {
        Direction::North => BOTTOM_LEFT.sub(Vec2::new(LIGHTS_SIZE, 0.0)),
        Direction::East => TOP_LEFT.sub(Vec2::new(LIGHTS_SIZE, LIGHTS_SIZE)),
        Direction::South => TOP_RIGHT.sub(Vec2::new(0.0, LIGHTS_SIZE)),
        Direction::West => BOTTOM_RIGHT,
    };

    draw_rectangle(
        pos.x,
        pos.y,
        LIGHTS_SIZE,
        LIGHTS_SIZE,
        match line.light {
            Light::Red => RED,
            Light::Green => GREEN,
        },
    );
}
