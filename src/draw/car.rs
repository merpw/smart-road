use crate::config::{CAR_LENGTH, CAR_WIDTH};
use crate::traffic::{Car, Going};
use macroquad::color::{BLUE, RED, YELLOW};

use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};

pub fn draw_car(car: &Car) {
    let x = car.x;
    let y = car.y;

    let color = match car.going {
        Going::Straight => BLUE,
        Going::Right => YELLOW,
        Going::Left => RED,
    };

    draw_rectangle_ex(
        x,
        y,
        CAR_LENGTH,
        CAR_WIDTH,
        DrawRectangleParams {
            rotation: car.rotation,
            color,
            ..Default::default()
        },
    );
}
