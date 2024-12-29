use bevy::{
    audio::Volume,
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    audio::GameSounds,
    components::{Bounds, Bullet, Health},
    enemies::{Enemy, EnemyCount, EnemyDestroyedData, EnemyDestroyedEvent, EnemyType},
    game_state::GameState,
    player::Player,
    scoreboard::Score,
    settings::Settings,
    sprite_animation::AnimationConfig,
};

pub struct CollisionsPlugin;
impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_event::<EnemyDestroyedEvent>()
            .add_systems(
                Update,
                (
                    (check_player_enemy_collision).run_if(in_state(GameState::Playing)),
                    (check_player_bullet_enemy_collision).run_if(in_state(GameState::Playing)),
                    update_explosion_animation,
                ),
            );
    }
}

const SPRITE_FPS: u8 = 12;

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
struct Explosion;

pub fn check_player_enemy_collision(
    player_query: Query<(Entity, &Transform, &Bounds, &Collider), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Bounds, &Collider), With<Enemy>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (_player_entity, player_transform, player_bounds, _) = &player_query.single();
    let player_aabb2d = Aabb2d::new(
        player_transform.translation.truncate(),
        player_bounds.size / 2.0,
    );

    for (_, enemy_transform, enemy_bounds, _) in &enemy_query {
        let collision = is_collision(
            player_aabb2d,
            Aabb2d::new(
                enemy_transform.translation.truncate(),
                enemy_bounds.size / 2.0,
            ),
        );

        if let Some(_collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            // Set the game state to GameOver
            game_state.set(GameState::GameOver);
        }
    }
}

fn check_player_bullet_enemy_collision(
    mut commands: Commands,
    mut enemy_query: Query<
        (Entity, &Transform, &Bounds, &Enemy, &mut Health, &Collider),
        With<Enemy>,
    >,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut enemy_count: ResMut<EnemyCount>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    game_sounds: Res<GameSounds>,
    mut enemy_destroyed_event: EventWriter<EnemyDestroyedEvent>,
    settings: Res<Settings>,
) {
    let explosion_image = asset_server.load("../assets/explosion.png");

    let explosion_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 1, None, None);
    let explosion_atlas_handle = texture_atlases.add(explosion_atlas);

    let player_damage = 1;

    for (enemy_entity, enemy_transform, enemy_bounds, enemy, mut enemy_health, _) in
        &mut enemy_query
    {
        for (bullet_entity, bullet_transform) in &bullet_query {
            let collision = is_collision(
                Aabb2d::new(
                    enemy_transform.translation.truncate(),
                    enemy_bounds.size / 2.0,
                ),
                Aabb2d::new(bullet_transform.translation.truncate(), Vec2::splat(8.0)),
            );
            if let Some(_collision) = collision {
                let new_health = enemy_health.0 - player_damage;
                enemy_health.0 = new_health;

                if new_health <= 0 {
                    commands.spawn((
                        Explosion,
                        AudioPlayer::new(game_sounds.explosion.clone()),
                        PlaybackSettings {
                            volume: Volume::new(settings.effect_volume),
                            ..default()
                        },
                        Transform::from_translation(enemy_transform.translation), // keep above bullet entities
                        Sprite {
                            image: explosion_image.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: explosion_atlas_handle.clone(),
                                index: 0,
                            }),
                            custom_size: Some(Vec2::splat(16.0) * 2.0),
                            ..default()
                        },
                        AnimationConfig::new(0, 4, SPRITE_FPS),
                    ));

                    commands.entity(enemy_entity).despawn();
                    enemy_count.decrement(&enemy.enemy_type);

                    **score += match enemy.enemy_type {
                        EnemyType::Large => 40,
                        EnemyType::Medium => 12,
                        EnemyType::Small => 2,
                    };

                    enemy_destroyed_event.send(EnemyDestroyedEvent(EnemyDestroyedData {
                        enemy_type: enemy.enemy_type,
                        position: enemy_transform.translation,
                    }));
                }

                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

fn is_collision(entity_a: Aabb2d, entity_b: Aabb2d) -> Option<Collision> {
    if !entity_a.intersects(&entity_b) {
        return None;
    }

    let closest = entity_b.closest_point(entity_a.center());
    let offset = entity_a.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn update_explosion_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationConfig, &mut Sprite), With<Explosion>>,
) {
    for (entity, mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= config.last_sprite_index {
                    // ...and it IS the last frame, then we despawn the explosion
                    commands.entity(entity).despawn();
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}
