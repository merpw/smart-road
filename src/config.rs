use macroquad::window::Conf;

pub const WINDOW_SIZE: i32 = 1000;
pub const ROAD_WIDTH: f32 = 500.0;

pub const CAR_WIDTH: f32 = 30.0;
pub const CAR_LENGTH: f32 = 50.0;

pub const CAR_PADDING: f32 = (ROAD_WIDTH / 2.0 - CAR_WIDTH) / 4.0;
pub const CAR_SAFE_DISTANCE: f32 = 50.0;
pub const BUFFER_DISTANCE: f32 = 50.0;

pub const MAX_CAR_SPEED: f32 = 3.0;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "smart-road | Grit:lab".to_owned(),
        window_width: WINDOW_SIZE,
        window_height: WINDOW_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

// Helper constants

pub const STRAIGHT_LENGTH: f32 = (WINDOW_SIZE as f32 - ROAD_WIDTH) / 2.0;
