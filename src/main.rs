use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy_rand::prelude::*;
use game::{GamePlugin, GameRestartEvent};
use game_state::{GameState, GameStatePlugin};
use menu::menu::MenuPlugin;
use settings::Settings;

mod audio;
mod background;
mod collisions;
mod components;
mod enemies;
mod game;
mod game_over;
mod game_state;
mod menu;
mod player;
mod scoreboard;
mod settings;
mod sprite_animation;
mod stepping;
mod systems;

use audio::GameAudioPlugin;

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Changed to black since we'll use shader

fn main() {
    // NOTE: Common resolution that most monitors scale well with is 640x360px
    App::new()
        .add_plugins((DefaultPlugins, EntropyPlugin::<WyRand>::default()))
        .add_plugins(FpsOverlayPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Settings::new())
        .add_plugins(GameAudioPlugin)
        .add_plugins((
            GameStatePlugin,
            MenuPlugin,
            GamePlugin,
            BackgroundPlugin,
            game_over::GameOverPlugin,
        ))
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(Update, handle_exit)
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Sound
    // let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    // commands.insert_resource(CollisionSound(ball_collision_sound));
}

fn handle_exit(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut game_restart_event: EventWriter<GameRestartEvent>,
) {
    if keyboard.just_released(KeyCode::KeyQ) {
        std::process::exit(0);
    }
    if keyboard.just_released(KeyCode::Escape) {
        match current_game_state.get() {
            GameState::Playing => next_game_state.set(GameState::Paused),
            GameState::Paused => next_game_state.set(GameState::Playing),
            _ => (),
        }
    }
    if keyboard.just_released(KeyCode::KeyR) {
        match current_game_state.get() {
            GameState::GameOver => {
                next_game_state.set(GameState::Playing);
                game_restart_event.send_default();
            }
            _ => (),
        }
    }
}
