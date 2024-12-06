use macroquad::math::vec2;
use macroquad_particles::{self as particles, AtlasConfig};

pub fn explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.2,
        lifetime_randomness: 0.3,
        explosiveness: 0.4,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 400.0,
        initial_velocity_randomness: 0.8,
        gravity: vec2(0.0, -1000.0),
        size: 16.0,
        size_randomness: 0.3,
        atlas: Some(AtlasConfig::new(5, 1, 0..)),
        ..Default::default()
    }
}
