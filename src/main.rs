#![allow(dead_code)] // TODO: remove
mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use crate::traffic::TrafficState;

use crate::control::handle_input;
use crate::draw::{draw_car, draw_roads};
use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();

    loop {
        handle_input(&mut traffic_state);

        clear_background(BLACK);

        draw_roads();

        traffic_state.cars.iter().for_each(draw_car);

        next_frame().await
    }
}
