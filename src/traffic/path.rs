use crate::config::{CAR_PADDING, STRAIGHT_LENGTH, WINDOW_SIZE};
use crate::traffic::curve::quadratic_curve;
use crate::traffic::{Direction, Going};
use macroquad::math::Vec2;
use std::ops::{Mul, Sub};

#[derive(Debug)]
pub struct Path {
    pub coming_from: Direction,
    pub going_to: Going,

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
fn straight_point(direction: Direction, border_point: Vec2) -> Vec2 {
    match direction {
        Direction::North => Vec2::new(border_point.x, border_point.y + STRAIGHT_LENGTH),
        Direction::East => Vec2::new(border_point.x - STRAIGHT_LENGTH, border_point.y),
        Direction::South => Vec2::new(border_point.x, border_point.y - STRAIGHT_LENGTH),
        Direction::West => Vec2::new(border_point.x + STRAIGHT_LENGTH, border_point.y),
    }
}

impl Path {
    pub fn new(coming_from: Direction, going_to: Going) -> Self {
        let destination = coming_from.destination(going_to);

        let start_point = border_point(coming_from, true);
        let end_point = border_point(destination, false);

        match going_to {
            Going::Straight => Self {
                coming_from,
                going_to,

                points: vec![start_point, end_point],
            },
            Going::Left | Going::Right => {
                let curve_start_point = straight_point(coming_from, start_point);
                let curve_end_point = straight_point(destination, end_point);

                let center = Vec2::new(WINDOW_SIZE as f32 / 2.0, WINDOW_SIZE as f32 / 2.0);

                let control_point = match going_to {
                    Going::Left => center,
                    Going::Right => {
                        // make curve radius smaller by half

                        // vector between curve_start_point and curve_end_point
                        let line = curve_start_point.sub(curve_end_point);

                        // perpendicular vector from center to line
                        let radial_vector = Vec2::new(-line.y, line.x);

                        center.sub(radial_vector.mul(0.5))
                    }
                    _ => unreachable!(),
                };

                let curve = quadratic_curve(curve_start_point, control_point, curve_end_point);

                Self {
                    coming_from,
                    going_to,

                    points: [start_point, curve_start_point]
                        .into_iter()
                        .chain(curve)
                        .chain([curve_end_point, end_point])
                        .collect(),
                }
            }
        }
    }

    pub fn points(&self) -> &Vec<Vec2> {
        &self.points
    }

    pub fn point(&self, index: usize) -> Option<Vec2> {
        self.points.get(index).copied()
    }
}
