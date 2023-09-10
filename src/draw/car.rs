use crate::config::{CAR_LENGTH, CAR_WIDTH};
use crate::traffic::{Car, Going};
use macroquad::color::{BLUE, RED, YELLOW};
use std::ops::Sub;

use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};

pub fn draw_car(car: &Car) {
    let color = match car.going {
        Going::Straight => BLUE,
        Going::Right => YELLOW,
        Going::Left => RED,
    };

    let move_vector = macroquad::math::Vec2::new(
        car.rotation.cos() * CAR_LENGTH,
        car.rotation.sin() * CAR_LENGTH,
    );

    let pos = car.pos.sub(move_vector);

    draw_rectangle_ex(
        pos.x,
        pos.y,
        CAR_LENGTH,
        CAR_WIDTH,
        DrawRectangleParams {
            rotation: car.rotation,
            color,
            ..Default::default()
        },
    );
}
