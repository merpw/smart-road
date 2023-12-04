use crate::traffic::{Going, Path};
use macroquad::color::Color;
use macroquad::prelude::draw_line;

pub fn draw_path(path: &Path) {
    let color = match path.going_to {
        Going::Straight => Color::from_rgba(186, 255, 241, 0), //change 0 to 100 to see the path
        Going::Right => Color::from_rgba(253, 233, 171, 0),    //change 0 to 100 to see the path
        Going::Left => Color::from_rgba(246, 174, 158, 0),     //change 0 to 100 to see the path
    };

    let points = path.points();
    for i in 0..(points.len() - 1) {
        let start = points[i];
        let end = points[i + 1];
        draw_line(start.x, start.y, end.x, end.y, 5.0, color);
    }
}
