#![allow(dead_code)] // TODO: remove
mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use traffic::TrafficState;

use config::window_conf;

use control::handle_input;

use draw::{draw_car, draw_roads};

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();

    loop {
        handle_input(&mut traffic_state);

        clear_background(BLACK);

        traffic_state.cars.iter_mut().for_each(|car| car.update());

        draw_roads();

        traffic_state.cars.iter().for_each(draw_car);

        next_frame().await
    }
}
