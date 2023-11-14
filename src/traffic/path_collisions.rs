use crate::traffic::car::CarStatus::Turning;
use crate::traffic::{Direction, Going, Path, TrafficState};
use std::rc::Rc;

impl Path {
    /// Returns all paths that collide with this path and have a car that is turning (possible collision)
    pub fn get_used_collision_paths(&self, traffic_state: &TrafficState) -> Vec<Rc<Path>> {
        self.get_intersected_paths(traffic_state)
            .iter()
            .filter(|path| {
                let cars = traffic_state.lines[path.coming_from as usize].path_cars(path);
                cars.iter().any(|car| car.get_status() == Turning)
            })
            .cloned()
            .collect()
    }

    /// Returns all paths that collide with this path
    fn get_intersected_paths(&self, traffic_state: &TrafficState) -> Vec<Rc<Path>> {
        let path_definitions: Vec<(Direction, Going)> = match (self.coming_from, self.going_to) {
            (_, Going::Right) => vec![],
            (Direction::South, Going::Left) => {
                vec![
                    (Direction::West, Going::Straight),
                    (Direction::East, Going::Left),
                    (Direction::West, Going::Left),
                    (Direction::North, Going::Straight),
                ]
            }
            (Direction::South, Going::Straight) => {
                vec![
                    (Direction::West, Going::Straight),
                    (Direction::North, Going::Left),
                    (Direction::East, Going::Left),
                    (Direction::East, Going::Straight),
                ]
            }
            (Direction::North, Going::Left) => {
                vec![
                    (Direction::East, Going::Straight),
                    (Direction::West, Going::Left),
                    (Direction::East, Going::Left),
                    (Direction::South, Going::Straight),
                ]
            }
            (Direction::North, Going::Straight) => {
                vec![
                    (Direction::East, Going::Straight),
                    (Direction::South, Going::Left),
                    (Direction::West, Going::Left),
                    (Direction::West, Going::Straight),
                ]
            }
            (Direction::East, Going::Left) => {
                vec![
                    (Direction::South, Going::Straight),
                    (Direction::North, Going::Left),
                    (Direction::South, Going::Left),
                    (Direction::West, Going::Straight),
                ]
            }
            (Direction::East, Going::Straight) => {
                vec![
                    (Direction::South, Going::Straight),
                    (Direction::West, Going::Left),
                    (Direction::North, Going::Left),
                    (Direction::North, Going::Straight),
                ]
            }
            (Direction::West, Going::Left) => {
                vec![
                    (Direction::North, Going::Straight),
                    (Direction::South, Going::Left),
                    (Direction::North, Going::Left),
                    (Direction::East, Going::Straight),
                ]
            }
            (Direction::West, Going::Straight) => {
                vec![
                    (Direction::North, Going::Straight),
                    (Direction::East, Going::Left),
                    (Direction::South, Going::Left),
                    (Direction::South, Going::Straight),
                ]
            }
        };

        traffic_state
            .lines
            .iter()
            .flat_map(|line| line.paths.iter())
            .filter(|path| {
                path_definitions.iter().any(|&(coming_from, going_to)| {
                    path.coming_from == coming_from && path.going_to == going_to
                })
            })
            .cloned()
            .collect()
    }
}
