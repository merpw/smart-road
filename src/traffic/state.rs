use crate::traffic::{Direction, Line, Path};

use crate::app::Statistics;
use macroquad::prelude::get_time;
use macroquad::rand::ChooseRandom;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TrafficState {
    //switch_timer: f32,
    pub lines: [Line; 4],

    pub statistics: Statistics,

    pub pause_time: f64,
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            lines: [
                Line::new(Direction::North),
                Line::new(Direction::East),
                Line::new(Direction::South),
                Line::new(Direction::West),
            ],

            statistics: Statistics::default(),

            pause_time: get_time(),
        }
    }

    pub fn toggle_pause(&mut self) {
        self.statistics.is_open = !self.statistics.is_open;

        if self.statistics.is_open {
            self.pause_time = get_time();
            return;
        }

        let pause_duration = get_time() - self.pause_time;

        self.lines.iter_mut().for_each(|line| {
            line.path_cars.iter_mut().for_each(|cars| {
                cars.iter_mut().for_each(|car| {
                    car.start_time += pause_duration;
                });
            });
        });
    }

    pub fn update(&mut self) {
        // TODO: consider performance optimizations

        let mut traffic_state;
        traffic_state = self.clone();

        for i in 0..self.lines.len() {
            self.lines[i].update(&traffic_state);
            traffic_state = self.clone();
        }

        self.statistics.update(&traffic_state);
    }

    pub fn add_car(&mut self, coming_from: Direction) {
        self.statistics.car_count += 1;

        let line = &mut self.lines[coming_from as usize];

        let available_paths = line.get_free_paths();

        if let Some(path) = available_paths.choose() {
            line.add_car(path.clone());
        }
    }

    pub fn add_car_random(&mut self) {
        let available_line_paths: Vec<(usize /* line_index */, Rc<Path>)> = self
            .lines
            .iter()
            .enumerate()
            .flat_map(|(line_index, line)| {
                line.get_free_paths()
                    .iter()
                    .map(|path| (line_index, path.clone()))
                    .collect::<Vec<_>>()
            })
            .collect();

        if let Some((line_index, path)) = available_line_paths.choose() {
            self.lines[*line_index].add_car(path.clone());
            self.statistics.car_count += 1;
        }
    }
}
