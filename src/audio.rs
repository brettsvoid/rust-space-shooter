use bevy::audio::*;
use bevy::prelude::*;

use crate::settings::Settings;

pub struct GameAudioPlugin;

#[derive(Resource)]
pub struct GameSounds {
    pub shoot: Handle<AudioSource>,
    pub explosion: Handle<AudioSource>,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_audio)
            .add_systems(Update, update_volume);
    }
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    // Load and play background music
    let music = asset_server.load("../assets/8bit-spaceshooter.ogg");

    commands.spawn((
        AudioPlayer::new(music.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(settings.music_volume),
            ..default()
        },
    ));

    // Load sound effects
    let shoot_sound = asset_server.load("../assets/laser.wav");
    let explosion_sound = asset_server.load("../assets/explosion.wav");

    commands.insert_resource(GameSounds {
        shoot: shoot_sound,
        explosion: explosion_sound,
    });
}

fn update_volume(settings: Res<Settings>, mut audio_query: Query<&mut AudioSink>) {
    for sink in audio_query.iter_mut() {
        sink.set_volume(settings.music_volume);
    }
}
