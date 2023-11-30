use crate::config::{CAR_LENGTH, CAR_WIDTH};
use crate::traffic::{Car, Going};
use macroquad::prelude::*;
use std::ops::Sub;

pub fn draw_car(car: &Car, car_texture: &(Texture2D, Texture2D, Texture2D)) {
    let texture = match car.path.going_to {
        Going::Straight => &car_texture.0,
        Going::Right => &car_texture.1,
        Going::Left => &car_texture.2,
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
