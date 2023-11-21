use crate::config::{CAR_LENGTH, CAR_SAFE_DISTANCE};
use crate::traffic::{Car, Direction, Going, Path, TrafficState};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Line {
    pub coming_from: Direction,

    pub paths: [Rc<Path>; 3],

    pub path_cars: [Vec<Car>; 3],
}

impl Line {
    pub fn new(coming_from: Direction) -> Self {
        Line {
            coming_from,

            paths: [
                Rc::new(Path::new(coming_from, Going::Straight)),
                Rc::new(Path::new(coming_from, Going::Left)),
                Rc::new(Path::new(coming_from, Going::Right)),
            ],

            path_cars: [vec![], vec![], vec![]],
        }
    }
    pub fn path_cars(&self, path: &Path) -> &Vec<Car> {
        &self.path_cars[path.going_to as usize]
    }

    pub fn path_cars_mut(&mut self, path: &Path) -> &mut Vec<Car> {
        &mut self.path_cars[path.going_to as usize]
    }

    pub fn update(&mut self, traffic_state: &TrafficState) {
        for path in self.paths.iter() {
            let cars = &mut self.path_cars[path.going_to as usize];

            let mut prev_car: Option<&Car> = None;

            for car in cars.iter_mut() {
                car.update(prev_car, traffic_state);

                prev_car = Some(car);
            }
        }

        self.cleanup_cars();
    }

    pub fn get_free_paths(&self) -> Vec<Rc<Path>> {
        self.paths
            .iter()
            .filter(|path| {
                let cars = self.path_cars(path);

                if let Some(car) = cars.last() {
                    if car.border_distance() < CAR_LENGTH + CAR_SAFE_DISTANCE {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    pub fn add_car(&mut self, path: Rc<Path>) {
        let car = Car::new(path.clone());

        self.path_cars_mut(&path).push(car);
    }

    pub fn cleanup_cars(&mut self) {
        self.path_cars.iter_mut().for_each(|cars| {
            cars.retain(|car| !car.is_done());
        });
    }
}
