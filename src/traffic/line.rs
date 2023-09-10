use crate::config::{CAR_LENGTH, CAR_SAFE_DISTANCE, WINDOW_SIZE};
use crate::traffic::{Car, Direction, Going, Path};

#[derive(Debug)]
pub enum Light {
    Red,
    Green,
}

#[derive(Debug)]
pub struct Line {
    pub coming_from: Direction,

    pub cars: Vec<Car>,
    pub light: Light,

    pub paths: [Path; 3],
}

fn get_path(paths: &[Path; 3], going: Going) -> &Path {
    match going {
        Going::Straight => &paths[0],
        Going::Left => &paths[1],
        Going::Right => &paths[2],
    }
}

impl Line {
    pub fn new(coming_from: Direction) -> Self {
        Line {
            coming_from,

            cars: Vec::new(),
            light: Light::Red,

            paths: [
                Path::new(coming_from, Going::Straight),
                Path::new(coming_from, Going::Left),
                Path::new(coming_from, Going::Right),
            ],
        }
    }

    pub fn update(&mut self) {
        let mut prev_car = None;

        for car in self.cars.iter_mut() {
            let path = get_path(&self.paths, car.going);

            car.update(path, prev_car);

            prev_car = Some(car);
        }

        self.cleanup_cars();
    }

    pub fn can_add_car(&self) -> bool {
        let prev_car = self
            .cars
            .iter()
            .rfind(|c| c.coming_from == self.coming_from);

        if prev_car.is_none() {
            return true;
        }

        let prev_car = prev_car.unwrap();

        match self.coming_from {
            Direction::North => prev_car.pos.y >= CAR_LENGTH + CAR_SAFE_DISTANCE,
            Direction::East => {
                WINDOW_SIZE as f32 - prev_car.pos.x >= CAR_LENGTH + CAR_SAFE_DISTANCE
            }
            Direction::South => {
                WINDOW_SIZE as f32 - prev_car.pos.y >= CAR_LENGTH + CAR_SAFE_DISTANCE
            }
            Direction::West => prev_car.pos.x >= CAR_LENGTH + CAR_SAFE_DISTANCE,
        }
    }

    pub fn add_car(&mut self) {
        if !self.can_add_car() {
            return;
        }
        let car = Car::new(self.coming_from);

        self.cars.push(car);
    }

    pub fn cleanup_cars(&mut self) {
        self.cars.retain(|car| {
            let path = get_path(&self.paths, car.going);
            !car.is_done(path)
        })
    }
}
