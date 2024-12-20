use bevy::{prelude::*, ui::widget::NodeImageMode};

use super::menu::MenuButtonAction;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn get_background_node(asset_server: &Res<AssetServer>) -> impl Bundle {
    let background_image = asset_server.load("../assets/window_background.png");

    let background_slicer = TextureSlicer {
        border: BorderRect::square(104.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    (
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
    )
}

pub fn get_button_node(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    action: MenuButtonAction,
) -> impl Bundle {
    let button_image = asset_server.load("../assets/button-background.png");

    let button_atlas = TextureAtlasLayout::from_grid(UVec2::splat(36), 2, 1, None, None);
    let button_atlas_handle = texture_atlases.add(button_atlas);

    let button_slicer = TextureSlicer {
        border: BorderRect::square(17.0),
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

    (
        Button,
        ImageNode::from_atlas_image(
            button_image.clone(),
            TextureAtlas::from(button_atlas_handle.clone()),
        )
        .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
        button_node.clone(),
        action,
    )
}

pub fn get_text_node(asset_server: &Res<AssetServer>, text: &str) -> impl Bundle {
    let text_font = asset_server.load("../assets/atari_games.ttf");

    let button_text_font = TextFont {
        font: text_font.clone(),
        font_size: 36.0,
        ..default()
    };

    (
        Text::new(text),
        button_text_font.clone(),
        TextColor(TEXT_COLOR),
    )
}
