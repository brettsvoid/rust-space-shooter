use crate::{
    components::{Bounds, MovementInput, MovementSpeed},
    sprite_animation::AnimationConfig,
};
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

const SPRITE_SHEET_PATH: &str = "../assets/ship.png";
const SPRITE_SIZE: UVec2 = UVec2::new(16, 24);
const SPRITE_COLUMNS: u32 = 2;
const SPRITE_ROWS: u32 = 5;
const SPRITE_FPS: u8 = 3;

const PLAYER_SPEED: f32 = 5.0;

// Sprite indices for different states
const IDLE_SPRITES: (usize, usize) = (0, 1);
const MOVE_RIGHT_SPRITES: (usize, usize) = (8, 9);
const MOVE_LEFT_SPRITES: (usize, usize) = (4, 5);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            ((
                handle_player_movement,
                update_player_state,
                update_player_animation,
                apply_player_movement,
                confine_player_movement,
            )
                .chain(),),
        );
    }
}

// Player marker component
#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    MovingLeft,
    MovingRight,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture =
        asset_server.load_with_settings(SPRITE_SHEET_PATH, |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        });
    let layout =
        TextureAtlasLayout::from_grid(SPRITE_SIZE, SPRITE_COLUMNS, SPRITE_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let size = SPRITE_SIZE.as_vec2();

    commands.spawn((
        Player,
        MovementInput {
            direction: Vec2::ZERO,
        },
        MovementSpeed(PLAYER_SPEED),
        Bounds { size: size * 2.0 },
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 1,
            }),
            custom_size: Some(size * 2.0),
            ..default()
        },
        AnimationConfig::new(IDLE_SPRITES.0, IDLE_SPRITES.1, SPRITE_FPS),
        PlayerState::default(),
    ));
}

fn handle_player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementInput, With<Player>>,
) {
    let mut input = query.single_mut();
    input.direction = get_input_direction(&keyboard);
}

fn apply_player_movement(
    time: Res<Time>,
    mut query: Query<(&MovementInput, &MovementSpeed, &mut Transform), With<Player>>,
) {
    let (input, speed, mut transform) = query.single_mut();
    let movement = input.direction * speed.0 * time.delta_secs();
    transform.translation += movement.extend(0.0);
}

fn confine_player_movement(
    window: Query<&Window>,
    mut query: Query<(&Bounds, &mut Transform), With<Player>>,
) {
    let window = window.single();
    let (bounds, mut transform) = query.single_mut();

    let half_size = bounds.size / 2.0;
    let min_x = -window.width() / 2.0 + half_size.x;
    let max_x = window.width() / 2.0 - half_size.x;
    let min_y = -window.height() / 2.0 + half_size.y;
    let max_y = window.height() / 2.0 - half_size.y;

    transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    transform.translation.y = transform.translation.y.clamp(min_y, max_y);
}

fn update_player_state(mut query: Query<(&MovementInput, &mut PlayerState), With<Player>>) {
    let (input, mut state) = query.single_mut();

    *state = if input.direction.x < 0.0 {
        PlayerState::MovingLeft
    } else if input.direction.x > 0.0 {
        PlayerState::MovingRight
    } else {
        PlayerState::Idle
    };
}

fn update_player_animation(
    time: Res<Time>,
    mut query: Query<(&PlayerState, &mut AnimationConfig, &mut Sprite), With<Player>>,
) {
    let (state, mut config, mut sprite) = query.single_mut();

    // Update animation range based on player state
    let (first, last) = match state {
        PlayerState::Idle => IDLE_SPRITES,
        PlayerState::MovingLeft => MOVE_LEFT_SPRITES,
        PlayerState::MovingRight => MOVE_RIGHT_SPRITES,
    };

    // TODO: reverse animation when going back to idle

    // If the state changes, reset timer and atlas position
    if config.first_sprite_index != first || config.last_sprite_index != last {
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        if let Some(atlas) = &mut sprite.texture_atlas {
            if first >= 2 {
                // Go back 2 animation frames for the transition (for ship animation only)
                atlas.index = first - 2;
            } else {
                atlas.index = first;
            }
        }
    }

    // We track how long the current sprite has been displayed for
    config.frame_timer.tick(time.delta());
    config.first_sprite_index = first;
    config.last_sprite_index = last;

    // If it has been displayed for the user-defined amount of time (fps)...
    if config.frame_timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index >= last {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = first;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
            }
            // ...and reset the frame timer to start counting all over again
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }
    }
}

fn get_input_direction(keyboard: &ButtonInput<KeyCode>) -> Vec2 {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    // Normalize the direction to prevent faster diagonal movement
    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    direction
}
