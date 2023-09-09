use macroquad::window::Conf;

pub const WINDOW_WIDTH: i32 = 1000;
pub const WINDOW_HEIGHT: i32 = 1000;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection | Grit:lab".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
