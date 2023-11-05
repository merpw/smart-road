use crate::config::{CAR_LENGTH, CAR_WIDTH};
use crate::traffic::{Car, Going};
use macroquad::prelude::*;
use std::ops::Sub;

pub fn draw_car(
    car: &Car,
    car_straight_texture: &Texture2D,
    car_right_texture: &Texture2D,
    car_left_texture: &Texture2D,
) {
    let texture = match car.going {
        Going::Straight => car_straight_texture,
        Going::Right => car_right_texture,
        Going::Left => car_left_texture,
    };

    let move_vector = Vec2::new(
        car.rotation.cos() * CAR_LENGTH,
        car.rotation.sin() * CAR_LENGTH,
    );

    let pos = car.pos.sub(move_vector);

    draw_texture_ex(
        texture,
        pos.x,
        pos.y,
        WHITE,
        DrawTextureParams {
            rotation: car.rotation,
            pivot: Some(vec2(pos.x, pos.y)),
            dest_size: Some(vec2(CAR_LENGTH, CAR_WIDTH)),
            ..Default::default()
        },
    );
}
