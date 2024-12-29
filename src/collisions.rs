use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{
    components::{Bounds, Bullet, Health},
    enemies::{Enemy, EnemyCount, EnemyDestroyedData, EnemyDestroyedEvent, EnemyType},
    explosion::{DestroyedData, DestroyedEvent},
    game_state::GameState,
    player::Player,
    scoreboard::Score,
};

pub struct CollisionsPlugin;
impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_event::<EnemyDestroyedEvent>()
            .add_systems(
                Update,
                ((
                    (
                        check_player_bullet_enemy_collision,
                        check_player_enemy_collision,
                    )
                        .chain(),
                    check_enemy_health,
                )
                    .run_if(in_state(GameState::Playing)),),
            );
    }
}

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

pub fn check_player_enemy_collision(
    player: Single<(&Transform, &Bounds, &mut Health), With<Player>>,
    mut enemy_query: Query<(&Transform, &Bounds, &mut Health), (With<Enemy>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (player_transform, player_bounds, mut player_health) = player.into_inner();
    let player_aabb2d = Aabb2d::new(
        player_transform.translation.truncate(),
        player_bounds.size / 2.0,
    );

    for (enemy_transform, enemy_bounds, mut enemy_health) in &mut enemy_query {
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

            if enemy_health.0 > 0 && player_health.0 > 0 {
                let new_player_health = player_health.0 - enemy_health.0;
                let new_enemy_health = enemy_health.0 - player_health.0;

                player_health.0 = new_player_health;
                enemy_health.0 = new_enemy_health;
            }
        }
    }
}

fn check_player_bullet_enemy_collision(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &Bounds, &mut Health, &Collider), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    let player_damage = 1;

    for (enemy_transform, enemy_bounds, mut enemy_health, _) in &mut enemy_query {
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

fn check_enemy_health(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Enemy, &Health), With<Enemy>>,
    mut enemy_count: ResMut<EnemyCount>,
    mut score: ResMut<Score>,
    mut destroyed_event: EventWriter<DestroyedEvent>,
    mut enemy_destroyed_event: EventWriter<EnemyDestroyedEvent>,
) {
    for (enemy_entity, enemy_transform, enemy, enemy_health) in &enemy_query {
        if enemy_health.0 <= 0 {
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

            destroyed_event.send(DestroyedEvent(DestroyedData {
                position: enemy_transform.translation,
            }));
        }
    }
}
