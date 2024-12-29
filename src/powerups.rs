use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    collisions::Collider,
    components::{Bounds, MovementSpeed, PlayerStats},
    enemies::{EnemyDestroyedEvent, EnemyType},
    game::GameRestartEvent,
    game_state::GameState,
    player::Player,
    sprite_animation::{update_animations, AnimationConfig},
    AppState,
};

const MAX_POWERUPS: usize = 3;

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

#[derive(Resource, Default)]
pub struct PowerupCount(pub usize);

pub struct PowerupsPlugin;
impl Plugin for PowerupsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PowerupCount>()
            .add_systems(OnEnter(AppState::Game), powerups_setup)
            .add_systems(
                Update,
                (
                    spawn_powerups,
                    apply_powerup_movement,
                    remove_fallen_powerups,
                    handle_powerup_collisions,
                    update_animations::<Powerup>,
                )
                    .run_if(in_state(AppState::Game).and(in_state(GameState::Playing))),
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut enemy_destroyed_event: EventReader<EnemyDestroyedEvent>,
) {
    // Only spawn new powerups if we haven't reached the maximum
    if powerup_count.0 >= MAX_POWERUPS {
        return;
    }

    for event in enemy_destroyed_event.read() {
        let enemy_type = event.0.enemy_type;
        if let EnemyType::Small = enemy_type {
            continue;
        };

        let powerup_type = match enemy_type {
            EnemyType::Medium => PowerupType::Speed,
            EnemyType::Large => PowerupType::FireRate,
            _ => PowerupType::Speed,
        };

        let config = powerup_type.config();
        let size = config.sprite_size.as_vec2();
        let spawn_position = event.0.position;

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
                    index: match powerup_type {
                        PowerupType::FireRate => 0,
                        PowerupType::Speed => 2,
                    },
                }),
                custom_size: Some(size * config.scale),
                ..default()
            },
            animation_config,
        ));

        powerup_count.0 += 1;
    }
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
                    player_stats.fire_rate *= 1.5;
                }
                PowerupType::Speed => {
                    player_stats.speed *= 1.2;
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
