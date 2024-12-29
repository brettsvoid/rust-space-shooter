use bevy::prelude::*;

use crate::{systems::despawn_screen, theme::Palette, AppState};

use super::{
    menu::{MenuButtonAction, MenuState},
    utils::{get_background_node, get_button_node, get_text_node},
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(Update, main_menu.run_if(in_state(MenuState::Main)))
            .add_systems(OnExit(MenuState::Main), despawn_screen::<MainMenuScreen>)
            .add_systems(OnExit(AppState::Menu), despawn_screen::<MainMenuScreen>);
    }
}

#[derive(Component)]
struct MainMenuScreen;

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_font = asset_server.load("../assets/atari_games.ttf");

    // Add menu entities
    commands
        .spawn((
            MainMenuScreen,
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
                    // get_text_node(&asset_server, "Space Shooter"),
                    p.spawn((
                        Text::new("Space Shooter"),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Palette::TEXT_PRIMARY),
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

                    p.spawn(get_button_node(
                        &asset_server,
                        &mut texture_atlases,
                        MenuButtonAction::Play,
                    ))
                    .with_children(|p| {
                        p.spawn(get_text_node(&asset_server, "New Game"));
                    });

                    p.spawn(get_button_node(
                        &asset_server,
                        &mut texture_atlases,
                        MenuButtonAction::Settings,
                    ))
                    .with_children(|p| {
                        p.spawn(get_text_node(&asset_server, "Settings"));
                    });

                    p.spawn(get_button_node(
                        &asset_server,
                        &mut texture_atlases,
                        MenuButtonAction::Quit,
                    ))
                    .with_children(|p| {
                        p.spawn(get_text_node(&asset_server, "Quit"));
                    });
                });
        });
}

fn main_menu() {}
