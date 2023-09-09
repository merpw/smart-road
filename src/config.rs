use macroquad::color::Color;
use macroquad::window::Conf;

pub const WINDOW_WIDTH: i32 = 1000;
pub const WINDOW_HEIGHT: i32 = 1000;

pub const ROAD_WIDTH: f32 = 200.0;

pub const ROAD_LINE_WIDTH: f32 = 5.0;

pub const ROAD_COLOR: Color = Color::new(0.267, 0.294, 0.325, 1.0);
pub const ROAD_LINE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);

pub const CAR_WIDTH: f32 = 50.0;
pub const CAR_LENGTH: f32 = 100.0;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection | Grit:lab".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
