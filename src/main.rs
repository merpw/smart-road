mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use config::window_conf;

use control::handle_input;

use crate::draw::{draw_light, draw_path, draw_roads, Textures};
use crate::traffic::TrafficState;
use draw::draw_car;

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();

    let textures = Textures::load().await;

    loop {
        handle_input(&mut traffic_state);

        traffic_state.update();

        clear_background(BLACK);

        draw_roads();

        for line in traffic_state.lines.iter() {
            draw_light(line, &textures);

            for path in line.paths.iter() {
                draw_path(path);
            }
        }

        for line in traffic_state.lines.iter() {
            for car in line.cars.iter() {
                draw_car(car);
            }
        }

        next_frame().await
    }
}
