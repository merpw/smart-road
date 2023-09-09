use crate::traffic::{Direction, Going, Path};
use macroquad::color::{Color, BLUE, GREEN, RED, YELLOW};
use macroquad::prelude::draw_line;

pub fn draw_path(path: &Path, color: Color) {
    let points = path.points();
    for i in 0..(points.len() - 1) {
        let start = points[i];
        let end = points[i + 1];
        draw_line(start.x, start.y, end.x, end.y, 5.0, color);
    }
}

pub struct PathDrawer {
    paths: Vec<(Path, Color)>,
}

impl PathDrawer {
    pub fn new() -> Self {
        Self {
            paths: vec![
                (Path::new(Direction::West, Going::Straight), RED),
                (Path::new(Direction::West, Going::Left), RED),
                (Path::new(Direction::West, Going::Right), RED),
                //
                (Path::new(Direction::North, Going::Straight), BLUE),
                (Path::new(Direction::North, Going::Left), BLUE),
                (Path::new(Direction::North, Going::Right), BLUE),
                //
                (Path::new(Direction::East, Going::Straight), GREEN),
                (Path::new(Direction::East, Going::Left), GREEN),
                (Path::new(Direction::East, Going::Right), GREEN),
                //
                (Path::new(Direction::South, Going::Straight), YELLOW),
                (Path::new(Direction::South, Going::Left), YELLOW),
                (Path::new(Direction::South, Going::Right), YELLOW),
            ],
        }
    }

    pub fn draw(&self) {
        for path in &self.paths {
            draw_path(&path.0, path.1);
        }
    }
}
