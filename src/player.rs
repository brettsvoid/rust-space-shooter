use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    collisions::Collider,
    components::{Bounds, Bullet, MovementInput, MovementSpeed, Shoot},
    game_state::GameState,
    sprite_animation::{update_animations, AnimationConfig},
};

const SPRITE_SHEET_PATH: &str = "../assets/ship.png";
const SPRITE_SIZE: UVec2 = UVec2::new(16, 24);
const SPRITE_COLUMNS: u32 = 2;
const SPRITE_ROWS: u32 = 5;
const SPRITE_FPS: u8 = 12;

// TODO: start very slow and gain speed with leveling up
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SHOOT_COOLDOWN: f32 = 0.2;

// Sprite indices for different states
const IDLE_SPRITES: (usize, usize) = (0, 1);
const TRANSITION_LEFT_SPRITES: (usize, usize) = (2, 3);
const MOVE_LEFT_SPRITES: (usize, usize) = (4, 5);
const TRANSITION_RIGHT_SPRITES: (usize, usize) = (6, 7);
const MOVE_RIGHT_SPRITES: (usize, usize) = (8, 9);

const BULLET_SPRITE_PATH: &str = "../assets/laser-bolts.png";
const BULLET_SPRITE_SIZE: UVec2 = UVec2::new(16, 16);
const BULLET_SPRITE_COLUMNS: u32 = 2;
const BULLET_SPRITE_ROWS: u32 = 2;
const BULLET_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    update_player_state,
                    handle_player_shoot,
                    (spawn_bullets, apply_bullet_movement),
                    update_animations::<Bullet>,
                    (update_animation_stack, update_player_animation).chain(),
                    (
                        handle_player_movement,
                        apply_player_movement,
                        confine_player_movement,
                    )
                        .chain(),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

// Player marker component
#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Default, Debug)]
pub enum PlayerState {
    #[default]
    Idle,
    MovingLeft,
    MovingRight,
}
#[derive(Component, Default, Debug)]
pub struct PrevPlayerState(PlayerState);

#[derive(Component)]
struct AnimationStack {
    frames: Vec<(usize, usize)>,
    cycles: u8,
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
        PlayerState::default(),
        PrevPlayerState::default(),
        Collider,
        Shoot::new(PLAYER_SHOOT_COOLDOWN),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), // keep above bullet entities
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            custom_size: Some(size * 2.0),
            ..default()
        },
        AnimationConfig::new(IDLE_SPRITES.0, IDLE_SPRITES.1, SPRITE_FPS),
        AnimationStack {
            frames: vec![IDLE_SPRITES],
            cycles: 1,
        },
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
    mut query: Populated<(&MovementInput, &MovementSpeed, &mut Transform), With<Player>>,
    time: Res<Time>,
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

fn update_player_state(
    mut query: Query<(&MovementInput, &mut PlayerState, &mut PrevPlayerState), With<Player>>,
) {
    let (input, mut state, mut prev_state) = query.single_mut();

    if input.direction.x < 0.0 {
        if !matches!(*state, PlayerState::MovingLeft) {
            *prev_state = PrevPlayerState(state.clone());
            *state = PlayerState::MovingLeft;
        }
    } else if input.direction.x > 0.0 {
        if !matches!(*state, PlayerState::MovingRight) {
            *prev_state = PrevPlayerState(state.clone());
            *state = PlayerState::MovingRight;
        }
    } else {
        if !matches!(*state, PlayerState::Idle) {
            *prev_state = PrevPlayerState(state.clone());
            *state = PlayerState::Idle;
        }
    }
}

fn update_animation_stack(
    mut query: Query<(&PlayerState, &PrevPlayerState, &mut AnimationStack), Changed<PlayerState>>,
) {
    for (state, prev_state, mut animation_stack) in query.iter_mut() {
        animation_stack.frames = match state {
            PlayerState::Idle => {
                if matches!(prev_state, PrevPlayerState(PlayerState::MovingLeft)) {
                    vec![IDLE_SPRITES, TRANSITION_LEFT_SPRITES]
                } else if matches!(prev_state, PrevPlayerState(PlayerState::MovingRight)) {
                    vec![IDLE_SPRITES, TRANSITION_RIGHT_SPRITES]
                } else {
                    vec![IDLE_SPRITES]
                }
            }
            PlayerState::MovingLeft => vec![MOVE_LEFT_SPRITES, TRANSITION_LEFT_SPRITES],
            PlayerState::MovingRight => vec![MOVE_RIGHT_SPRITES, TRANSITION_RIGHT_SPRITES],
        };
        animation_stack.cycles = 0;
    }
}

fn update_player_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &mut AnimationStack), With<Player>>,
) {
    let (mut config, mut sprite, mut animation_stack) = query.single_mut();

    let frame = animation_stack.frames.last();
    let (first, last) = frame.unwrap();

    // If the state changes, reset timer and set initial frame
    if config.first_sprite_index != *first || config.last_sprite_index != *last {
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = *first;
        }
    }

    // We track how long the current sprite has been displayed for
    config.frame_timer.tick(time.delta());
    config.first_sprite_index = *first;
    config.last_sprite_index = *last;

    // If it has been displayed for the user-defined amount of time (fps)...
    if config.frame_timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index >= *last {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = *first;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
            }
            // ...and reset the frame timer to start counting all over again
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }

        // We keep track of 2 cycles for transition periods
        if animation_stack.cycles > 1 {
            animation_stack.cycles = 0;
            if animation_stack.frames.len() > 1 {
                animation_stack.frames.pop();
            }
        } else {
            animation_stack.cycles += 1;
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

fn handle_player_shoot(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Shoot, With<Player>>,
) {
    let mut shoot = query.single_mut();
    shoot.is_shooting = keyboard.pressed(KeyCode::Space);
}

fn spawn_bullets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut Shoot, &Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut shoot, transform) = query.single_mut();
    shoot.timer.tick(time.delta());
    if !shoot.is_shooting {
        return;
    }

    let texture = asset_server.load_with_settings(
        BULLET_SPRITE_PATH,
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );
    let layout = TextureAtlasLayout::from_grid(
        BULLET_SPRITE_SIZE,
        BULLET_SPRITE_COLUMNS,
        BULLET_SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let size = BULLET_SPRITE_SIZE.as_vec2();
    if shoot.timer.finished() {
        commands.spawn((
            Bullet,
            Sprite {
                image: texture,
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 2,
                }),
                custom_size: Some(size * 2.0),
                ..default()
            },
            AnimationConfig::new(2, 3, SPRITE_FPS),
            Transform::from_translation(transform.translation),
        ));
        shoot.timer = Shoot::timer_from_cooldown(PLAYER_SHOOT_COOLDOWN);
    }
}

fn apply_bullet_movement(mut query: Populated<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let movement = Vec2::new(0.0, BULLET_SPEED) * time.delta_secs();
        transform.translation += movement.extend(0.0);
    }
}
