use bevy::prelude::*;

use crate::{
    components::{Health, PlayerStats},
    game_state::GameState,
    player::Player,
    theme::Palette,
};

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (cleanup_hud, setup_hud).chain(),
        )
        .add_systems(Update, (update_hud).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct HudUi;

#[derive(Component)]
struct HudHealth;

#[derive(Component)]
struct HudFireRate;

#[derive(Component)]
struct HudSpeed;

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = asset_server.load("../assets/atari_games.ttf");
    commands
        .spawn((
            HudUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                bottom: Val::ZERO,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::new(
                        Val::Px(16.0),
                        Val::Px(16.0),
                        Val::Px(16.0),
                        Val::Px(16.0),
                    ),
                    ..default()
                },))
                .with_children(|p| {
                    p.spawn((
                        HudFireRate,
                        Text::new("Fire rate: "),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
                    ))
                    .with_child((
                        TextSpan::default(),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
                    ));

                    p.spawn((
                        HudSpeed,
                        Text::new("Speed: "),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
                    ))
                    .with_child((
                        TextSpan::default(),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
                    ));
                });

            parent
                .spawn((Node {
                    padding: UiRect::new(
                        Val::Px(16.0),
                        Val::Px(16.0),
                        Val::Px(16.0),
                        Val::Px(16.0),
                    ),
                    ..default()
                },))
                .with_children(|p| {
                    p.spawn((
                        HudHealth,
                        Text::new("Health: "),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
                    ))
                    .with_child((
                        TextSpan::default(),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
                    ));
                });
        });
}

fn update_hud(
    health_root: Single<Entity, (With<HudHealth>, With<Text>)>,
    fire_rate_root: Single<Entity, (With<HudFireRate>, With<Text>)>,
    speed_root: Single<Entity, (With<HudSpeed>, With<Text>)>,
    player_health: Single<&Health, With<Player>>,
    player_stats: Single<&PlayerStats, With<Player>>,
    mut writer: TextUiWriter,
) {
    *writer.text(*health_root, 1) = player_health.0.to_string();
    *writer.text(*fire_rate_root, 1) = player_stats.fire_rate.to_string();
    *writer.text(*speed_root, 1) = player_stats.speed.to_string();
}
