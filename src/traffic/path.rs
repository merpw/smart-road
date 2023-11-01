use crate::config::{CAR_PADDING, ROAD_WIDTH, STRAIGHT_LENGTH, WINDOW_SIZE};
use crate::traffic::curve::quadratic_curve;
use crate::traffic::{Direction, Going};
use macroquad::math::Vec2;
use std::ops::{Mul, Sub};

#[derive(Debug, Clone)]
pub struct Path {
    pub coming_from: Direction,
    pub going_to: Going,

    points: Vec<Vec2>,
}

//                |   |   |   |   | ^ |   |
//                |   |   |   |   | ^ |   |
//                |   |   |   |   | ^ |   |
//                |r  | s | l |   | ^ |   |
// _______________| ← | ↓ | → |   | ^ |   |________________
//                            |     ^      ↑ r
// _______________            |     ^      ________________
//                            |     ^      ← s
// _______________            |     ^      ________________
//  < < < < < < < < < < < < < < <   ^      ↓ l
// ___________________________|_^___^______________________
//            l ↑             | ^   ^
// _______________            | ^   ^      ________________
//            s →             | ^   ^
// _______________            | ^   ^      ________________
//            r ↓             | ^   ^   > > > > > > > > > >
// _______________            | 1   2   3  ________________
//                |   |   |   | ← | ↑ | → |
//                |   |   |   | l | s | r |
//                |   |   |   |   |   |   |
//                |   |   |   |   |   |   |
//                |   |   |   |   |   |   |
//                |   |   |   |   |   |   |
//
// ( r = right, s = straight, l = left )
//
// Example:
// departure from South:
// first lane - direction: West
// second lane - direction: North
// third lane - direction: East

/// Returns the point on the border where the car should appear or disappear
fn border_point(coming_from: Direction, going_to: Going) -> Vec2 {
    let lane = (coming_from, going_to);

    match lane {
        // NORTH -------------------------------------------------------------------------------------
        (Direction::North, Going::Right) => Vec2::new(
            WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 2.0 + CAR_PADDING,
            0.0,
        ), // YELLOW
        (Direction::North, Going::Straight) => {
            Vec2::new(WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 4.0, 0.0)
        } // BLUE
        (Direction::North, Going::Left) => Vec2::new(WINDOW_SIZE as f32 / 2.0 - CAR_PADDING, 0.0), // RED

        // EAST --------------------------------------------------------------------------------------
        (Direction::East, Going::Right) => Vec2::new(
            WINDOW_SIZE as f32,
            WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 2.0 + CAR_PADDING,
        ),
        (Direction::East, Going::Straight) => Vec2::new(
            WINDOW_SIZE as f32,
            WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 4.0,
        ),
        (Direction::East, Going::Left) => {
            Vec2::new(WINDOW_SIZE as f32, WINDOW_SIZE as f32 / 2.0 - CAR_PADDING)
        }

        // SOUTH -------------------------------------------------------------------------------------
        (Direction::South, Going::Left) => {
            Vec2::new(WINDOW_SIZE as f32 / 2.0 + CAR_PADDING, WINDOW_SIZE as f32)
        }
        (Direction::South, Going::Straight) => Vec2::new(
            WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 4.0,
            WINDOW_SIZE as f32,
        ),
        (Direction::South, Going::Right) => Vec2::new(
            WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 2.0 - CAR_PADDING,
            WINDOW_SIZE as f32,
        ),

        // WEST --------------------------------------------------------------------------------------
        (Direction::West, Going::Left) => Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 + CAR_PADDING),
        (Direction::West, Going::Straight) => {
            Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 4.0)
        }
        (Direction::West, Going::Right) => Vec2::new(
            0.0,
            WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 2.0 - CAR_PADDING,
        ),
    }
}

fn border_end_point(coming_from: Direction, going_to: Going) -> Vec2 {
    let car_padding = CAR_PADDING;

    let lane = (coming_from, going_to);

    match lane {
        // NORTH -------------------------------------------------------------------------------------
        (Direction::North, Going::Right) => Vec2::new(
            WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 2.0 - car_padding,
            0.0,
        ), // YELLOW
        (Direction::North, Going::Straight) => {
            Vec2::new(WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 4.0, 0.0)
        } // BLUE
        (Direction::North, Going::Left) => Vec2::new(WINDOW_SIZE as f32 / 2.0 + car_padding, 0.0), // RED

        // EAST --------------------------------------------------------------------------------------
        (Direction::East, Going::Right) => Vec2::new(
            WINDOW_SIZE as f32,
            WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 2.0 - car_padding,
        ),
        (Direction::East, Going::Straight) => Vec2::new(
            WINDOW_SIZE as f32,
            WINDOW_SIZE as f32 / 2.0 + ROAD_WIDTH / 4.0,
        ),
        (Direction::East, Going::Left) => {
            Vec2::new(WINDOW_SIZE as f32, WINDOW_SIZE as f32 / 2.0 + car_padding)
        }

        // SOUTH -------------------------------------------------------------------------------------
        (Direction::South, Going::Left) => {
            Vec2::new(WINDOW_SIZE as f32 / 2.0 - car_padding, WINDOW_SIZE as f32)
        }
        (Direction::South, Going::Straight) => Vec2::new(
            WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 4.0,
            WINDOW_SIZE as f32,
        ),
        (Direction::South, Going::Right) => Vec2::new(
            WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 2.0 + car_padding,
            WINDOW_SIZE as f32,
        ),

        // WEST --------------------------------------------------------------------------------------
        (Direction::West, Going::Left) => Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 - car_padding),
        (Direction::West, Going::Straight) => {
            Vec2::new(0.0, WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 4.0)
        }
        (Direction::West, Going::Right) => Vec2::new(
            0.0,
            WINDOW_SIZE as f32 / 2.0 - ROAD_WIDTH / 2.0 + car_padding,
        ),
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

        let start_point = border_point(coming_from, going_to);
        let end_point = border_end_point(destination, going_to);

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

                // vector between curve_start_point and curve_end_point
                let line = curve_start_point.sub(curve_end_point);

                // perpendicular vector from center to line
                let radial_vector = Vec2::new(-line.y, line.x);
                let control_point = match going_to {
                    Going::Left => center.sub(radial_vector.mul(0.15)),
                    Going::Right => center.sub(radial_vector.mul(3.5)),
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
