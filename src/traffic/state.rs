use crate::config::{CAR_LENGTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::traffic::car::COMING_FROM;
use crate::traffic::{Car, ComingFrom};

use rand::prelude::IteratorRandom;

#[derive(Debug)]
pub struct TrafficState {
    pub cars: Vec<Car>,
    pub lights: [bool; 4],
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            cars: Vec::new(),
            lights: [false; 4],
        }
    }

    fn can_add_car(&self, coming_from: ComingFrom) -> bool {
        let prev_car = self.cars.iter().rfind(|c| c.coming_from == coming_from);

        if prev_car.is_none() {
            return true;
        }

        let prev_car = prev_car.unwrap();

        match coming_from {
            ComingFrom::North => prev_car.y >= CAR_LENGTH,
            ComingFrom::East => WINDOW_WIDTH as f32 - prev_car.x >= CAR_LENGTH,
            ComingFrom::South => WINDOW_HEIGHT as f32 - prev_car.y >= CAR_LENGTH,
            ComingFrom::West => prev_car.x >= CAR_LENGTH,
        }
    }

    pub fn add_car_random(&mut self) {
        let available_coming_from = COMING_FROM
            .iter()
            .filter(|cf| self.can_add_car(**cf))
            .choose(&mut rand::thread_rng());

        if let Some(coming_from) = available_coming_from {
            self.add_car(*coming_from);
        }
    }

    pub fn add_car(&mut self, coming_from: ComingFrom) {
        if self.can_add_car(coming_from) {
            self.cars.push(Car::new(coming_from));
        }
    }
}

pub fn traffic(_ts: &mut TrafficState) {
    todo!()
}
