use crate::config::{BOTTOM_LEFT, BOTTOM_RIGHT, LIGHTS_SIZE, TOP_LEFT, TOP_RIGHT};
use crate::draw::Textures;
use crate::traffic::{Direction, Light, Line};
use macroquad::prelude::*;
use std::ops::Sub;

pub fn draw_light(line: &Line, textures: &Textures) {
    let pos = match line.coming_from {
        Direction::North => TOP_LEFT.sub(Vec2::new(LIGHTS_SIZE, LIGHTS_SIZE)),

        Direction::East => TOP_RIGHT.sub(Vec2::new(0.0, LIGHTS_SIZE)),
        Direction::South => BOTTOM_RIGHT,

        Direction::West => BOTTOM_LEFT.sub(Vec2::new(LIGHTS_SIZE, 0.0)),
    };

    let texture = match line.light {
        Light::Red => &textures.light_red,
        Light::Green => &textures.light_green,
    };

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            ..Default::default()
        },
    );
}
