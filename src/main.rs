use macroquad::audio::{play_sound, play_sound_once, set_sound_volume, PlaySoundParams};
use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use macroquad_particles::{Emitter, EmitterConfig};
use std::fs;
use std::ops::Range;

use crate::resources::Resources;
use crate::settings::Settings;

mod particle_effects;
mod resources;
mod settings;

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

enum GameState {
    MainMenu,
    Playing,
    Paused,
    Settings,
    GameOver,
}

#[macroquad::main("Space Shooter")]
async fn main() -> Result<(), macroquad::Error> {
    const MOVE_SPEED: f32 = 200.0;

    // Seed the random number generator
    rand::srand(miniquad::date::now() as u64);

    let mut settings = Settings::new();
    // Sound seems way too loud by default
    settings.set_music_volume(0.5);
    settings.set_sound_volume(0.5);

    let mut squares = vec![];
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
                    name: "iResolution".to_owned(),
                    uniform_type: UniformType::Float2,
                    array_count: 1,
                },
                UniformDesc {
                    name: "direction_modifier".to_owned(),
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
    let mut enemy_small_sprite = AnimatedSprite::new(
        17,
        16,
        &[Animation {
            name: "enemy_small".to_string(),
            row: 0,
            frames: 2,
            fps: 12,
        }],
        true,
    );

    play_sound(
        &resources.theme_music,
        PlaySoundParams {
            looped: true,
            volume: settings.music_volume,
        },
    );
    set_sound_volume(&resources.sound_explosion, settings.sound_volume);
    set_sound_volume(&resources.sound_laser, settings.sound_volume);

    loop {
        clear_background(DARKPURPLE);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
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
                            squares.clear();
                            bullets.clear();
                            explosions.clear();
                            circle.x = screen_width() / 2.0;
                            circle.y = screen_height() / 2.0;
                            score = 0;
                            game_state = GameState::Playing;
                        }
                        if ui.button(vec2(35.0, 90.0), "Settings") {
                            game_state = GameState::Settings;
                        }
                        if ui.button(vec2(80.0, 170.0), "Quit") {
                            std::process::exit(0);
                        }
                    },
                );
            }
            GameState::Playing => {
                let delta_time = get_frame_time();
                ship_sprite.set_animation(0);
                if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    circle.x += MOVE_SPEED * delta_time;
                    direction_modifier += 0.05 * delta_time;
                    ship_sprite.set_animation(2);
                }
                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    circle.x -= MOVE_SPEED * delta_time;
                    direction_modifier -= 0.05 * delta_time;
                    ship_sprite.set_animation(1);
                }
                if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    circle.y += MOVE_SPEED * delta_time;
                }
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    circle.y -= MOVE_SPEED * delta_time
                }
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: circle.x,
                        y: circle.y - 24.0,
                        speed: circle.speed * 2.0,
                        size: 32.0,
                        collided: false,
                    });
                    play_sound_once(&resources.sound_laser);
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Clamp X and Y to be within the screen
                circle.x = clamp(circle.x, 0.0, screen_width());
                circle.y = clamp(circle.y, 0.0, screen_height());

                // Generate a new square
                if rand::gen_range(0, 99) >= 95 {
                    let size = rand::gen_range(16.0, 64.0);
                    squares.push(Shape {
                        size,
                        speed: rand::gen_range(50.0, 150.0),
                        x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                        y: -size,
                        collided: false,
                    });
                }

                // Movement
                for square in &mut squares {
                    square.y += square.speed * delta_time;
                }
                for bullet in &mut bullets {
                    bullet.y -= bullet.speed * delta_time;
                }

                ship_sprite.update();
                bullet_sprite.update();
                enemy_small_sprite.update();

                // Remove shapes outside of screen
                squares.retain(|square| square.y < screen_height() + square.size);
                bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

                // Remove collided shapes
                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);

                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // Check for collisions
                if squares.iter().any(|square| circle.collides_with(square)) {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver;
                }
                for square in squares.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if bullet.collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.size.round() as u32 * 4,
                                    texture: Some(resources.explosion_texture.clone()),
                                    ..particle_effects::explosion()
                                }),
                                vec2(square.x, square.y),
                            ));
                            play_sound_once(&resources.sound_explosion);
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
                let enemy_frame = enemy_small_sprite.frame();
                for square in &squares {
                    draw_texture_ex(
                        &resources.enemy_small_texture,
                        square.x - square.size / 2.0,
                        square.y - square.size / 2.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(square.size, square.size)),
                            source: Some(enemy_frame.source_rect),
                            ..Default::default()
                        },
                    );
                }
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
                let text = "Paused";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::Settings => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
                root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        let prev_music_volume = settings.music_volume.clone();
                        let prev_sound_volume = settings.sound_volume.clone();
                        ui.label(vec2(80.0, -34.0), "Settings");
                        ui.slider(hash!(), "Music", 0f32..1f32, &mut settings.music_volume);
                        ui.slider(hash!(), "Effects", 0f32..1f32, &mut settings.sound_volume);

                        if settings.music_volume != prev_music_volume {
                            set_sound_volume(&resources.theme_music, settings.music_volume);
                        }

                        if settings.sound_volume != prev_sound_volume {
                            set_sound_volume(&resources.sound_explosion, settings.sound_volume);
                            set_sound_volume(&resources.sound_laser, settings.sound_volume);
                            //play_sound_once(&resources.sound_laser);
                        }

                        if ui.button(vec2(80.0, 170.0), "Back") {
                            game_state = GameState::MainMenu;
                        }
                    },
                );
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }
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
