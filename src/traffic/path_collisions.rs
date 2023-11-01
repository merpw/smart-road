use crate::traffic::line::get_path;
use crate::traffic::{Car, Direction, Going, Path, TrafficState};
use std::mem::swap;

impl Path {
    pub fn get_collision_cars<'a>(&'a self, traffic_state: &'a TrafficState) -> Vec<&Car> {
        // take cars from traffic state for each collision_path

        let all_cars = traffic_state.lines.iter().flat_map(|line| line.cars.iter());

        let collision_paths = self.get_collision_paths(traffic_state);

        all_cars
            .filter(|car| {
                collision_paths
                    .iter()
                    .any(|path| path.coming_from == car.coming_from && path.going_to == car.going)
            })
            .collect()
    }

    fn get_collision_paths<'a>(&'a self, traffic_state: &'a TrafficState) -> Vec<&Path> {
        match (self.coming_from, self.going_to) {
            (_, Going::Right) => vec![],
            // use all paths for each line in the traffic state
            _ => traffic_state
                .lines
                .iter()
                .flat_map(|line| line.paths.iter())
                .collect(),
        }
    }
}

//     if self.going == Going::Right {
//             return;
//         }
//
//         let [north_line, east_line, south_line, west_line] = &traffic_state.lines;
//
//         match self.coming_from {
//             Direction::North => {
//                 let west_obstacle = west_line
//                     .cars
//                     .iter()
//                     .find(|c| c.going != Going::Right && c.pos.x < self.pos.x);
//
//                 let south_obstacle = south_line.cars.iter().find(|c| {
//                     self.going == Going::Left
//                         && c.going == Going::Straight
//                         && c.pos.y + CAR_LENGTH > self.pos.y
//                 });
//
//                 // __________________________________Lane at car's right (Car from North, obstacle from West)
//                 if let Some(obstacle) = west_obstacle {
//                     if self.going == Going::Left && obstacle.going == Going::Straight {
//                         return;
//                     }
//                     if self.pos.y <= obstacle.pos.y
//                         && obstacle.pos.x + BUFFER_DISTANCE >= self.pos.x
//                         && self.pos.y + BUFFER_DISTANCE >= obstacle.pos.y
//                         && self.pos.x < WINDOW_SIZE as f32 / 2.0
//                         && obstacle.pos.y > WINDOW_SIZE as f32 / 2.0 - BUFFER_DISTANCE
//                     {
//                         if obstacle.pos.y - self.pos.y < CAR_SAFE_DISTANCE
//                             && self.going == Going::Left
//                         {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                     // _________Car turning left, Obstacle from South going straight, Obstacle intersecting the car from car's right
//                 } else if let Some(obstacle) = south_obstacle {
//                     if self.pos.x <= obstacle.pos.x
//                         && obstacle.pos.y - BUFFER_DISTANCE <= self.pos.y
//                         && self.pos.y >= WINDOW_SIZE as f32 / 2.0 - BUFFER_DISTANCE
//                         && obstacle.pos.x >= WINDOW_SIZE as f32 / 2.0 - CAR_LENGTH
//                     {
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else {
//                     self.velocity = MAX_CAR_SPEED;
//                 }
//             }
//             Direction::East => {
//                 let north_obstacle = north_line
//                     .cars
//                     .iter()
//                     .find(|c| c.going != Going::Right && c.pos.y - CAR_LENGTH < self.pos.y);
//                 let west_obstacle = west_line.cars.iter().find(|c| {
//                     self.going == Going::Left
//                         && c.going == Going::Straight
//                         && c.pos.x - CAR_LENGTH < self.pos.x
//                 });
//
//                 if let Some(obstacle) = north_obstacle {
//                     if self.going == Going::Left && obstacle.going == Going::Straight {
//                         return;
//                     }
//                     if self.pos.x >= obstacle.pos.x
//                         && obstacle.pos.y + BUFFER_DISTANCE >= self.pos.y
//                         && self.pos.x - BUFFER_DISTANCE <= obstacle.pos.x
//                         && self.pos.y < WINDOW_SIZE as f32 / 2.0
//                     //&& obstacle.pos.y < WINDOW_SIZE as f32 / 2.0 + BUFFER_DISTANCE
//                     {
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else if let Some(obstacle) = west_obstacle {
//                     if self.pos.y <= obstacle.pos.y
//                         && obstacle.pos.x + BUFFER_DISTANCE >= self.pos.x
//                         && self.pos.x <= WINDOW_SIZE as f32 / 2.0 + BUFFER_DISTANCE
//                         && obstacle.pos.x <= WINDOW_SIZE as f32 / 2.0 + CAR_LENGTH
//                     {
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else {
//                     self.velocity = MAX_CAR_SPEED;
//                 }
//             }
//             Direction::South => {
//                 let east_obstacle = east_line
//                     .cars
//                     .iter()
//                     .find(|c| c.going != Going::Right && c.pos.x + CAR_LENGTH > self.pos.x);
//
//                 let north_obstacle = north_line.cars.iter().find(|c| {
//                     self.going == Going::Left
//                         && c.going == Going::Straight
//                         && c.pos.y - CAR_LENGTH < self.pos.y
//                 });
//
//                 if let Some(obstacle) = east_obstacle {
//                     if self.going == Going::Left && obstacle.going == Going::Straight {
//                         return;
//                     }
//                     if self.pos.y >= obstacle.pos.y
//                         && obstacle.pos.x - BUFFER_DISTANCE <= self.pos.x
//                         && self.pos.y - BUFFER_DISTANCE <= obstacle.pos.y
//                         && self.pos.x > WINDOW_SIZE as f32 / 2.0
//                         && obstacle.pos.y < WINDOW_SIZE as f32 / 2.0 + BUFFER_DISTANCE
//                     {
//                         //self.velocity = 1.0;
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE
//                             && self.going != Going::Left
//                             || obstacle.pos.y - self.pos.y < CAR_SAFE_DISTANCE
//                             && self.going == Going::Left
//                         {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else if let Some(obstacle) = north_obstacle {
//                     if self.pos.x >= obstacle.pos.x
//                         && obstacle.pos.y + BUFFER_DISTANCE >= self.pos.y
//                         && self.pos.y <= WINDOW_SIZE as f32 / 2.0 + BUFFER_DISTANCE
//                         && obstacle.pos.x <= WINDOW_SIZE as f32 / 2.0 + CAR_LENGTH
//                     {
//                         //self.velocity = 1.0;
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else {
//                     self.velocity = MAX_CAR_SPEED;
//                 }
//             }
//             Direction::West => {
//                 let south_obstacle = south_line.cars.iter().find(|c| {
//                     //self.going != Going::Right
//                     c.going != Going::Right && c.pos.y + CAR_LENGTH >= self.pos.y
//                 });
//                 let east_obstacle = east_line.cars.iter().find(|c| {
//                     self.going == Going::Left
//                         && c.going == Going::Straight
//                         && c.pos.x + CAR_LENGTH > self.pos.x
//                 });
//
//                 if let Some(obstacle) = south_obstacle {
//                     if self.going == Going::Left && obstacle.going == Going::Straight {
//                         return;
//                     }
//                     if self.pos.x <= obstacle.pos.x
//                         && obstacle.pos.y - BUFFER_DISTANCE <= self.pos.y
//                         && self.pos.x + BUFFER_DISTANCE >= obstacle.pos.x
//                         && self.pos.y > WINDOW_SIZE as f32 / 2.0
//                     //&& obstacle.pos.x > WINDOW_SIZE as f32 / 2.0 - BUFFER_DISTANCE
//                     {
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else if let Some(obstacle) = east_obstacle {
//                     if self.pos.y >= obstacle.pos.y
//                         && obstacle.pos.x - BUFFER_DISTANCE <= self.pos.x
//                         && self.pos.x >= WINDOW_SIZE as f32 / 2.0 - BUFFER_DISTANCE
//                         && obstacle.pos.x >= WINDOW_SIZE as f32 / 2.0 - CAR_LENGTH
//                     {
//                         if obstacle.pos.x - self.pos.x < CAR_SAFE_DISTANCE {
//                             self.velocity = 0.0;
//                         } else {
//                             self.velocity = 1.0;
//                         }
//                     } else {
//                         self.velocity = MAX_CAR_SPEED;
//                     }
//                 } else {
//                     self.velocity = MAX_CAR_SPEED;
//                 }
//             }
//         }
