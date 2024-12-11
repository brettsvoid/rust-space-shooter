use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_rand::prelude::*;
use rand::prelude::*;

use crate::{
    components::MovementSpeed,
    sprite_animation::{update_animations, AnimationConfig},
};

const SPRITE_SHEET_PATH: &str = "../assets/enemy-small.png";
const SPRITE_SIZE: UVec2 = UVec2::new(17, 16);
const SPRITE_COLUMNS: u32 = 2;
const SPRITE_ROWS: u32 = 1;
const SPRITE_FPS: u8 = 12;

const MAX_ENEMIES: usize = 100;
const ENEMY_SPAWN_CHANCE: u32 = 1;
const ENEMY_SPAWN_DENOMINATOR: u32 = 25;
const ENEMY_GUTTER: f32 = 4.0;
const ENEMY_SPEED: f32 = 100.0;

// This resource tracks the amount of enemies score
#[derive(Resource, Deref, DerefMut)]
pub struct EnemyCount(usize);

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, spawn_enemies)
            .insert_resource(EnemyCount(0))
            .add_systems(
                Update,
                (
                    spawn_enemies,
                    apply_enemy_movement,
                    remove_fallen_enemies,
                    update_animations::<Enemy>,
                ),
            );
    }
}

// Enemy marker component
#[derive(Component)]
struct Enemy;

fn spawn_enemies(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    window: Single<&Window>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Only spawn new enemies if we haven't reached the maximum
    if **enemy_count >= MAX_ENEMIES {
        return;
    }

    // Random chance to spawn a new enemy
    if rng.gen_range(0..ENEMY_SPAWN_DENOMINATOR) > ENEMY_SPAWN_CHANCE {
        return;
    }

    let scale = 2.0;
    let size = SPRITE_SIZE.as_vec2();
    let size_x = size.x * scale;
    let column_count = (window.width() / (size_x + ENEMY_GUTTER)) as u32;
    let column = rng.gen_range(0..column_count);
    let x_pos = calculate_enemy_x_position(&window, column, size_x);
    let spawn_position = Vec3::new(x_pos, window.height() / 2.0 + size_x / 2.0, 1.0);

    let texture =
        asset_server.load_with_settings(SPRITE_SHEET_PATH, |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        });
    let layout =
        TextureAtlasLayout::from_grid(SPRITE_SIZE, SPRITE_COLUMNS, SPRITE_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Enemy,
        Transform::from_translation(spawn_position),
        MovementSpeed(ENEMY_SPEED),
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            custom_size: Some(size * scale),
            ..default()
        },
        AnimationConfig::new(0, 1, SPRITE_FPS),
    ));

    **enemy_count += 1;
}

fn apply_enemy_movement(
    mut query: Populated<(&MovementSpeed, &mut Transform), With<Enemy>>,
    time: Res<Time>,
) {
    for (speed, mut transform) in query.iter_mut() {
        let movement = Vec2::new(0.0, -speed.0 * time.delta_secs());
        transform.translation += movement.extend(0.0);
    }
}

/// Calculate the x position for an enemy in a given column
fn calculate_enemy_x_position(window: &Window, column: u32, size_x: f32) -> f32 {
    let width = window.width();
    let column_count = (width / (size_x + ENEMY_GUTTER)) as u32;
    let gutter_count = column_count - 1;
    let content_width = column_count as f32 * size_x + gutter_count as f32 * ENEMY_GUTTER;
    let margin = (width - content_width) / 2.0;

    (column as f32 * (size_x + ENEMY_GUTTER)) + size_x / 2.0 + margin - width / 2.0
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
