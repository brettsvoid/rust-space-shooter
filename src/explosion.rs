use bevy::{audio::Volume, prelude::*};

use crate::{
    audio::GameSounds,
    game_state::GameState,
    settings::Settings,
    sprite_animation::{AnimationConfig, SPRITE_FPS},
};

pub struct ExplosionPlugin;
impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DestroyedEvent>().add_systems(
            Update,
            (handle_destroy_event, update_explosion_animation).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Explosion;

pub struct DestroyedData {
    pub position: Vec3,
}

#[derive(Event)]
pub struct DestroyedEvent(pub DestroyedData);

fn handle_destroy_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    game_sounds: Res<GameSounds>,
    mut destroyed_event: EventReader<DestroyedEvent>,
    settings: Res<Settings>,
) {
    let explosion_image = asset_server.load("../assets/explosion.png");

    let explosion_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 1, None, None);
    let explosion_atlas_handle = texture_atlases.add(explosion_atlas);

    for event in destroyed_event.read() {
        commands.spawn((
            Explosion,
            AudioPlayer::new(game_sounds.explosion.clone()),
            PlaybackSettings {
                volume: Volume::new(settings.effect_volume),
                ..default()
            },
            Transform::from_translation(event.0.position),
            Sprite {
                image: explosion_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: explosion_atlas_handle.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::splat(16.0) * 2.0),
                ..default()
            },
            AnimationConfig::new(0, 4, SPRITE_FPS),
        ));
    }
}

fn update_explosion_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationConfig, &mut Sprite), With<Explosion>>,
) {
    for (entity, mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= config.last_sprite_index {
                    // ...and it IS the last frame, then we despawn the explosion
                    commands.entity(entity).despawn();
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                }
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}
