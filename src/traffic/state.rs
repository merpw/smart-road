use crate::traffic::{Direction, Line, Path};

use macroquad::rand::ChooseRandom;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TrafficState {
    //switch_timer: f32,
    pub lines: [Line; 4],
}

impl TrafficState {
    pub fn new() -> TrafficState {
        TrafficState {
            //switch_timer: 0.0,
            lines: [
                Line::new(Direction::North),
                Line::new(Direction::East),
                Line::new(Direction::South),
                Line::new(Direction::West),
            ],
        }
    }

    pub fn update(&mut self) {
        // self.switch_timer += get_frame_time();

        let lines = &self.clone();

        self.lines.iter_mut().for_each(|line| line.update(lines));
    }

    pub fn add_car(&mut self, coming_from: Direction) {
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
        }
    }
}
