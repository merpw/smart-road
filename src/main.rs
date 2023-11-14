mod app;
mod config;
mod draw;
mod traffic;

use app::*;
use config::window_conf;
use once_cell::sync::Lazy;

pub static mut STATS: Lazy<Statistics> = Lazy::new(Statistics::default);

#[macroquad::main(window_conf)]
async fn main() {
    let mut smart_road = App::new().await;

    smart_road.run().await;
}
