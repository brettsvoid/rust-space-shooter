pub struct Settings {
    pub music_volume: f32,
    pub effect_volume: f32,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            music_volume: 1.0,
            effect_volume: 1.0,
        }
    }
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume;
    }
    pub fn set_effect_volume(&mut self, volume: f32) {
        self.effect_volume = volume;
    }
}
