use crate::app::control::*;
use crate::draw::*;
use crate::traffic::TrafficState;
use crate::STATS;
use macroquad::prelude::*;
use std::path::PathBuf;

pub struct App {
    pub traffic_state: TrafficState,
    pub background_texture: Texture2D,
    pub car_textures: (Texture2D, Texture2D, Texture2D),
}

impl App {
    pub async fn new() -> Self {
        let traffic_state = TrafficState::new();
        let background_texture = load_texture_from_assets("background.png").await.unwrap();
        let car_textures = (
            load_texture_from_assets("car_yellow.png").await.unwrap(),
            load_texture_from_assets("car_green.png").await.unwrap(),
            load_texture_from_assets("car_violet.png").await.unwrap(),
        );

        Self {
            traffic_state,
            background_texture,
            car_textures,
        }
    }

    pub async fn run(&mut self) {
        loop {
            handle_input(&mut self.traffic_state);
            if unsafe { STATS.is_open } {
                draw_statistics();
            } else {
                self.traffic_state.update();

                draw_background(&self.background_texture);

                for line in self.traffic_state.lines.iter() {
                    for path in line.paths.iter() {
                        draw_path(path);
                    }
                }

                for line in self.traffic_state.lines.iter() {
                    for car in line.cars.iter() {
                        draw_car(car, &self.car_textures);
                    }
                }
            }
            next_frame().await;
        }
    }
}

async fn load_texture_from_assets(asset_path: &str) -> Result<Texture2D, macroquad::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets");
    path.push(asset_path);
    load_texture(path.to_str().unwrap()).await
}
