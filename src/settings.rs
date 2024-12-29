use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    pub music_volume: f32,
    pub effect_volume: f32,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            music_volume: 0.5,
            effect_volume: 0.5,
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_effect_volume(&mut self, volume: f32) {
        self.effect_volume = volume.clamp(0.0, 1.0);
    }
}
