use crate::traffic::{ComingFrom, TrafficState};
use macroquad::prelude::*;

pub fn handle_input(traffic_state: &mut TrafficState) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if is_key_pressed(KeyCode::Up) {
        traffic_state.add_car(ComingFrom::South);
    }

    if is_key_pressed(KeyCode::Down) {
        traffic_state.add_car(ComingFrom::North);
    }

    if is_key_pressed(KeyCode::Right) {
        traffic_state.add_car(ComingFrom::West);
    }

    if is_key_pressed(KeyCode::Left) {
        traffic_state.add_car(ComingFrom::East);
    }
}

// pub struct Control {
//     last_spawn_time: Instant,
// }
// impl Control {
//     pub fn new() -> Self {
//         Self {
//             last_spawn_time: Instant::now(),
//         }
//     }

// pub fn spawn_cars(&mut self, traffic_state: &mut TrafficState) {
//     // Implement logic for spawning cars at a controlled rate
//     let now = Instant::now();
//     let time_since_last_spawn = now.duration_since(self.last_spawn_time);
//
//     // Adjust the duration below as needed to control the spawn rate.
//     let spawn_interval = Duration::from_secs(2);
//
//     if time_since_last_spawn >= spawn_interval {
//         traffic_state.add_car(ComingFrom::Random);
//         self.last_spawn_time = now;
//     }
// }
// }
