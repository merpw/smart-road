use macroquad::color::Color;
use macroquad::window::Conf;

pub const WINDOW_SIZE: i32 = 1000;
pub const ROAD_WIDTH: f32 = 500.0;

pub const ROAD_LINE_WIDTH: f32 = 5.0;

pub const ROAD_COLOR: Color = Color::new(0.267, 0.294, 0.325, 1.0);
pub const ROAD_LINE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);

pub const CAR_WIDTH: f32 = 30.0;
pub const CAR_LENGTH: f32 = 50.0;

pub const CAR_PADDING: f32 = (ROAD_WIDTH / 2.0 - CAR_WIDTH) / 4.0;
pub const CAR_SAFE_DISTANCE: f32 = 50.0;
pub const MAX_CAR_SPEED_AFTER_TURN: f32 = 4.0;

pub const MAX_CAR_SPEED_BEFORE_TURN: f32 = 3.0;

pub const CAR_ACCELERATION: f32 = 0.1;
pub const CAR_BREAKING_ACCELERATION: f32 = 0.5;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection | Grit:lab".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

// Helper constants

pub const STRAIGHT_LENGTH: f32 = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0;
/*
// center square corners
pub const TOP_LEFT: Vec2 = Vec2::new(STRAIGHT_LENGTH, STRAIGHT_LENGTH);

pub const TOP_RIGHT: Vec2 = Vec2::new(WINDOW_SIZE as f32 - STRAIGHT_LENGTH, STRAIGHT_LENGTH);

pub const BOTTOM_LEFT: Vec2 = Vec2::new(STRAIGHT_LENGTH, WINDOW_SIZE as f32 - STRAIGHT_LENGTH);

pub const BOTTOM_RIGHT: Vec2 = Vec2::new(
    WINDOW_SIZE as f32 - STRAIGHT_LENGTH,
    WINDOW_SIZE as f32 - STRAIGHT_LENGTH,
);
*/
