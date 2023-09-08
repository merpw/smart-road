mod car;
mod road;
mod traffic;

use crate::road::render_roads;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

#[macroquad::main("Texture")]
async fn main() {
    // let texture: Texture2D = load_texture("assets/cat.png").await.unwrap();
    set_window_size(WIDTH, HEIGHT);
    loop {
        clear_background(BLACK);
        // draw_texture(&texture, 0., 0., WHITE);

        render_roads();
        next_frame().await
    }
}
