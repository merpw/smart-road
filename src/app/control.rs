use crate::traffic::{Direction, TrafficState};
use crate::STATS;
use macroquad::prelude::*;

pub fn handle_input(traffic_state: &mut TrafficState) {
    if is_key_pressed(KeyCode::Escape) {
        if unsafe { STATS.is_open } {
            std::process::exit(0);
        }
        unsafe {
            STATS.toggle();
        }
    }

    if !unsafe { STATS.is_open } {
        if is_key_pressed(KeyCode::Up) {
            traffic_state.add_car(Direction::South);
        }

        if is_key_pressed(KeyCode::Down) {
            traffic_state.add_car(Direction::North);
        }

        if is_key_pressed(KeyCode::Right) {
            traffic_state.add_car(Direction::West);
        }

        if is_key_pressed(KeyCode::Left) {
            traffic_state.add_car(Direction::East);
        }

        if is_key_pressed(KeyCode::R) {
            traffic_state.add_car_random();
        }
    }
}
