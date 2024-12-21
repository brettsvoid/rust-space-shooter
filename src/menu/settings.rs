use bevy::prelude::*;

use crate::systems::despawn_screen;

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

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn settings_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    volume: Res<Volume>,
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

                    p.spawn(get_text_node(&asset_server, "Volume"));
                    // for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                    //     let mut entity = p.spawn((get_button_node(
                    //         &asset_server,
                    //         &mut texture_atlases,
                    //         Volume(volume_setting),
                    //     ),));
                    //     if *volume == Volume(volume_setting) {
                    //         entity.insert(SelectedOption);
                    //     }
                    // }

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

fn settings() {}
