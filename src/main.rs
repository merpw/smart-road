#![allow(dead_code)] // TODO: remove
mod config;
mod control;
mod draw;
mod traffic;

use macroquad::prelude::*;

use traffic::TrafficState;

use config::window_conf;

use control::handle_input;

use crate::draw::PathDrawer;
use draw::{draw_car, draw_roads};

#[macroquad::main(window_conf)]
async fn main() {
    let mut traffic_state = TrafficState::new();

    let path_drawer = PathDrawer::new();

    loop {
        handle_input(&mut traffic_state);

        traffic_state.update();

        clear_background(BLACK);

        draw_roads();

        path_drawer.draw();

        traffic_state.cars.iter().for_each(draw_car);

        next_frame().await
    }
}
