use crate::config::{
    CAR_ACCELERATION, CAR_BREAKING_ACCELERATION, CAR_LENGTH, CAR_SAFE_DISTANCE,
    MAX_CAR_SPEED_AFTER_TURN, MAX_CAR_SPEED_BEFORE_TURN, STRAIGHT_LENGTH, WINDOW_SIZE,
};
use crate::traffic::car::CarStatus::BeforeTurn;
use crate::traffic::{Path, TrafficState};
use macroquad::math::Vec2;
use macroquad::prelude::*;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static CAR_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
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
    pub id: usize,

    pub path: Rc<Path>,

    pub velocity: f32,

    /// Position of the front left corner of the car
    pub pos: Vec2,

    /// Rotation of the car in radians, 0 is facing right
    pub rotation: f32,

    point_index: usize,

    pub start_time: f64,
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
            id: CAR_ID.fetch_add(1, Ordering::SeqCst),
            path,
            point_index: 0,
            start_time: get_time(),

            pos: first_point,
            rotation: 0.0,
            velocity: MAX_CAR_SPEED_AFTER_TURN,
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
            self.velocity = self.calculate_velocity(prev_car, traffic_state);

            if move_vector.length() < self.velocity * 1.0 {
                self.point_index += 1;
                self.update(prev_car, traffic_state);
                return;
            }

            let move_vector = move_vector.normalize();

            self.rotation = move_vector.y.atan2(move_vector.x);
            self.pos += move_vector * self.velocity;
        }
    }

    pub fn calculate_velocity(&self, prev_car: Option<&Car>, traffic_state: &TrafficState) -> f32 {
        let velocity = self.velocity.max(1.0);

        if let Some(prev_car) = prev_car {
            let distance = (prev_car.pos - self.pos).length() - CAR_LENGTH;

            if distance < CAR_SAFE_DISTANCE {
                return 0.0;
            }

            if distance < CAR_SAFE_DISTANCE * 2.0 {
                return (velocity * (1.0 - CAR_BREAKING_ACCELERATION))
                    .min(MAX_CAR_SPEED_BEFORE_TURN);
            }
        }

        let status = self.get_status();

        if status != BeforeTurn {
            return (velocity * (1.0 + CAR_ACCELERATION)).min(MAX_CAR_SPEED_AFTER_TURN);
        }

        let collision_paths = self.path.get_potential_collision_paths(traffic_state);

        if !collision_paths.is_empty() {
            let center_distance = STRAIGHT_LENGTH - self.border_distance();

            if center_distance < CAR_SAFE_DISTANCE / 2.0 {
                return 0.0;
            }
            if center_distance < CAR_SAFE_DISTANCE {
                return (velocity * (1.0 - CAR_BREAKING_ACCELERATION))
                    .min(MAX_CAR_SPEED_BEFORE_TURN);
            }
        }

        (velocity * (1.0 + CAR_ACCELERATION)).min(MAX_CAR_SPEED_BEFORE_TURN)
    }

    pub fn is_done(&self) -> bool {
        self.path.point(self.point_index + 1).is_none()
    }
}
