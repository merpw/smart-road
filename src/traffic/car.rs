use crate::config::{BUFFER_DISTANCE, CAR_LENGTH, CAR_SAFE_DISTANCE, MAX_CAR_SPEED, WINDOW_SIZE};
use crate::traffic::{Path, TrafficState};
use macroquad::math::Vec2;
use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn destination(&self, going_to: Going) -> Direction {
        match (self, going_to) {
            (Direction::North, Going::Straight) => Direction::South,
            (Direction::North, Going::Left) => Direction::East,
            (Direction::North, Going::Right) => Direction::West,

            (Direction::East, Going::Straight) => Direction::West,
            (Direction::East, Going::Left) => Direction::South,
            (Direction::East, Going::Right) => Direction::North,

            (Direction::South, Going::Straight) => Direction::North,
            (Direction::South, Going::Left) => Direction::West,
            (Direction::South, Going::Right) => Direction::East,

            (Direction::West, Going::Straight) => Direction::East,
            (Direction::West, Going::Left) => Direction::North,
            (Direction::West, Going::Right) => Direction::South,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Going {
    Straight,
    Right,
    Left,
}

pub const GOING: [Going; 3] = [Going::Straight, Going::Right, Going::Left];

#[derive(Debug, Clone)]
pub struct Car {
    pub coming_from: Direction,
    pub going: Going,
    pub velocity: f32,

    /// Position of the front left corner of the car
    pub pos: Vec2,

    /// Rotation of the car in radians, 0 is facing right
    pub rotation: f32,

    point_index: usize,
}

impl Car {
    pub fn new(coming_from: Direction) -> Car {
        let going = GOING[rand::thread_rng().gen_range(0..GOING.len())];

        let path = Path::new(coming_from, going);

        let first_point = path.point(0).unwrap();

        Self {
            coming_from,
            going,
            point_index: 0,

            pos: first_point,
            rotation: 0.0,
            velocity: MAX_CAR_SPEED,
        }
    }
    /*
    pub fn is_in_queue(&self) -> bool {
        self.point_index == 0
    }
     */

    pub fn border_distance(&self) -> f32 {
        match self.coming_from {
            Direction::North => self.pos.y,
            Direction::East => WINDOW_SIZE as f32 - self.pos.x,
            Direction::South => WINDOW_SIZE as f32 - self.pos.y,
            Direction::West => self.pos.x,
        }
    }

    pub fn update(&mut self, path: &Path, prev_car: Option<&Car>, traffic_state: &TrafficState) {
        let next_point = path.point(self.point_index + 1);

        if next_point.is_none() {
            return;
        }

        if let Some(prev_car) = prev_car {
            let distance = (prev_car.pos - self.pos).length() - CAR_LENGTH;

            if distance < CAR_SAFE_DISTANCE {
                return;
            }
        }

        let vector = next_point.unwrap() - self.pos;

        if vector.length() < self.velocity * 1.0 {
            self.point_index += 1;
            self.update(path, prev_car, traffic_state);
            return;
        }

        let collision_cars = path.get_collision_cars(traffic_state);

        for car in collision_cars {
            if car.coming_from == self.coming_from {
                continue;
            }

            let collision_vector = car.pos - self.pos;

            // check if collision_vector is in the same direction as vector

            let dot = vector.dot(collision_vector);
            if dot > 0.0 {
                continue;
            }

            let distance = collision_vector.length() - CAR_LENGTH;

            if distance < BUFFER_DISTANCE {
                self.velocity = 0.0;
                return;
            }
        }

        self.velocity = MAX_CAR_SPEED;

        self.rotation = vector.y.atan2(vector.x);

        let vector = vector.normalize();

        self.pos += vector * self.velocity;
    }

    // Obstacles coming from car's right are priority, the car gives way to them
    // pub fn update_velocity(&mut self, collision_cars: &[&Car]) {}

    pub fn is_done(&self, path: &Path) -> bool {
        path.point(self.point_index + 1).is_none()
    }
}
