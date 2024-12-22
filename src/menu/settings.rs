use bevy::{
    prelude::*,
    ui::{measurement, MeasureArgs, UiRect},
};

use crate::{settings::Settings, systems::despawn_screen};

use super::{
    menu::{MenuButtonAction, MenuState},
    utils::{get_background_node, get_button_node, get_text_node},
};

pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Settings), settings_setup)
            .add_systems(Update, settings.run_if(in_state(MenuState::Settings)))
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<SettingsScreen>,
            );
    }
}

#[derive(Component)]
struct SettingsScreen;

#[derive(Component)]
enum VolumeControl {
    Music,
    Effects,
}

#[derive(Component)]
struct Slider;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn settings_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_font = asset_server.load("../assets/atari_games.ttf");

    // Add menu entities
    commands
        .spawn((
            SettingsScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(get_background_node(&asset_server))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Settings"),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::new(
                                Val::Px(16.0),
                                Val::Px(16.0),
                                Val::Px(32.0),
                                Val::Px(12.0),
                            ),
                            ..default()
                        },
                    ));

                    // Music Volume Control
                    p.spawn(get_text_node(&asset_server, "Music Volume"));
                    p.spawn((
                        VolumeControl::Music,
                        Button,
                        Interaction::default(),
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(20.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Slider,
                            Node {
                                width: Val::Percent(50.0), // Initial value
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                        ));
                    });

                    // Effects Volume Control
                    p.spawn(get_text_node(&asset_server, "Effects Volume"));
                    p.spawn((
                        VolumeControl::Effects,
                        Button,
                        Interaction::default(),
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(20.0),
                            margin: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Slider,
                            Node {
                                width: Val::Percent(50.0), // Initial value
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                        ));
                    });

                    p.spawn(get_button_node(
                        &asset_server,
                        &mut texture_atlases,
                        MenuButtonAction::BackToMainMenu,
                    ))
                    .with_children(|p| {
                        p.spawn(get_text_node(&asset_server, "Back"));
                    });
                });
        });
}

fn settings(
    interaction_query: Query<(
        &Interaction,
        &VolumeControl,
        &Children,
        &GlobalTransform,
        &ComputedNode,
    )>,
    mut node_query: Query<&mut Node>,
    mut settings: ResMut<Settings>,
    windows: Query<&Window>,
) {
    let window = windows.single();

    for (interaction, volume_control, children, global_transform, computed_node) in
        interaction_query.iter()
    {
        if let Interaction::Pressed = interaction {
            if let Some(cursor_position) = window.cursor_position() {
                let node_width = computed_node.size()[0] * computed_node.inverse_scale_factor();
                let half_width = node_width / 2.0;
                let left_side = global_transform.translation().x
                    * computed_node.inverse_scale_factor()
                    - half_width;
                let relative_x = cursor_position.x - left_side;
                let volume = (relative_x / node_width).clamp(0.0, 1.0);

                // Update the slider visual
                if let Some(slider) = children.first() {
                    if let Ok(mut slider_node) = node_query.get_mut(*slider) {
                        slider_node.width = Val::Percent(volume * 100.0);
                    }
                }

                // Update the settings
                match volume_control {
                    VolumeControl::Music => settings.set_music_volume(volume),
                    VolumeControl::Effects => settings.set_effect_volume(volume),
                }
            }
        }
    }
}
