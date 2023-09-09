use crate::config::{CAR_SPEED, WINDOW_SIZE};
use rand::Rng;
use std::f32::consts::PI;

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
    pub x: f32,
    pub y: f32,

    /// Rotation of the car in radians, 0 is facing right
    pub rotation: f32,

    pub moving: bool,
}

impl Car {
    pub fn new(coming_from: Direction) -> Car {
        let going = GOING[rand::thread_rng().gen_range(0..GOING.len())];

        match coming_from {
            Direction::North => Car {
                coming_from,
                going,
                moving: true,

                x: WINDOW_SIZE as f32 / 2.0,
                y: 0.0,

                rotation: PI / 2.0,
            },
            Direction::East => Car {
                coming_from,
                going,
                moving: true,

                x: WINDOW_SIZE as f32,
                y: WINDOW_SIZE as f32 / 2.0,

                rotation: PI,
            },
            Direction::South => Car {
                coming_from,
                going,
                moving: true,

                x: WINDOW_SIZE as f32 / 2.0,
                y: WINDOW_SIZE as f32,

                rotation: 3.0 * PI / 2.0,
            },
            Direction::West => Car {
                coming_from,
                going,
                moving: true,

                x: 0.0,
                y: WINDOW_SIZE as f32 / 2.0,

                rotation: 0.0,
            },
        }
    }
    pub fn update(&mut self) {
        match self.coming_from {
            Direction::North => {
                if self.y >= 500.0 {
                    match self.going {
                        Going::Straight => self.down(),
                        Going::Right => self.left(),
                        Going::Left => self.right(),
                    }
                } else {
                    self.down();
                }
            }
            Direction::East => {
                if self.x <= 500.0 {
                    match self.going {
                        Going::Straight => self.left(),
                        Going::Right => self.up(),
                        Going::Left => self.down(),
                    }
                } else {
                    self.left()
                }
            }
            Direction::South => {
                if self.y <= 500.0 {
                    match self.going {
                        Going::Straight => self.up(),
                        Going::Right => self.right(),
                        Going::Left => self.left(),
                    }
                } else {
                    self.up();
                }
            }
            _ => {
                if self.x >= 500.0 {
                    match self.going {
                        Going::Straight => self.right(),
                        Going::Right => self.down(),
                        Going::Left => self.up(),
                    }
                } else {
                    self.right()
                }
            }
        }
    }

    fn up(&mut self) {
        self.y -= CAR_SPEED
    }
    fn down(&mut self) {
        self.y += CAR_SPEED
    }
    fn right(&mut self) {
        self.x += CAR_SPEED
    }
    fn left(&mut self) {
        self.x -= CAR_SPEED
    }
}
