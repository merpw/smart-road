use crate::config::{BOTTOM_LEFT, BOTTOM_RIGHT, LIGHTS_SIZE, TOP_LEFT, TOP_RIGHT};
use crate::traffic::{Direction, Light, Line};
use macroquad::prelude::*;
use std::ops::Sub;

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
