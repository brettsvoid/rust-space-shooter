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
    pub timer: Timer,
}
impl Shoot {
    pub fn new(seconds: f32) -> Self {
        Self {
            is_shooting: false,
            timer: Self::timer_from_cooldown(seconds),
        }
    }

    pub fn timer_from_cooldown(seconds: f32) -> Timer {
        Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Health(pub i32);
