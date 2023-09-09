mod car;
mod config;
mod road;
mod traffic;

use crate::road::render_roads;
use macroquad::prelude::*;

use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    // let texture: Texture2D = load_texture("assets/cat.png").await.unwrap();
    loop {
        clear_background(BLACK);
        // draw_texture(&texture, 0., 0., WHITE);

        render_roads();
        next_frame().await
    }
}
