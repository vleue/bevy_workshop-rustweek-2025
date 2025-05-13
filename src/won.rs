use bevy::{color::palettes, prelude::*};

use crate::GameState;

pub fn won_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Won), display_text)
        .add_systems(Update, back_to_menu.run_if(in_state(GameState::Won)));
}

fn display_text(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("You Won!"),
                TextFont {
                    font_size: 200.0,
                    ..default()
                },
                TextColor::from(palettes::tailwind::RED_600),
            ),
            (
                Text::new("Press any key to go back to the menu"),
                TextFont {
                    font_size: 50.0,
                    ..default()
                },
                TextColor::from(palettes::tailwind::RED_800),
            )
        ],
        StateScoped(GameState::Won),
    ));
}

fn back_to_menu(keyboard: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keyboard.get_just_pressed().next().is_some() {
        next.set(GameState::StartMenu);
    }
}
