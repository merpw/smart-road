use crate::config::{CAR_LENGTH, CAR_WIDTH};
use crate::traffic::{Car, Going};
use macroquad::color::{BLUE, RED, YELLOW};

use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};

pub fn draw_car(car: &Car) {
    let color = match car.going {
        Going::Straight => BLUE,
        Going::Right => YELLOW,
        Going::Left => RED,
    };

    draw_rectangle_ex(
        car.pos.x,
        car.pos.y,
        CAR_LENGTH,
        CAR_WIDTH,
        DrawRectangleParams {
            rotation: car.rotation,
            color,
            ..Default::default()
        },
    );
}
