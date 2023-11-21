use crate::config::{CAR_LENGTH, CAR_SAFE_DISTANCE};
use crate::traffic::{Car, TrafficState};

#[derive(Debug, Clone, Default)]
pub struct Statistics {
    pub car_count: usize,
    pub max_speed: f32,
    pub min_speed: f32,
    pub max_time: f32,
    pub min_time: f32,

    pub collisions: Vec<(usize, usize)>,
    pub close_calls: Vec<(usize, usize)>,

    pub is_open: bool,
}

impl Statistics {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn update(&mut self, traffic_state: &TrafficState) {
        let cars = traffic_state
            .lines
            .iter()
            .flat_map(|line| line.path_cars.iter())
            .flatten()
            .collect::<Vec<&Car>>();

        for (i, car) in cars.iter().enumerate() {
            if car.velocity > self.max_speed {
                self.max_speed = car.velocity;
            }
            if car.id == 0 && cars.len() == 1 && self.min_speed == 0.0 {
                self.min_speed = self.max_speed;
            }
            if car.velocity < self.min_speed {
                self.min_speed = car.velocity;
            }

            for other_car in cars.iter().skip(i + 1) {
                let distance = (car.pos - other_car.pos).length() - CAR_LENGTH;
                if distance <= 0.0 && !self.collisions.contains(&(car.id, other_car.id)) {
                    self.collisions.push((car.id, other_car.id));
                    self.close_calls.push((car.id, other_car.id));
                }

                if car.path == other_car.path
                    && distance < CAR_SAFE_DISTANCE / 2.0
                    && !self.close_calls.contains(&(car.id, other_car.id))
                {
                    self.close_calls.push((car.id, other_car.id));
                }
            }
        }
    }
}
