mod config;
mod control;
mod draw;
mod traffic;

use crate::draw::{draw_background, draw_path};
use crate::traffic::TrafficState;
use config::window_conf;
use control::handle_input;
use draw::draw_car;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();

    let background_texture = load_texture("assets/background.png").await.unwrap();
    let car_right_texture = load_texture("assets/car_green.png").await.unwrap();
    let car_left_texture = load_texture("assets/car_violet.png").await.unwrap();
    let car_straight_texture = load_texture("assets/car_yellow.png").await.unwrap();

    loop {
        handle_input(&mut traffic_state);

        traffic_state.update();

        draw_background(&background_texture);

        for line in traffic_state.lines.iter() {
            for path in line.paths.iter() {
                draw_path(path);
            }
        }

        for line in traffic_state.lines.iter() {
            for car in line.cars.iter() {
                draw_car(
                    car,
                    &car_straight_texture,
                    &car_right_texture,
                    &car_left_texture,
                );
            }
        }

        next_frame().await
    }
}
