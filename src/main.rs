use avian2d::{PhysicsPlugins, prelude::Gravity};
use bevy::prelude::*;
use level::Level;

mod game;
mod level;
mod splash;
mod start_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Workshop".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
        .add_plugins((
            splash::splash_plugin,
            start_menu::menu_plugin,
            game::game_plugin,
            level::level_loader_plugin,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    StartMenu,
    Game,
}

#[derive(Resource)]
struct GameAssets {
    player_ship: Handle<Image>,
    asteroid: Handle<Image>,
    jets: Handle<Image>,
    explosion: Handle<Image>,
}

#[derive(Resource)]
pub struct LoadedLevel {
    pub level: Handle<Level>,
}
