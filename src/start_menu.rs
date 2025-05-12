use bevy::{color::palettes, prelude::*};

use crate::GameState;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::StartMenu), display_title)
        .add_systems(Update, start_game.run_if(in_state(GameState::StartMenu)));
}

fn display_title(mut commands: Commands) {
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
                Text::new("Asteroid"),
                TextFont {
                    font_size: 200.0,
                    ..default()
                },
                TextColor::from(palettes::tailwind::RED_600),
            ),
            (
                Text::new("Press any key to start"),
                TextFont {
                    font_size: 70.0,
                    ..default()
                },
                TextColor::from(palettes::tailwind::RED_800),
            )
        ],
        StateScoped(GameState::StartMenu),
    ));
}

fn start_game(keyboard: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keyboard.get_just_pressed().next().is_some() {
        next.set(GameState::Game);
    }
}
