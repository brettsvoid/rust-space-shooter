use bevy::prelude::*;
use bevy_dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy_rand::prelude::*;
use enemies::EnemiesPlugin;
use player::PlayerPlugin;

mod background;
mod components;
mod enemies;
mod player;
mod sprite_animation;
mod stepping;

use background::BackgroundPlugin;

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0); // Changed to black since we'll use shader
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EntropyPlugin::<WyRand>::default()))
        .add_plugins(FpsOverlayPlugin::default())
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        .insert_resource(Score(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(Update, (update_scoreboard, handle_exit))
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

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreboardUi;

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Sound
    // let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    // commands.insert_resource(CollisionSound(ball_collision_sound));

    // Scoreboard
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

fn handle_exit(keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.pressed(KeyCode::KeyQ) {
        std::process::exit(0);
    }
}
