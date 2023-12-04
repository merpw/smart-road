use macroquad::prelude::*;

pub fn draw_statistics_background(background_statistics_texture: &Texture2D) {
    draw_texture_ex(
        background_statistics_texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(1000.0, 1000.0)),
            ..Default::default()
        },
    );
}
