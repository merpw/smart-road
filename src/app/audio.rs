use macroquad::audio::{play_sound, PlaySoundParams, Sound};

pub fn play_soundtrack(soundtrack: &Sound) {
    let soundtrack_params = PlaySoundParams {
        looped: true,
        volume: 0.5,
    };
    play_sound(soundtrack, soundtrack_params);
}
