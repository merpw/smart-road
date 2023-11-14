mod config;
mod control;
mod draw;
mod traffic;

use config::window_conf;
use macroquad::prelude::*;

use crate::draw::{draw_path, draw_roads};
use crate::traffic::TrafficState;
use control::handle_input;
use draw::draw_car;

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();

    loop {
        handle_input(&mut traffic_state);

        traffic_state.update();

        clear_background(BLACK);

        draw_roads();

        for line in traffic_state.lines.iter() {
            for path in line.paths.iter() {
                draw_path(path);

                line.path_cars(path).iter().for_each(draw_car);
            }
        }

        next_frame().await
    }
}
