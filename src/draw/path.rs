use crate::traffic::{Going, Path};
use macroquad::color::{Color, BLUE, RED, YELLOW};
use macroquad::prelude::draw_line;

pub fn draw_path(path: &Path) {
    let color = match path.going_to {
        Going::Straight => Color::new(BLUE.r, BLUE.g, BLUE.b, 0.5),
        Going::Right => Color::new(YELLOW.r, YELLOW.g, YELLOW.b, 0.5),
        Going::Left => Color::new(RED.r, RED.g, RED.b, 0.5),
    };

    let points = path.points();
    for i in 0..(points.len() - 1) {
        let start = points[i];
        let end = points[i + 1];
        draw_line(start.x, start.y, end.x, end.y, 5.0, color);
    }
}
