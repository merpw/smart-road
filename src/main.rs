#![allow(dead_code)] // TODO: remove
mod config;
mod draw;
mod traffic;

use macroquad::prelude::*;

use config::window_conf;
use draw::draw_roads;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_roads();

        next_frame().await
    }
}
