use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_rand::prelude::*;
use rand::prelude::*;

use crate::{
    collisions::Collider,
    components::{Bounds, MovementSpeed, PlayerStats},
    game::GameRestartEvent,
    game_state::GameState,
    player::Player,
    sprite_animation::{update_animations, AnimationConfig},
};

const MAX_POWERUPS: usize = 1;
const POWERUPS_SPAWN_CHANCE: u32 = 1;
const POWERUPS_SPAWN_DENOMINATOR: u32 = 10; // higher means less powerups

struct PowerupsConfig {
    sprite_path: &'static str,
    sprite_size: UVec2,
    sprite_columns: u32,
    sprite_rows: u32,
    sprite_fps: u8,
    speed: f32,
    scale: f32,
    //spawn_weight: f32,
}

#[derive(Component, Clone, Copy)]
pub enum PowerupType {
    FireRate,
    Speed,
}
impl PowerupType {
    fn config(&self) -> PowerupsConfig {
        match self {
            PowerupType::FireRate => PowerupsConfig {
                sprite_path: "powerup.png",
                sprite_size: UVec2::new(16, 16),
                sprite_columns: 2,
                sprite_rows: 2,
                sprite_fps: 12,
                speed: 50.0,
                scale: 2.0,
                //spawn_weight: 1.0,
            },
            PowerupType::Speed => PowerupsConfig {
                sprite_path: "powerup.png",
                sprite_size: UVec2::new(16, 16),
                sprite_columns: 2,
                sprite_rows: 2,
                sprite_fps: 12,
                speed: 50.0,
                scale: 2.0,
                //spawn_weight: 1.0,
            },
        }
    }
}

// Powerup marker component
#[derive(Component)]
pub struct Powerup {
    pub powerup_type: PowerupType,
}

#[derive(Resource)]
pub struct PowerupCount(pub usize);
impl Default for PowerupCount {
    fn default() -> Self {
        Self(0)
    }
}

pub struct PowerupsPlugin;
impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerupCount>()
            .add_systems(OnEnter(GameState::Playing), powerups_setup)
            .add_systems(
                Update,
                (
                    spawn_powerups,
                    apply_powerup_movement,
                    remove_fallen_powerups,
                    handle_powerup_collisions,
                    update_animations::<Powerup>,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, reset_powerups);
    }
}

fn powerups_setup(mut powerup_count: ResMut<PowerupCount>) {
    // Reset all counts to 0 when entering Playing state
    *powerup_count = PowerupCount::default();
}

fn spawn_powerups(
    mut commands: Commands,
    mut powerup_count: ResMut<PowerupCount>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    window: Single<&Window>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Only spawn new powerups if we haven't reached the maximum
    if powerup_count.0 >= MAX_POWERUPS {
        return;
    }

    // Random chance to spawn a new powerup
    if rng.gen_range(0..POWERUPS_SPAWN_DENOMINATOR) > POWERUPS_SPAWN_CHANCE {
        return;
    }

    let powerup_type = {
        let weights = [PowerupType::FireRate, PowerupType::Speed];
        let roll: usize = rng.gen_range(0..weights.len());

        weights[roll]
    };

    let config = powerup_type.config();
    let size = config.sprite_size.as_vec2();
    let size_x = size.x * config.scale;
    let column_count = (window.width() / (size_x)) as u32;
    let column = rng.gen_range(0..column_count);
    let x_pos = calculate_powerup_x_position(&window, column, size_x);
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

    let animation_config = match powerup_type {
        PowerupType::FireRate => AnimationConfig::new(0, 1, config.sprite_fps),
        PowerupType::Speed => AnimationConfig::new(2, 3, config.sprite_fps),
    };
    commands.spawn((
        Powerup { powerup_type },
        Collider,
        Transform::from_translation(spawn_position),
        MovementSpeed(config.speed),
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
        animation_config,
    ));

    powerup_count.0 += 1;
}

fn apply_powerup_movement(
    mut query: Populated<(&MovementSpeed, &mut Transform), With<Powerup>>,
    time: Res<Time>,
) {
    for (speed, mut transform) in query.iter_mut() {
        let movement = Vec2::new(0.0, -speed.0 * time.delta_secs());
        transform.translation += movement.extend(0.0);
    }
}

/// Calculate the x position for an powerup in a given column
fn calculate_powerup_x_position(window: &Window, column: u32, size_x: f32) -> f32 {
    let width = window.width();
    let column_count = (width / (size_x)) as u32;
    let gutter_count = column_count - 1;
    let content_width = column_count as f32 * size_x + gutter_count as f32;
    let margin = (width - content_width) / 2.0;

    (column as f32 * (size_x)) + size_x / 2.0 + margin - width / 2.0
}

fn remove_fallen_powerups(
    mut commands: Commands,
    mut powerup_count: ResMut<PowerupCount>,
    query: Query<(Entity, &Transform), With<Powerup>>,
    window: Single<&Window>,
) {
    for (entity, transform) in &query {
        if transform.translation.y < -window.height() / 2.0 {
            commands.entity(entity).despawn();
            powerup_count.0 -= 1;
        }
    }
}

fn handle_powerup_collisions(
    mut commands: Commands,
    mut powerup_count: ResMut<PowerupCount>,
    mut player_query: Query<(&Transform, &Bounds, &mut PlayerStats), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &Bounds, &Powerup)>,
) {
    let (player_transform, player_bounds, mut player_stats) = player_query.single_mut();

    for (powerup_entity, powerup_transform, powerup_bounds, powerup) in powerup_query.iter() {
        let player_pos = player_transform.translation.truncate();
        let powerup_pos = powerup_transform.translation.truncate();

        // Simple AABB collision check
        if (player_pos.x - powerup_pos.x).abs()
            < (player_bounds.size.x + powerup_bounds.size.x) / 2.0
            && (player_pos.y - powerup_pos.y).abs()
                < (player_bounds.size.y + powerup_bounds.size.y) / 2.0
        {
            match powerup.powerup_type {
                // PowerupType::HealthBoost => {
                //     stats.health.0 = (stats.health.0 + 1).min(3); // Max health of 3
                // },
                PowerupType::FireRate => {
                    player_stats.fire_rate *= 4.0; // 200% fire rate boost
                }
                PowerupType::Speed => {
                    player_stats.speed *= 1.5; // 50% speed boost
                } // PowerupType::WeaponUpgrade => {
                  //     stats.weapon_level = (stats.weapon_level + 1).min(3); // Max weapon level of 3
                  // },
            }

            commands.entity(powerup_entity).despawn();
            powerup_count.0 -= 1;
        }
    }
}

fn reset_powerups(
    mut commands: Commands,
    mut powerup_count: ResMut<PowerupCount>,
    mut game_restart_event: EventReader<GameRestartEvent>,
    query: Query<Entity, With<Powerup>>,
) {
    if !game_restart_event.is_empty() {
        game_restart_event.clear();

        for entity in &query {
            commands.entity(entity).despawn();
        }

        // Reset all counts to 0
        *powerup_count = PowerupCount::default();
    }
}