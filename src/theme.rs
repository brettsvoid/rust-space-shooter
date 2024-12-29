use bevy::color::Color;

pub struct Palette;

impl Palette {
    pub const TEXT_PRIMARY: Color = Color::srgb(0.9, 0.9, 0.9);
    pub const TEXT_GAME_OVER: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const TEXT_PAUSED: Color = Color::srgb(0.6, 0.6, 0.6);
}
