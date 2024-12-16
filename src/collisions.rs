use bevy::{
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::{components::Bounds, enemies::Enemy, game_state::GameState, player::Player};

pub struct CollisionsPlugin;
impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            Update,
            (
                check_player_enemy_collision,
                //check_player_bullet_enemy_collision,
            )
                .run_if(in_state(GameState::Playing)),
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

            //println!("Collision detected: {:?}", collision);

            // Bricks should be despawned and increment the scoreboard on collision
            // if maybe_brick.is_some() {
            //     commands.entity(collider_entity).despawn();
            //     **score += 1;
            // }
            //commands.entity(*player_entity).despawn();

            // Set the game state to GameOver
            game_state.set(GameState::GameOver);

            // Reflect the ball's velocity when it collides
            // let mut reflect_x = false;
            // let mut reflect_y = false;

            // Reflect only if the velocity is in the opposite direction of the collision
            // This prevents the ball from getting stuck inside the bar
            // let mut entity_velocity = source_entity.velocity.unwrap();
            // match collision {
            //     Collision::Left => reflect_x = entity_velocity.x > 0.0,
            //     Collision::Right => reflect_x = entity_velocity.x < 0.0,
            //     Collision::Top => reflect_y = entity_velocity.y < 0.0,
            //     Collision::Bottom => reflect_y = entity_velocity.y > 0.0,
            // }
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
