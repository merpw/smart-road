use crate::config::{CAR_PADDING, ROAD_WIDTH, WINDOW_SIZE};
use crate::traffic::{Direction, Going};
use macroquad::math::Vec2;

pub struct Path {
    points: Vec<Vec2>,
}

//                         North
//                     |  ↓  |  ↑  |
//                     |  ↓  |  ↑  |
//                     |     |     |
//                     |     |     |
//                     |     |     |
//                     |     |     |
//      _______________|     |     |_______________
//      ← ←                                     ← ←
// East ---------------             --------------- West
//      → →                                     → →
//      _______________             _______________
//                     |     |     |
//                     |     |     |
//                     |     |     |
//                     |     |     |
//                     |     |     |
//                     |  ↓  |  ↑  |
//                     |  ↓  |  ↑  |
//                         South

impl Path {
    fn start_point(coming_from: Direction) -> Vec2 {
        match coming_from {
            Direction::North => Vec2::new(
                WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 2.0 + CAR_PADDING,
                0.0,
            ),
            Direction::East => {
                Vec2::new(WINDOW_SIZE as f32, WINDOW_SIZE as f32 / 2.0 - CAR_PADDING)
            }
            Direction::South => {
                Vec2::new(WINDOW_SIZE as f32 / 2.0 + CAR_PADDING, WINDOW_SIZE as f32)
            }
            Direction::West => Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 + CAR_PADDING),
        }
    }

    fn end_point(coming_from: Direction, going_to: Going) -> Vec2 {
        let destination = coming_from.destination(going_to);

        match destination {
            Direction::North => Vec2::new(WINDOW_SIZE as f32 / 2.0 + CAR_PADDING, 0.0),
            Direction::East => {
                Vec2::new(WINDOW_SIZE as f32, WINDOW_SIZE as f32 / 2.0 + CAR_PADDING)
            }
            Direction::South => Vec2::new(
                WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 2.0 + CAR_PADDING,
                WINDOW_SIZE as f32,
            ),
            Direction::West => Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 - CAR_PADDING),
        }
    }

    pub fn new(coming_from: Direction, going_to: Going) -> Self {
        let start_point = Self::start_point(coming_from);
        let end_point = Self::end_point(coming_from, going_to);

        match going_to {
            Going::Straight => Self {
                points: vec![start_point, end_point],
            },
            Going::Left => Self {
                points: vec![start_point, end_point],
            },
            Going::Right => Self {
                points: vec![start_point, end_point],
            },
        }
    }

    pub fn points(&self) -> &Vec<Vec2> {
        &self.points
    }
}
