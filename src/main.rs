use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy_rand::prelude::*;
use components::Volume;
use game::GamePlugin;
use game_state::GameStatePlugin;
use menu::menu::MenuPlugin;

mod background;
mod collisions;
mod components;
mod enemies;
mod game;
mod game_state;
mod menu;
mod player;
mod scoreboard;
mod sprite_animation;
mod stepping;
mod systems;

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Changed to black since we'll use shader

fn main() {
    // NOTE: Common resolution that most monitors scale well with is 640x360px
    App::new()
        .add_plugins((DefaultPlugins, EntropyPlugin::<WyRand>::default()))
        .add_plugins(FpsOverlayPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Volume {
            effects: 5,
            music: 5,
        })
        .add_plugins((GameStatePlugin, MenuPlugin, GamePlugin, BackgroundPlugin))
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

fn handle_exit(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.pressed(KeyCode::KeyQ) {
        std::process::exit(0);
    }
}
