use bevy::prelude::*;

use crate::{game_state::GameState, theme::Palette};

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(
                OnEnter(GameState::Playing),
                ((cleanup_scoreboard, setup).chain(), reset_score),
            )
            .add_systems(
                Update,
                update_scoreboard.run_if(in_state(GameState::Playing)),
            );
    }
}

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
pub struct Score(usize);

#[derive(Component)]
struct ScoreboardUi;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = asset_server.load("../assets/atari_games.ttf");

    // MenuText {
    //     text: Text::new(text),
    //     font: button_text_font.clone(),
    //     ..default()
    // }

    // Scoreboard
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font: text_font.clone(),
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(Palette::TEXT_PRIMARY),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                right: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font: text_font.clone(),
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn cleanup_scoreboard(mut commands: Commands, query: Query<Entity, With<ScoreboardUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}
