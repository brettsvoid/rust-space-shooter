use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_rand::prelude::*;
use rand::prelude::*;

use crate::{
    collisions::Collider,
    components::{Bounds, Health, MovementSpeed},
    game::GameRestartEvent,
    game_state::GameState,
    sprite_animation::{update_animations, AnimationConfig},
};

struct EnemyConfig {
    sprite_path: &'static str,
    sprite_size: UVec2,
    sprite_columns: u32,
    sprite_rows: u32,
    sprite_fps: u8,
    speed: f32,
    scale: f32,
    health: i32,
    spawn_weight: f32,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum EnemyType {
    Small,
    Medium,
    Large,
}
impl EnemyType {
    fn config(&self) -> EnemyConfig {
        match self {
            EnemyType::Small => EnemyConfig {
                sprite_path: "../assets/enemy-small.png",
                sprite_size: UVec2::new(17, 16),
                sprite_columns: 2,
                sprite_rows: 1,
                sprite_fps: 12,
                speed: 100.0,
                scale: 2.0,
                health: 2,
                spawn_weight: 8.0,
            },
            EnemyType::Medium => EnemyConfig {
                sprite_path: "../assets/enemy-medium.png",
                sprite_size: UVec2::new(32, 16),
                sprite_columns: 2,
                sprite_rows: 1,
                sprite_fps: 12,
                speed: 50.0,
                scale: 2.0,
                health: 8,
                spawn_weight: 0.4,
            },
            EnemyType::Large => EnemyConfig {
                sprite_path: "../assets/enemy-large.png",
                sprite_size: UVec2::new(32, 32),
                sprite_columns: 2,
                sprite_rows: 1,
                sprite_fps: 12,
                speed: 25.0,
                scale: 2.0,
                health: 20,
                spawn_weight: 0.1,
            },
        }
    }
}

pub struct EnemyDestroyedData {
    pub enemy_type: EnemyType,
    pub position: Vec3,
}

#[derive(Event)]
pub struct EnemyDestroyedEvent(pub EnemyDestroyedData);

const MAX_ENEMIES: usize = 40;
const ENEMY_SPAWN_CHANCE: u32 = 1;
const ENEMY_SPAWN_DENOMINATOR: u32 = 100; // higher means less enemies
const ENEMY_GUTTER: f32 = 4.0;

// This resource tracks the count of each enemy type
#[derive(Resource)]
pub struct EnemyCount {
    pub small: usize,
    pub medium: usize,
    pub large: usize,
}

impl EnemyCount {
    pub fn total(&self) -> usize {
        self.small + self.medium + self.large
    }

    pub fn increment(&mut self, enemy_type: &EnemyType) {
        match enemy_type {
            EnemyType::Small => self.small += 1,
            EnemyType::Medium => self.medium += 1,
            EnemyType::Large => self.large += 1,
        }
    }

    pub fn decrement(&mut self, enemy_type: &EnemyType) {
        match enemy_type {
            EnemyType::Small => self.small = self.small.saturating_sub(1),
            EnemyType::Medium => self.medium = self.medium.saturating_sub(1),
            EnemyType::Large => self.large = self.large.saturating_sub(1),
        }
    }
}

impl Default for EnemyCount {
    fn default() -> Self {
        Self {
            small: 0,
            medium: 0,
            large: 0,
        }
    }
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, spawn_enemies)
            .init_resource::<EnemyCount>()
            .add_systems(OnEnter(GameState::Playing), enemies_setup)
            .add_systems(
                Update,
                (
                    spawn_enemies,
                    apply_enemy_movement,
                    remove_fallen_enemies,
                    update_animations::<Enemy>,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, reset_enemies);
    }
}

// Enemy marker component
#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

fn enemies_setup(mut enemy_count: ResMut<EnemyCount>) {
    // Reset all counts to 0 when entering Playing state
    *enemy_count = EnemyCount::default();
}

fn spawn_enemies(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    window: Single<&Window>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Only spawn new enemies if we haven't reached the maximum
    if enemy_count.total() >= MAX_ENEMIES {
        return;
    }

    // Random chance to spawn a new enemy
    if rng.gen_range(0..ENEMY_SPAWN_DENOMINATOR) > ENEMY_SPAWN_CHANCE {
        return;
    }

    let enemy_type = {
        let weights = [
            (EnemyType::Large, EnemyType::Large.config().spawn_weight),
            (EnemyType::Medium, EnemyType::Medium.config().spawn_weight),
            (EnemyType::Small, EnemyType::Small.config().spawn_weight),
        ];
        let total_weight: f32 = weights.iter().map(|(_, w)| w).sum();
        let roll = rng.gen::<f32>() * total_weight;

        let mut selected = EnemyType::Small;
        for (enemy_type, weight) in weights.iter() {
            if roll <= *weight {
                selected = *enemy_type;
                break;
            }
        }
        selected
    };

    let config = enemy_type.config();
    let size = config.sprite_size.as_vec2();
    let size_x = size.x * config.scale;
    let column_count = (window.width() / (size_x + ENEMY_GUTTER)) as u32;
    let column = rng.gen_range(0..column_count);
    let x_pos = calculate_enemy_x_position(&window, column, size_x);
    let spawn_position = Vec3::new(x_pos, window.height() / 2.0 + size_x / 2.0, 1.0);

    let texture = asset_server.load_with_settings(
        config.sprite_path,
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );
    let layout = TextureAtlasLayout::from_grid(
        config.sprite_size,
        config.sprite_columns,
        config.sprite_rows,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Enemy { enemy_type },
        Collider,
        Transform::from_translation(spawn_position),
        MovementSpeed(config.speed),
        Health(config.health),
        Bounds {
            size: size * config.scale,
        },
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            custom_size: Some(size * config.scale),
            ..default()
        },
        AnimationConfig::new(0, 1, config.sprite_fps),
    ));

    enemy_count.increment(&enemy_type);
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
    query: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
    window: Single<&Window>,
) {
    for (entity, transform, enemy) in &query {
        if transform.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            enemy_count.decrement(&enemy.enemy_type);
        }
    }
}

fn reset_enemies(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    mut game_restart_event: EventReader<GameRestartEvent>,
    query: Query<Entity, With<Enemy>>,
) {
    if !game_restart_event.is_empty() {
        game_restart_event.clear();

        for entity in &query {
            commands.entity(entity).despawn();
        }

        // Reset all counts to 0
        *enemy_count = EnemyCount::default();
    }
}
