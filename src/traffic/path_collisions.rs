use crate::traffic::car::CarStatus::Turning;
use crate::traffic::{Car, Direction, Going, Line, Path, TrafficState};
use std::rc::Rc;

impl Path {
    /// Returns all paths that collide with this path and has a car in the intersection
    pub fn get_used_collision_paths(&self, traffic_state: &TrafficState) -> Vec<Rc<Path>> {
        self.get_collision_paths(traffic_state)
            .iter()
            .filter(|path| {
                let cars = traffic_state.lines[path.coming_from as usize].path_cars(path);
                cars.iter().any(|car| car.get_status() == Turning)
            })
            .cloned()
            .collect()
    }

    /// Returns all paths that collide with this path
    fn get_collision_paths(&self, traffic_state: &TrafficState) -> Vec<Rc<Path>> {
        let all_paths = traffic_state
            .lines
            .iter()
            .flat_map(|line| line.paths.iter())
            .filter(|path| {
                if path.going_to == Going::Right {
                    return false;
                }

                if path.coming_from == self.coming_from {
                    // same line
                    return false;
                }

                true
            })
            .cloned()
            .collect::<Vec<_>>();

        match (self.coming_from, self.going_to) {
            (_, Going::Right) => vec![],
            _ => all_paths,
        }
    }
}
