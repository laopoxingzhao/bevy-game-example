use bevy::prelude::*;

use crate::{components::Menu, stares::GameState};

pub fn start(mut cmd: Commands) {
    let menu_ui = (
        Name::new("Menu UI"),
        Menu,
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        // BackgroundColor(Color::srgb(0., 0.1, 0.3)),
        children![(
            Button,
            Node {
                width: Val::Px(160.),
                right: Val::Px(60.),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,//
                align_items: AlignItems::Center,
                top: Val::Px(0.),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0., 0.1, 0.3)),
            Name::new("Start Button"),
            children![(
                Name::new("Start Text"),
                Text::new("Play"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ),],
    );
    info!("Menu UI started");

    cmd.spawn(menu_ui);
}

pub fn handle_menu_ui(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Start Button Clicked");
                // 在这里处理开始游戏的逻辑，例如切换状态
                // commands.remove_resource::<Entity>();
                background_color.0 = Color::WHITE;
                commands.insert_resource(NextState::Pending(GameState::Playing));
            }
            Interaction::Hovered => {
                info!("Start Button Hovered");
                *background_color = Color::srgb(0., 0.3, 0.).into();
            }
            Interaction::None => {
                *background_color = Color::BLACK.into();
            }
        }
    }
}


pub fn cleanup(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("Menu UI cleaned up");
}