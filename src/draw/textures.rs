use macroquad::prelude::Texture2D;

pub struct Textures {
    pub light_green: Texture2D,
    pub light_red: Texture2D,
}

impl Textures {
    pub async fn load() -> Self {
        Self {
            light_green: macroquad::texture::load_texture("assets/green-light.png")
                .await
                .unwrap(),
            light_red: macroquad::texture::load_texture("assets/red-light.png")
                .await
                .unwrap(),
        }
    }
}
