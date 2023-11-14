use macroquad::prelude::*;

pub fn draw_background(background_texture: &Texture2D) {
    draw_texture(background_texture, 0.0, 0.0, WHITE);
}
