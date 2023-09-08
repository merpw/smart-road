use rand::Rng;
use crate::car::{Car, ComingFrom};

#[derive(Debug)]
pub struct TrafficState {
    pub from_north: Vec<Car>,
    pub from_east: Vec<Car>,
    pub from_south: Vec<Car>,
    pub from_west: Vec<Car>,
    pub to_north: Vec<Car>,
    pub to_east: Vec<Car>,
    pub to_south: Vec<Car>,
    pub to_west: Vec<Car>,
    pub allow_new_car: [bool; 4],
    pub lights: [bool; 4]
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            from_north: Vec::with_capacity(8),
            from_east: Vec::with_capacity(8),
            from_south: Vec::with_capacity(8),
            from_west: Vec::with_capacity(8),
            to_north: Vec::with_capacity(8),
            to_east: Vec::with_capacity(8),
            to_south: Vec::with_capacity(8),
            to_west: Vec::with_capacity(8),
            allow_new_car: [true; 4],
            lights: [false; 4]
        }
    }

    pub fn add_car(&mut self, coming_from: ComingFrom) {
        match coming_from {
            ComingFrom::North => {
                if self.from_north.len() < 8 && self.allow_new_car[0] {
                    self.allow_new_car[0] = false;
                    self.from_north.insert(0, Car::new(coming_from));
                }
            },
            ComingFrom::East => {
                if self.from_east.len() < 8 && self.allow_new_car[1] {
                    self.allow_new_car[1] = false;
                    self.from_east.insert(0, Car::new(coming_from));
                }
            },
            ComingFrom::South => {
                if self.from_south.len() < 8 && self.allow_new_car[2]{
                    self.allow_new_car[2] = false;
                    self.from_south.insert(0, Car::new(coming_from));
                }
            }
            ComingFrom::West => {
                if self.from_west.len() < 8 && self.allow_new_car[3]{
                    self.allow_new_car[3] = false;
                    self.from_west.insert(0, Car::new(coming_from));
                }
            },
            ComingFrom::Random => {
                let car = Car::random(self);
                match car.coming_from {
                    ComingFrom::North => self.from_north.insert(0, car),
                    ComingFrom::East => self.from_east.insert(0, car),
                    ComingFrom::South => self.from_south.insert(0, car),
                    _ => self.from_west.insert(0, car),
                }
            }
        }
    }
}

pub fn traffic(ts: &mut TrafficState) {
 todo!()
}