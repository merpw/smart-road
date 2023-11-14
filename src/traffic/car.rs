use crate::config::{CAR_LENGTH, CAR_SAFE_DISTANCE, MAX_CAR_SPEED, WINDOW_SIZE};
use crate::traffic::{Path, TrafficState};
use macroquad::math::Vec2;
use std::rc::Rc;

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
    Straight = 0,
    Right = 1,
    Left = 2,
}

#[derive(Debug, Clone)]
pub struct Car {
    pub path: Rc<Path>,

    pub velocity: f32,

    /// Position of the front left corner of the car
    pub pos: Vec2,

    /// Rotation of the car in radians, 0 is facing right
    pub rotation: f32,

    point_index: usize,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum CarStatus {
    BeforeTurn,
    Turning,
    AfterTurn,
}

impl Car {
    pub fn new(path: Rc<Path>) -> Car {
        let first_point = path.point(0).unwrap();

        Self {
            path,
            point_index: 0,

            pos: first_point,
            rotation: 0.0,
            velocity: MAX_CAR_SPEED,
        }
    }

    pub fn border_distance(&self) -> f32 {
        match self.path.coming_from {
            Direction::North => self.pos.y,
            Direction::East => WINDOW_SIZE as f32 - self.pos.x,
            Direction::South => WINDOW_SIZE as f32 - self.pos.y,
            Direction::West => self.pos.x,
        }
    }

    pub fn get_move_vector(&self) -> Option<Vec2> {
        let next_point = self.path.point(self.point_index + 1);

        next_point.map(|next_point| next_point - self.pos)
    }

    pub fn get_status(&self) -> CarStatus {
        let total_points = self.path.points().len();

        match self.point_index {
            0 => CarStatus::BeforeTurn,
            i if i < total_points - 1 => CarStatus::Turning,
            _ => CarStatus::AfterTurn,
        }
    }

    pub fn update(&mut self, prev_car: Option<&Car>, traffic_state: &TrafficState) {
        if let Some(move_vector) = self.get_move_vector() {
            if move_vector.length() < self.velocity * 1.0 {
                self.point_index += 1;
                self.update(prev_car, traffic_state);
                return;
            }

            self.velocity = self.calculate_velocity(prev_car, traffic_state);

            let move_vector = move_vector.normalize();

            self.rotation = move_vector.y.atan2(move_vector.x);
            self.pos += move_vector * self.velocity;
        }
    }

    pub fn calculate_velocity(&self, prev_car: Option<&Car>, traffic_state: &TrafficState) -> f32 {
        if let Some(prev_car) = prev_car {
            let distance = (prev_car.pos - self.pos).length() - CAR_LENGTH;

            if distance < CAR_SAFE_DISTANCE {
                return 0.0;
            }
        }

        if self.get_status() == CarStatus::Turning {
            return 0.0;
        }

        MAX_CAR_SPEED
    }

    pub fn is_done(&self) -> bool {
        self.path.point(self.point_index + 1).is_none()
    }
}
