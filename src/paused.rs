use crate::{game_state::GameState, theme::Palette};
use bevy::prelude::*;

const PAUSED_FONT_SIZE: f32 = 80.0;

pub struct PausedPlugin;
impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), setup_paused)
            .add_systems(OnExit(GameState::Paused), cleanup_paused);
    }
}

#[derive(Component)]
struct PausedText;

fn setup_paused(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("../assets/atari_games.ttf");

    commands
        .spawn((
            PausedText,
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font: font.clone(),
                    font_size: PAUSED_FONT_SIZE,
                    ..default()
                },
                TextColor(Palette::TEXT_PAUSED),
            ));

            parent.spawn((
                Text::new("Press `Esc` to unpause"),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Palette::TEXT_PRIMARY),
                Node {
                    margin: UiRect {
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

fn cleanup_paused(mut commands: Commands, query: Query<Entity, With<PausedText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
