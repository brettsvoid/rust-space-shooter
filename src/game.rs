use bevy::prelude::*;

use crate::collisions::CollisionsPlugin;
use crate::enemies::EnemiesPlugin;
use crate::explosion::ExplosionPlugin;
use crate::player::PlayerPlugin;
use crate::powerups::PowerupsPlugin;
use crate::scoreboard::ScoreboardPlugin;
use crate::AppState;
//use crate::systems::despawn_screen;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameRestartEvent>()
            .add_systems(OnEnter(AppState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(AppState::Game)))
            .add_plugins((
                ScoreboardPlugin,
                PlayerPlugin,
                EnemiesPlugin,
                CollisionsPlugin,
                PowerupsPlugin,
                ExplosionPlugin,
            ));
        //.add_systems(OnExit(GameState::Playing), despawn_screen::<GameScreen>);
        //TODO: handle exiting to the game to main menu
    }
}

// #[derive(Component)]
// struct GameScreen;

#[derive(Event, Default)]
pub struct GameRestartEvent;

fn game_setup() {}

fn game() {}
