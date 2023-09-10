use crate::traffic::{Direction, Light, Line};
use macroquad::prelude::get_frame_time;
use std::ops::IndexMut;

use crate::config::LIGHTS_TIMEOUT;
use rand::prelude::IteratorRandom;

#[derive(Debug)]
pub struct TrafficState {
    switch_timer: f32,

    green_num: usize,

    pub lines: [Line; 4],
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            switch_timer: 0.0,
            green_num: 0,

            lines: [
                Line::new(Direction::North, Light::Green),
                Line::new(Direction::East, Light::Red),
                Line::new(Direction::South, Light::Red),
                Line::new(Direction::West, Light::Red),
            ],
        }
    }

    fn switch(&mut self) {
        self.lines.index_mut(self.green_num).switch();

        self.green_num = (self.green_num + 1) % self.lines.len();

        self.lines.index_mut(self.green_num).switch();
    }

    pub fn update(&mut self) {
        self.switch_timer += get_frame_time();

        if self.switch_timer > LIGHTS_TIMEOUT {
            self.switch();
            self.switch_timer = 0.0;
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
