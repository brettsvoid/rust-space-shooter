use bevy::prelude::*;
use bevy_rand::prelude::*;
use player::PlayerPlugin;
use rand::prelude::*;

mod components;
mod player;
mod sprite_animation;
mod stepping;

const ENEMY_SIZE: f32 = 30.;

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const ENEMY_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EntropyPlugin::<WyRand>::default()))
        .add_plugins(PlayerPlugin)
        .add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(Update)
                .add_schedule(FixedUpdate)
                .at(Val::Percent(35.0), Val::Percent(50.0)),
        )
        .insert_resource(Score(0))
        .insert_resource(EnemyCount(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                //move_paddle,
                //check_for_collisions,
                //play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(
            Update,
            (move_enemies, remove_fallen_enemies, update_scoreboard),
        )
        .run();
}

#[derive(Component)]
struct Enemy;

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
struct EnemyCount(usize);

#[derive(Component)]
struct Bullet;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

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

const MAX_ENEMIES: usize = 10;
const ENEMY_SPAWN_CHANCE: u32 = 1;
const ENEMY_SPAWN_DENOMINATOR: u32 = 25;
const ENEMY_GUTTER: f32 = 4.0;
const ENEMY_FALL_SPEED: f32 = -100.0;

/// Calculate the x position for an enemy in a given column
fn calculate_enemy_x_position(window: &Window, column: u32) -> f32 {
    let width = window.width();
    let column_count = (width / (ENEMY_SIZE + ENEMY_GUTTER)) as u32;
    let gutter_count = column_count - 1;
    let content_width = column_count as f32 * ENEMY_SIZE + gutter_count as f32 * ENEMY_GUTTER;
    let margin = (width - content_width) / 2.0;

    (column as f32 * (ENEMY_SIZE + ENEMY_GUTTER)) + ENEMY_SIZE / 2.0 + margin - width / 2.0
}

/// Spawn a new enemy at the given position
fn spawn_enemy(commands: &mut Commands, position: Vec3) -> Entity {
    commands
        .spawn((
            Sprite::from_color(ENEMY_COLOR, Vec2::ONE),
            Transform::from_translation(position).with_scale(Vec2::splat(ENEMY_SIZE).extend(1.0)),
            Enemy,
            Velocity(Vec2::new(0.0, ENEMY_FALL_SPEED)),
            Collider,
        ))
        .id()
}

fn move_enemies(
    mut enemy_count: ResMut<EnemyCount>,
    mut commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    window: Single<&Window>,
) {
    // Only spawn new enemies if we haven't reached the maximum
    if **enemy_count >= MAX_ENEMIES {
        return;
    }

    // Random chance to spawn a new enemy
    if rng.gen_range(0..ENEMY_SPAWN_DENOMINATOR) > ENEMY_SPAWN_CHANCE {
        return;
    }

    let column_count = (window.width() / (ENEMY_SIZE + ENEMY_GUTTER)) as u32;
    let column = rng.gen_range(0..column_count);
    let x_pos = calculate_enemy_x_position(&window, column);
    let spawn_position = Vec3::new(x_pos, window.height() / 2.0 + ENEMY_SIZE / 2.0, 1.0);

    spawn_enemy(&mut commands, spawn_position);
    **enemy_count += 1;
}

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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn remove_fallen_enemies(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    query: Query<(Entity, &Transform), With<Enemy>>,
    window: Single<&Window>,
) {
    for (entity, transform) in &query {
        if transform.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            **enemy_count -= 1;
        }
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}
