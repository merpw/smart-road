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
//      A →            B                        → →
//      _______________    C        _______________
//                     |   | |     |
//                     |   | |     |
//                     |   | |     |
//                     |   | |     |
//                     |   | |     |
//                     |   ↓ |  ↑  |
//                     |   D |  ↑  |
//                         South
// Path(East, Right) =  [A -> B -> C -> D]
// A - border_point(East, true)
// D - border_point(South, false)
// B - straight_point(East, A, STRAIGHT_LENGTH)
// C - straight_point(South, D, STRAIGHT_LENGTH)

const STRAIGHT_LENGTH: f32 = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0;

/// Returns the point on the border where the car should appear or disappear
fn border_point(coming_from: Direction, right_side: bool) -> Vec2 {
    let car_padding = if right_side {
        CAR_PADDING
    } else {
        -CAR_PADDING
    };
    match coming_from {
        Direction::North => Vec2::new(WINDOW_SIZE as f32 / 2.0 - car_padding, 0.0),
        Direction::East => Vec2::new(WINDOW_SIZE as f32, WINDOW_SIZE as f32 / 2.0 - car_padding),
        Direction::South => Vec2::new(WINDOW_SIZE as f32 / 2.0 + car_padding, WINDOW_SIZE as f32),
        Direction::West => Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 + car_padding),
    }
}

/// Returns the point in center associated with the border point
fn straight_point(direction: Direction, border_point: Vec2, straight_length: f32) -> Vec2 {
    match direction {
        Direction::North => Vec2::new(border_point.x, border_point.y + straight_length),
        Direction::East => Vec2::new(border_point.x - straight_length, border_point.y),
        Direction::South => Vec2::new(border_point.x, border_point.y - straight_length),
        Direction::West => Vec2::new(border_point.x + straight_length, border_point.y),
    }
}

impl Path {
    pub fn new(coming_from: Direction, going_to: Going) -> Self {
        let destination = coming_from.destination(going_to);

        let start_point = border_point(coming_from, true);
        let end_point = border_point(destination, false);

        match going_to {
            Going::Straight => Self {
                points: vec![start_point, end_point],
            },
            Going::Left | Going::Right => {
                let start_strength_length = match going_to {
                    Going::Left => STRAIGHT_LENGTH + ROAD_WIDTH / 2.0, // TODO: maybe change
                    _ => STRAIGHT_LENGTH,
                };

                let curve_start_point =
                    straight_point(coming_from, start_point, start_strength_length);

                let curve_end_point = straight_point(destination, end_point, STRAIGHT_LENGTH);

                Self {
                    points: vec![start_point, curve_start_point, curve_end_point, end_point],
                }
            }
        }
    }

    pub fn points(&self) -> &Vec<Vec2> {
        &self.points
    }
}
