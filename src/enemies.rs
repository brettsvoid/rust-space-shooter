use macroquad::{
    color::WHITE,
    math::vec2,
    prelude::animation::{AnimatedSprite, Animation},
    texture::{draw_texture_ex, DrawTextureParams},
};

use crate::{resources::Resources, Shape};

struct EnemiesSprites {
    small: AnimatedSprite,
    medium: AnimatedSprite,
    large: AnimatedSprite,
}

pub struct Enemies {
    pub small: Vec<Shape>,
    pub medium: Vec<Shape>,
    pub large: Vec<Shape>,
    sprites: EnemiesSprites,
}

impl Enemies {
    pub fn new() -> Enemies {
        let enemy_small_sprite = AnimatedSprite::new(
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
        let enemy_medium_sprite = AnimatedSprite::new(
            32,
            16,
            &[Animation {
                name: "enemy_medium".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            }],
            true,
        );
        let enemy_large_sprite = AnimatedSprite::new(
            32,
            32,
            &[Animation {
                name: "enemy_large".to_string(),
                row: 0,
                frames: 2,
                fps: 12,
            }],
            true,
        );

        Enemies {
            small: vec![],
            medium: vec![],
            large: vec![],
            sprites: EnemiesSprites {
                small: enemy_small_sprite,
                medium: enemy_medium_sprite,
                large: enemy_large_sprite,
            },
        }
    }

    pub fn clear(&mut self) {
        self.small.clear();
        self.medium.clear();
        self.large.clear();
    }

    pub fn draw(&mut self, resources: &Resources) {
        let enemy_frame = self.sprites.small.frame();
        for enemy in &self.small {
            draw_texture_ex(
                &resources.enemy_small_texture,
                enemy.x - enemy.size / 2.0,
                enemy.y - enemy.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(enemy.size, enemy.size)),
                    source: Some(enemy_frame.source_rect),
                    ..Default::default()
                },
            );
        }
        let enemy_frame = self.sprites.medium.frame();
        for enemy in &self.medium {
            draw_texture_ex(
                &resources.enemy_medium_texture,
                enemy.x - enemy.size / 2.0,
                enemy.y - enemy.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(enemy.size, enemy.size)),
                    source: Some(enemy_frame.source_rect),
                    ..Default::default()
                },
            );
        }
        let enemy_frame = self.sprites.large.frame();
        for enemy in &self.large {
            draw_texture_ex(
                &resources.enemy_large_texture,
                enemy.x - enemy.size / 2.0,
                enemy.y - enemy.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(enemy.size, enemy.size)),
                    source: Some(enemy_frame.source_rect),
                    ..Default::default()
                },
            );
        }
    }

    pub fn movement(&mut self, delta: f32) {
        for enemy in &mut self.small {
            enemy.y += enemy.speed * delta;
        }
        for enemy in &mut self.medium {
            enemy.y += enemy.speed * delta;
        }
        for enemy in &mut self.large {
            enemy.y += enemy.speed * delta;
        }
    }

    pub fn update(&mut self) {
        self.sprites.small.update();
        self.sprites.medium.update();
        self.sprites.large.update();
    }
}
