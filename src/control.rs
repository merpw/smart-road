use crate::car::ComingFrom;
use crate::traffic::TrafficState;
use crate::road;

use macroquad::prelude::*;
use std::time::Duration;
use std::time::Instant;

pub struct Control {
    last_spawn_time: Instant,
}

impl Control {
    pub fn new() -> Self {
        Self {
            last_spawn_time: Instant::now(),
        }
    }

    pub fn handle_input(&mut self, traffic_state: &mut TrafficState) {
        // Handle user input to add cars using a match statement
        match keycode() {
            KeyCode::Up => traffic_state.add_car(ComingFrom::North),
            KeyCode::Down => traffic_state.add_car(ComingFrom::South),
            KeyCode::Right => traffic_state.add_car(ComingFrom::East),
            KeyCode::Left => traffic_state.add_car(ComingFrom::West),
            KeyCode::R => traffic_state.add_car(ComingFrom::Random),
            KeyCode::Escape => {
                // Handle Escape key to end the simulation or perform other actions.
                // You can add the logic here to exit the program or reset the simulation.
            }
            _ => {} // Ignore other key presses
        }
    }
    
    pub fn spawn_cars(&mut self, traffic_state: &mut TrafficState) {
        // Implement logic for spawning cars at a controlled rate
        let now = Instant::now();
        let time_since_last_spawn = now.duration_since(self.last_spawn_time);

        // Adjust the duration below as needed to control the spawn rate.
        let spawn_interval = Duration::from_secs(2);

        if time_since_last_spawn >= spawn_interval {
            traffic_state.add_car(ComingFrom::Random);
            self.last_spawn_time = now;
        }
    }
}
