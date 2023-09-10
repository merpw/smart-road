use crate::config::{
    CAR_LENGTH, CAR_SAFE_DISTANCE, CAR_SPEED, STRAIGHT_LENGTH, TOP_LEFT, TOP_RIGHT, WINDOW_SIZE,
};
use crate::traffic::Path;
use macroquad::math::Vec2;
use rand::Rng;
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

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

#[derive(Debug)]
pub struct Car {
    pub coming_from: Direction,
    pub going: Going,

    /// Position of the front left corner of the car
    pub pos: Vec2,

    /// Rotation of the car in radians, 0 is facing right
    pub rotation: f32,

    index: usize,
}

impl Car {
    pub fn new(coming_from: Direction) -> Car {
        let going = GOING[rand::thread_rng().gen_range(0..GOING.len())];

        let path = Path::new(coming_from, going);

        let first_point = path.point(0).unwrap();

        Self {
            coming_from,
            going,
            index: 0,

            pos: first_point,
            rotation: 0.0,
        }
    }

    pub fn border_distance(&self) -> f32 {
        match self.coming_from {
            Direction::North => self.pos.y,
            Direction::East => WINDOW_SIZE as f32 - self.pos.x,
            Direction::South => WINDOW_SIZE as f32 - self.pos.y,
            Direction::West => self.pos.x,
        }
    }

    pub fn update(&mut self, path: &Path, next_car: Option<&Car>) {
        let next_point = path.point(self.index + 1);

        if next_point.is_none() {
            return;
        }

        if let Some(next_car) = next_car {
            let distance = (next_car.pos - self.pos).length() - CAR_LENGTH;

            if distance < CAR_SAFE_DISTANCE {
                return;
            }
        }

        let vector = next_point.unwrap() - self.pos;

        if vector.length() < CAR_SPEED * 1.0 {
            self.index += 1;
            self.update(path, next_car);
            return;
        }

        self.rotation = vector.y.atan2(vector.x);

        let vector = vector.normalize();

        self.pos += vector * CAR_SPEED;
    }

    pub fn is_done(&self, path: &Path) -> bool {
        path.point(self.index + 1).is_none()
    }
}
