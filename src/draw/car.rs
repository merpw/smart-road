use crate::config::{CAR_LENGTH, CAR_WIDTH};
use crate::traffic::Car;
use macroquad::shapes::{draw_rectangle_ex, DrawRectangleParams};

pub fn draw_car(car: &Car) {
    let x = car.x;
    let y = car.y;

    draw_rectangle_ex(
        x,
        y,
        CAR_WIDTH,
        CAR_LENGTH,
        DrawRectangleParams {
            rotation: car.rotation,
            ..Default::default()
        },
    );
}
