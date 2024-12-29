use bevy::prelude::*;

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    GameOver,
}
