use bevy::prelude::*;

use crate::game_state::GameState;

use super::main_menu::MainMenuPlugin;
use super::settings::SettingsPlugin;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::MainMenu), menu_setup)
            .add_plugins((MainMenuPlugin, SettingsPlugin))
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::MainMenu)),
            );
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    //SettingsDisplay,
    //SettingsSound,
    Disabled,
}

// All actions that can be triggered from a button click
#[derive(Component, Default)]
pub enum MenuButtonAction {
    Play,
    Settings,
    // SettingsDisplay,
    // SettingsSound,
    BackToMainMenu,
    //BackToSettings,
    Quit,
    #[default]
    Noop,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Playing);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                }

                // MenuButtonAction::SettingsDisplay => {
                //     menu_state.set(MainMenuState::SettingsDisplay);
                // }
                // MenuButtonAction::SettingsSound => {
                //     menu_state.set(MainMenuState::SettingsSound);
                // }
                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main);
                } // MenuButtonAction::BackToSettings => {
                //     menu_state.set(MainMenuState::Settings);
                // }
                MenuButtonAction::Noop => (),
            }
        }
    }
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image, selected) in &mut interaction_query {
        if let Some(atlas) = &mut image.texture_atlas {
            atlas.index = match (*interaction, selected) {
                (Interaction::Pressed, _) => 1,
                (Interaction::Hovered, Some(_)) => 0,
                (Interaction::Hovered, None) => 0,
                (Interaction::None, Some(_)) => 0,
                (Interaction::None, None) => 0,
            }
        }
    }
}
