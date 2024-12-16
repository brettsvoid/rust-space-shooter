use bevy::{color::palettes::css::CRIMSON, prelude::*, ui::widget::NodeImageMode};

use crate::{game_state::GameState, systems::despawn_screen};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
            .add_systems(Update, main_menu.run_if(in_state(GameState::MainMenu)))
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_screen::<MainMenuScreen>,
            );
    }
}

#[derive(Component)]
struct MainMenuScreen;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MainMenuState {
    Main,
    Settings,
    //SettingsDisplay,
    //SettingsSound,
    #[default]
    Disabled,
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_image = asset_server.load("../assets/window_background.png");
    let button_image = asset_server.load("../assets/button_background.png");
    let text_font = asset_server.load("../assets/atari_games.ttf");

    let background_slicer = TextureSlicer {
        border: BorderRect::square(12.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    let button_slicer = TextureSlicer {
        border: BorderRect::square(10.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    // Common style for all buttons on the screen
    let button_node = Node {
        //width: Val::Px(.0),
        height: Val::Px(48.0),
        margin: UiRect::axes(Val::Px(16.0), Val::Px(4.0)),
        padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_font = TextFont {
        font: text_font.clone(),
        font_size: 36.0,
        ..default()
    };

    // Add menu entities
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            (),
        ))
        .with_children(|parent| {
            // First create a `Node` for centering what we want to display
            parent
                .spawn((
                    ImageNode {
                        image: background_image.clone(),
                        image_mode: NodeImageMode::Sliced(background_slicer.clone()),
                        ..default()
                    },
                    Node {
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(16.0)),
                        ..default()
                    },
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Space Shooter"),
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

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    p.spawn((
                        Button,
                        ImageNode {
                            image: button_image.clone(),
                            image_mode: NodeImageMode::Sliced(button_slicer.clone()),
                            ..default()
                        },
                        button_node.clone(),
                        MenuButtonAction::Play,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ));
                    });
                    p.spawn((
                        Button,
                        ImageNode {
                            image: button_image.clone(),
                            image_mode: NodeImageMode::Sliced(button_slicer.clone()),
                            ..default()
                        },
                        button_node.clone(),
                        MenuButtonAction::Settings,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ));
                    });
                    p.spawn((
                        Button,
                        ImageNode {
                            image: button_image.clone(),
                            image_mode: NodeImageMode::Sliced(button_slicer.clone()),
                            ..default()
                        },
                        button_node.clone(),
                        MenuButtonAction::Quit,
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Text::new("Quit"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ));
                    });
                });
        });
}

fn main_menu() {}
