use crate::traffic::{Direction, Light, Line};
use macroquad::prelude::get_frame_time;

use crate::config::{BOTTOM_RIGHT, LIGHTS_TIMEOUT, TOP_LEFT};
use rand::prelude::IteratorRandom;

#[derive(Debug)]
pub struct TrafficState {
    switch_timer: f32,

    pub lines: [Line; 4],
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            switch_timer: 0.0,

            lines: [
                Line::new(Direction::North, Light::Green),
                Line::new(Direction::East, Light::Red),
                Line::new(Direction::South, Light::Red),
                Line::new(Direction::West, Light::Red),
            ],
        }
    }

    fn update_lights(&mut self) {
        self.lines.iter_mut().for_each(|line| {
            if line.light == Light::Green {
                line.switch();
            }
        });

        let is_car_in_intersection = self.lines.iter().any(|line| {
            line.cars.iter().any(|car| {
                matches!((car.pos.x, car.pos.y), (x, y) if x > TOP_LEFT.x
                        && y > TOP_LEFT.y
                        && x < BOTTOM_RIGHT.x
                        && y < BOTTOM_RIGHT.y)
            })
        });

        if is_car_in_intersection {
            return;
        }

        let biggest_queue = self
            .lines
            .iter_mut()
            .max_by(|line1, line2| line1.cars.len().cmp(&line2.cars.len()))
            .unwrap();

        biggest_queue.switch();
        self.switch_timer = 0.0;
    }

    pub fn update(&mut self) {
        self.switch_timer += get_frame_time();

        if self.switch_timer > LIGHTS_TIMEOUT {
            self.update_lights();
        }

        self.lines.iter_mut().for_each(|line| line.update());
    }

    pub fn add_car(&mut self, coming_from: Direction) {
        let line = self
            .lines
            .iter_mut()
            .find(|line| line.coming_from == coming_from)
            .unwrap();

        line.add_car();
    }

    pub fn add_car_random(&mut self) {
        let available_lines = self
            .lines
            .iter_mut()
            .filter(|line| line.can_add_car())
            .choose(&mut rand::thread_rng());

        if let Some(line) = available_lines {
            line.add_car();
        }
    }
}
