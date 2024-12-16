use bevy::prelude::*;

use crate::collisions::CollisionsPlugin;
use crate::components::Volume;
use crate::enemies::EnemiesPlugin;
use crate::game_state::GameState;
use crate::player::PlayerPlugin;
use crate::scoreboard::ScoreboardPlugin;
use crate::systems::despawn_screen;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Playing)))
            .add_plugins((
                ScoreboardPlugin,
                PlayerPlugin,
                EnemiesPlugin,
                CollisionsPlugin,
            ))
            .add_systems(OnExit(GameState::Playing), despawn_screen::<GameScreen>);
        //TODO: handle exiting to the game to main menu
    }
}

#[derive(Component)]
struct GameScreen;

fn game_setup(mut _commands: Commands, _volume: Res<Volume>) {}

fn game() {}
