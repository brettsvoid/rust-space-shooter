pub struct Settings {
    pub music_volume: f32,
    pub sound_volume: f32,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            music_volume: 1.0,
            sound_volume: 1.0,
        }
    }
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume;
    }
    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume;
    }
}
