use crate::{game_state::GameState, theme::Palette};
use bevy::prelude::*;

const GAME_OVER_FONT_SIZE: f32 = 80.0;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOverText;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over);
    }
}

fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("../assets/atari_games.ttf");

    commands
        .spawn((
            GameOverText,
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
                Text::new("GAME OVER"),
                TextFont {
                    font: font.clone(),
                    font_size: GAME_OVER_FONT_SIZE,
                    ..default()
                },
                TextColor(Palette::TEXT_GAME_OVER),
            ));

            parent.spawn((
                Text::new("Press `R` to restart"),
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

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
