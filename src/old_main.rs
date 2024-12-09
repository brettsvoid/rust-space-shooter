use enemies::Enemies;
use game_state::GameState;
use macroquad::audio::{play_sound, set_sound_volume, PlaySoundParams};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use macroquad_particles::{Emitter, EmitterConfig};
use std::fs;
use throttler::Throttler;

use crate::resources::Resources;
use crate::settings::Settings;

mod enemies;
mod game_state;
mod particle_effects;
mod resources;
mod settings;
mod throttler;

const FRAGMENT_SHADER: &str = include_str!("starfield.frag");
const VERTEX_SHADER: &str = include_str!("vertex.vert");

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

enum StarfieldSpeed {
    Stop,
    Slow,
    Fast,
}

fn get_column_x(size: f32) -> f32 {
    let gutter = 4.0;
    let width = screen_width();
    let column_count = (width / (size + gutter)) as u32;
    let gutter_count = column_count - 1;
    let content_width = column_count as f32 * size + gutter_count as f32 * gutter;
    let margin = (width - content_width) / 2.0;
    let column = rand::gen_range(0, column_count);

    (column as f32 * (size + gutter)) + size / 2.0 + margin
}

#[macroquad::main("Space Shooter")]
async fn main() -> Result<(), macroquad::Error> {
    const MOVE_SPEED: f32 = 200.0;

    // Seed the random number generator
    rand::srand(miniquad::date::now() as u64);

    let mut settings = Settings::new();
    // Sound seems way too loud by default
    settings.set_music_volume(0.5);
    settings.set_effect_volume(0.5);

    let mut enemies = Enemies::new();
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVE_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };
    let mut game_state = GameState::MainMenu;
    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);
    let mut starfield_speed = StarfieldSpeed::Slow;

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc {
                    name: "iResolution".to_string(),
                    uniform_type: UniformType::Float2,
                    array_count: 1,
                },
                UniformDesc {
                    name: "direction_modifier".to_string(),
                    uniform_type: UniformType::Float1,
                    array_count: 1,
                },
                UniformDesc {
                    name: "speed".to_string(),
                    uniform_type: UniformType::Float1,
                    array_count: 1,
                },
            ],
            ..Default::default()
        },
    )?;
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    set_pc_assets_folder("assets");
    Resources::load().await?;
    let resources = storage::get::<Resources>();

    root_ui().push_skin(&resources.ui_skin);
    let window_size = vec2(370.0, 320.0);

    let mut bullet_sprite = AnimatedSprite::new(
        16,
        16,
        &[
            Animation {
                name: "bullet".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "bolt".to_string(),
                row: 1,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );
    bullet_sprite.set_animation(1);
    let mut ship_sprite = AnimatedSprite::new(
        16,
        24,
        &[
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "left".to_string(),
                row: 2,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "right".to_string(),
                row: 4,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );

    play_sound(
        &resources.theme_music,
        PlaySoundParams {
            looped: true,
            volume: settings.music_volume,
        },
    );
    set_sound_volume(&resources.sound_explosion, settings.effect_volume);
    set_sound_volume(&resources.sound_laser, settings.effect_volume);

    let mut last_shot_throttler = Throttler::new(0.2);
    let mut settings_sound_effect_throttler = Throttler::new(0.2);
    let small_enemy_size = 32.0;
    let medium_enemy_size = 48.0;
    let large_enemy_size = 64.0;

    loop {
        clear_background(BLACK);

        let speed: f32 = match starfield_speed {
            StarfieldSpeed::Stop => 0.0,
            StarfieldSpeed::Slow => 1.0,
            StarfieldSpeed::Fast => 3.0,
        };

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        material.set_uniform("speed", speed);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::P) {
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::S) {
                    game_state = GameState::Settings;
                }
                if is_key_pressed(KeyCode::Q) {
                    std::process::exit(0);
                }
                starfield_speed = StarfieldSpeed::Slow;
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(80.0, -34.0), "Main Menu");
                        if ui.button(vec2(80.0, 10.0), "Play") {
                            enemies.clear();
                            bullets.clear();
                            explosions.clear();
                            circle.x = screen_width() / 2.0;
                            circle.y = screen_height() / 2.0;
                            score = 0;
                            game_state = GameState::Playing;
                        }
                        if ui.button(vec2(35.0, 85.0), "Settings") {
                            game_state = GameState::Settings;
                        }
                        if ui.button(vec2(80.0, 160.0), "Quit") {
                            std::process::exit(0);
                        }
                    },
                );
            }
            GameState::Playing => {
                let delta = get_frame_time();
                starfield_speed = StarfieldSpeed::Fast;
                ship_sprite.set_animation(0);
                last_shot_throttler.update(delta);
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    circle.x += MOVE_SPEED * delta;
                    direction_modifier += 0.05 * delta;
                    ship_sprite.set_animation(2);
                }
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    circle.x -= MOVE_SPEED * delta;
                    direction_modifier -= 0.05 * delta;
                    ship_sprite.set_animation(1);
                }
                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    circle.y += MOVE_SPEED * delta;
                }
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    circle.y -= MOVE_SPEED * delta
                }
                if is_key_down(KeyCode::Space) {
                    last_shot_throttler.run_action(|| {
                        bullets.push(Shape {
                            x: circle.x,
                            y: circle.y - 24.0,
                            speed: circle.speed * 2.0,
                            size: 32.0,
                            collided: false,
                        });
                        play_sound(
                            &resources.sound_laser,
                            PlaySoundParams {
                                looped: false,
                                volume: settings.effect_volume,
                            },
                        );
                    })
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Clamp X and Y to be within the screen
                circle.x = clamp(circle.x, 0.0, screen_width());
                circle.y = clamp(circle.y, 0.0, screen_height());

                // Generate a new enemy
                if rand::gen_range(0, 25) <= 1 && enemies.small.len() < 10 {
                    let size = small_enemy_size;
                    enemies.small.push(Shape {
                        size,
                        speed: 100.0,
                        x: get_column_x(size),
                        y: -size,
                        collided: false,
                    });
                }
                if rand::gen_range(0, 75) <= 1 && enemies.medium.len() < 4 {
                    let size = medium_enemy_size;
                    enemies.medium.push(Shape {
                        size,
                        speed: 80.0,
                        x: get_column_x(size),
                        y: -size,
                        collided: false,
                    });
                }
                if rand::gen_range(0, 120) <= 1 && enemies.large.len() < 1 {
                    let size = large_enemy_size;
                    enemies.large.push(Shape {
                        size,
                        speed: 50.0,
                        x: get_column_x(size),
                        y: -size,
                        collided: false,
                    });
                }

                // Movement
                enemies.movement(delta);
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta;
                }

                ship_sprite.update();
                bullet_sprite.update();
                enemies.update();

                // Remove shapes outside of screen
                enemies
                    .small
                    .retain(|enemy| enemy.y < screen_height() + enemy.size);
                enemies
                    .medium
                    .retain(|enemy| enemy.y < screen_height() + enemy.size);
                enemies
                    .large
                    .retain(|enemy| enemy.y < screen_height() + enemy.size);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

                // Remove collided shapes
                enemies.small.retain(|enemy| !enemy.collided);
                enemies.medium.retain(|enemy| !enemy.collided);
                enemies.large.retain(|enemy| !enemy.collided);
                bullets.retain(|bullet| !bullet.collided);

                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // Check for collisions
                if enemies
                    .small
                    .iter()
                    .any(|enemy| circle.collides_with(enemy))
                {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for enemy in enemies.small.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(enemy) {
                            bullet.collided = true;
                            enemy.collided = true;
                            score += enemy.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: enemy.size.round() as u32 * 1,
                                    texture: Some(resources.explosion_texture.clone()),
                                    ..particle_effects::explosion()
                                }),
                                vec2(enemy.x, enemy.y),
                            ));
                            play_sound(
                                &resources.sound_explosion,
                                PlaySoundParams {
                                    looped: false,
                                    volume: settings.effect_volume,
                                },
                            );
                        }
                    }
                }
                if enemies
                    .medium
                    .iter()
                    .any(|enemy| circle.collides_with(enemy))
                {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for enemy in enemies.medium.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(enemy) {
                            bullet.collided = true;
                            enemy.collided = true;
                            score += enemy.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: enemy.size.round() as u32 * 1,
                                    texture: Some(resources.explosion_texture.clone()),
                                    ..particle_effects::explosion()
                                }),
                                vec2(enemy.x, enemy.y),
                            ));
                            play_sound(
                                &resources.sound_explosion,
                                PlaySoundParams {
                                    looped: false,
                                    volume: settings.effect_volume,
                                },
                            );
                        }
                    }
                }
                if enemies
                    .large
                    .iter()
                    .any(|enemy| circle.collides_with(enemy))
                {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for enemy in enemies.large.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(enemy) {
                            bullet.collided = true;
                            enemy.collided = true;
                            score += enemy.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: enemy.size.round() as u32 * 1,
                                    texture: Some(resources.explosion_texture.clone()),
                                    ..particle_effects::explosion()
                                }),
                                vec2(enemy.x, enemy.y),
                            ));
                            play_sound(
                                &resources.sound_explosion,
                                PlaySoundParams {
                                    looped: false,
                                    volume: settings.effect_volume,
                                },
                            );
                        }
                    }
                }

                // Draw everything
                let bullet_frame = bullet_sprite.frame();
                for bullet in &bullets {
                    draw_texture_ex(
                        &resources.bullet_texture,
                        bullet.x - bullet.size / 2.0,
                        bullet.y - bullet.size / 2.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(bullet.size, bullet.size)),
                            source: Some(bullet_frame.source_rect),
                            ..Default::default()
                        },
                    );
                }
                let ship_frame = ship_sprite.frame();
                draw_texture_ex(
                    &resources.ship_texture,
                    circle.x - ship_frame.dest_size.x,
                    circle.y - ship_frame.dest_size.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(ship_frame.dest_size * 2.0),
                        source: Some(ship_frame.source_rect),
                        ..Default::default()
                    },
                );
                enemies.draw(&resources);

                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }
                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High score: {}", high_score);
                let text_dimensions = measure_text(highscore_text.as_str(), None, 25, 1.0);
                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimensions.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
                starfield_speed = StarfieldSpeed::Stop;
                let paused_text = "Paused";
                let paused_dimensions = measure_text(paused_text, None, 50, 1.0);
                draw_text(
                    paused_text,
                    screen_width() / 2.0 - paused_dimensions.width / 2.0,
                    screen_height() / 2.0 - 50.0,
                    50.0,
                    WHITE,
                );

                let space_text = "Press SPACE to resume";
                let space_dimensions = measure_text(space_text, None, 20, 1.0);
                let space_height = screen_height() / 2.0 - 10.0;
                draw_text(
                    space_text,
                    screen_width() / 2.0 - space_dimensions.width / 2.0,
                    space_height,
                    20.0,
                    WHITE,
                );

                let escape_text = "Press ESC to return to main menu";
                let escape_dimensions = measure_text(escape_text, None, 20, 1.0);
                draw_text(
                    escape_text,
                    screen_width() / 2.0 - escape_dimensions.width / 2.0,
                    space_height + 30.0,
                    20.0,
                    WHITE,
                );
            }
            GameState::Settings => {
                let delta = get_frame_time();
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
                settings_sound_effect_throttler.update(delta);
                starfield_speed = StarfieldSpeed::Slow;
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        let prev_music_volume = settings.music_volume;
                        let prev_effect_volume = settings.effect_volume;
                        ui.label(vec2(80.0, -34.0), "Settings");
                        ui.slider(hash!(), "Music", 0f32..1f32, &mut settings.music_volume);
                        ui.slider(hash!(), "Effects", 0f32..1f32, &mut settings.effect_volume);

                        if settings.music_volume != prev_music_volume {
                            set_sound_volume(&resources.theme_music, settings.music_volume);
                        }

                        if settings.effect_volume != prev_effect_volume {
                            settings_sound_effect_throttler.run_action(|| {
                                play_sound(
                                    &resources.sound_laser,
                                    PlaySoundParams {
                                        looped: false,
                                        volume: settings.effect_volume,
                                    },
                                );
                            })
                        }

                        if ui.button(vec2(80.0, 160.0), "Back") {
                            game_state = GameState::MainMenu;
                        }
                    },
                );
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
                starfield_speed = StarfieldSpeed::Slow;
                let text = "GAME OVER!";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    RED,
                );
            }
        }

        next_frame().await
    }
}
