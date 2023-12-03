use crate::app::audio::play_soundtrack;
use crate::app::control::*;
use crate::draw::*;
use crate::traffic::TrafficState;
use macroquad::audio::*;
use macroquad::prelude::*;
use std::path::PathBuf;

pub struct App {
    pub traffic_state: TrafficState,
    pub background_texture: Texture2D,
    pub background_statistics_texture: Texture2D,
    pub car_textures: (Texture2D, Texture2D, Texture2D),
    pub font: Option<Font>,
    pub soundtrack: Sound,
}

impl App {
    pub async fn new() -> Self {
        let traffic_state = TrafficState::new();
        let background_texture = load_texture_from_assets("background.png").await.unwrap();
        let background_statistics_texture = load_texture_from_assets("background_statistics.png")
            .await
            .unwrap();
        let car_textures = (
            load_texture_from_assets("car_yellow.png").await.unwrap(),
            load_texture_from_assets("car_green.png").await.unwrap(),
            load_texture_from_assets("car_violet.png").await.unwrap(),
        );
        let font = load_ttf_font("./assets/PlaypenSans.ttf").await.ok();
        let soundtrack = load_sound("./assets/christmas_soundtrack.wav")
            .await
            .expect("Failed to load soundtrack");

        Self {
            traffic_state,
            background_texture,
            background_statistics_texture,
            car_textures,
            font,
            soundtrack,
        }
    }

    pub async fn run(&mut self) {
        play_soundtrack(&self.soundtrack);
        loop {
            handle_input(&mut self.traffic_state);

            if self.traffic_state.statistics.is_open {
                draw_statistics(
                    &self.traffic_state.statistics,
                    &self.background_statistics_texture,
                    self.font.as_ref(),
                );
                next_frame().await;
                continue;
            }

            self.traffic_state.update();

            draw_background(&self.background_texture);

            for line in self.traffic_state.lines.iter() {
                for path in line.paths.iter() {
                    draw_path(path);

                    line.path_cars(path).iter().for_each(|car| {
                        draw_car(car, &self.car_textures);
                    });
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
