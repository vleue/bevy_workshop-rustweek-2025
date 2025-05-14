use avian2d::{PhysicsPlugins, prelude::Gravity};
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use level::Level;

mod audio;
mod game;
mod hud;
mod level;
mod splash;
mod starfield;
mod start_menu;
mod won;

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
        .add_plugins((PhysicsPlugins::default(), EnhancedInputPlugin))
        .insert_resource(Gravity::ZERO)
        .add_plugins((
            splash::splash_plugin,
            start_menu::menu_plugin,
            game::game_plugin,
            level::level_loader_plugin,
            hud::hud_plugin,
            won::won_plugin,
            audio::audio_plugin,
            starfield::starfield_plugin,
        ))
        .run();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Splash,
    StartMenu,
    Game,
    Won,
}

#[derive(Resource)]
struct GameAssets {
    player_ship: Handle<Image>,
    asteroid: Handle<Image>,
    jets: Handle<Image>,
    explosion: Handle<Image>,
    laser: Handle<Image>,
}

#[derive(Resource)]
pub struct LoadedLevel {
    pub level: Handle<Level>,
}

#[derive(Resource)]
pub struct AudioAssets {
    pub laser: Handle<AudioSource>,
    ship_explosion: Handle<AudioSource>,
    asteroid_explosion: Handle<AudioSource>,
    start: Handle<AudioSource>,
    win: Handle<AudioSource>,
    lose: Handle<AudioSource>,
    game_loop: Handle<AudioSource>,
}
