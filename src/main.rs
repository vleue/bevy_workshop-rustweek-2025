use bevy::prelude::*;

mod game;
mod level;
mod splash;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                canvas: Some("#game".into()),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins((splash::SplashPlugin, game::game_plugin)) // adding our new plugin
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    // Menu,
    Game,
}

#[derive(Resource)]
struct GameAssets {
    player_ship: Handle<Image>,
    asteroid: Handle<Image>,
}
