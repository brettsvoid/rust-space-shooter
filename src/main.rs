use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy_rand::prelude::*;
use enemies::EnemiesPlugin;
use player::PlayerPlugin;
use scoreboard::ScoreboardPlugin;

mod background;
mod components;
mod enemies;
mod player;
mod scoreboard;
mod sprite_animation;
mod stepping;

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Changed to black since we'll use shader

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EntropyPlugin::<WyRand>::default()))
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(ScoreboardPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(Update, handle_exit)
        .run();
}

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
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
