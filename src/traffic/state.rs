use crate::traffic::{Direction, Line};

use rand::prelude::IteratorRandom;

#[derive(Debug)]
pub struct TrafficState {
    pub lines: [Line; 4],
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            lines: [
                Line::new(Direction::North),
                Line::new(Direction::East),
                Line::new(Direction::South),
                Line::new(Direction::West),
            ],
        }
    }

    pub fn update(&mut self) {
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
