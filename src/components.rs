#[derive(Component, Clone, Debug)]
pub struct PlayerStats {
    pub fire_rate: f32,
    pub speed: f32,
}
impl PlayerStats {
    pub fn new(fire_rate: f32, speed: f32) -> Self {
        Self { fire_rate, speed }
    }

    // Reset stats to default values (useful when powerups wear off)
    pub fn reset(&mut self) {
        self.fire_rate = 1.0;
        self.speed = 1.0;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            fire_rate: 1.0, // Base fire rate multiplier
            speed: 1.0,     // Base speed multiplier
        }
    }
}

use std::time::Duration;

use bevy::prelude::*;

/// Component for handling movement input direction
#[derive(Component)]
pub struct MovementInput {
    pub direction: Vec2,
}

/// Component for entity movement speed
#[derive(Component, Debug)]
pub struct MovementSpeed(pub f32);

/// Component for entity boundary constraints
#[derive(Component)]
pub struct Bounds {
    pub size: Vec2,
}

#[derive(Component, Debug)]
pub struct Shoot {
    pub is_shooting: bool,
    pub seconds: f32,
    pub timer: Timer,
}
impl Shoot {
    pub fn new(seconds: f32) -> Self {
        Self {
            is_shooting: false,
            seconds,
            timer: Self::timer_from_cooldown(seconds),
        }
    }

    pub fn timer_from_cooldown(seconds: f32) -> Timer {
        Timer::from_seconds(seconds, TimerMode::Once)
    }

    pub fn get_adjusted_cooldown(&self, fire_rate: f32) -> f32 {
        self.seconds / fire_rate
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Health(pub i32);
