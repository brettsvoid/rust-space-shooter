use bevy::prelude::*;

/// Component for handling movement input direction
#[derive(Component)]
pub struct MovementInput {
    pub direction: Vec2,
}

/// Component for entity movement speed
#[derive(Component)]
pub struct MovementSpeed(pub f32);

/// Component for entity boundary constraints
#[derive(Component)]
pub struct Bounds {
    pub size: Vec2,
}
