use crate::game_state::GameState;
use bevy::prelude::*;

const GAME_OVER_FONT_SIZE: f32 = 80.0;
const TEXT_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);

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
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                GameOverText,
                Text::new("GAME OVER"),
                TextFont {
                    font: font.clone(),
                    font_size: GAME_OVER_FONT_SIZE,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
