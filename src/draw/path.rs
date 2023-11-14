use crate::traffic::{Going, Path};
use macroquad::color::{Color, LIME, PURPLE, YELLOW};
use macroquad::prelude::draw_line;

pub fn draw_path(path: &Path) {
    let color = match path.going_to {
        Going::Straight => Color::new(YELLOW.r, YELLOW.g, YELLOW.b, 0.1),
        Going::Right => Color::new(LIME.r, LIME.g, LIME.b, 0.15),
        Going::Left => Color::new(PURPLE.r, PURPLE.g, PURPLE.b, 0.2),
    };

    let points = path.points();
    for i in 0..(points.len() - 1) {
        let start = points[i];
        let end = points[i + 1];
        draw_line(start.x, start.y, end.x, end.y, 5.0, color);
    }
}
