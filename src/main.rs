use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_enoki::{EnokiPlugin, Particle2dEffect};
use bevy_rapier2d::{
    math::Vect,
    plugin::{RapierConfiguration, RapierContextInitialization, RapierPhysicsPlugin},
    rapier::prelude::IntegrationParameters,
};
use level::Level;

mod audio;
mod game;
mod hud;
mod level;
mod rapier_events;
mod splash;
mod starfield;
mod start_menu;
mod won;

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
        .add_plugins((
            RapierPhysicsPlugin::<()>::default().with_custom_initialization(
                RapierContextInitialization::InitializeDefaultRapierContext {
                    // TODO: impl default for RapierConfiguration
                    rapier_configuration: RapierConfiguration {
                        gravity: Vect::ZERO,
                        physics_pipeline_active: true,
                        query_pipeline_active: true,
                        scaled_shape_subdivision: 10,
                        force_update_from_transform_changes: false,
                    },
                    integration_parameters: IntegrationParameters::default(),
                },
            ),
            EnhancedInputPlugin,
            EnokiPlugin,
        ))
        //.insert_resource(Gravity::ZERO)
        .add_plugins((
            splash::splash_plugin,
            start_menu::menu_plugin,
            game::game_plugin,
            level::level_loader_plugin,
            hud::hud_plugin,
            won::won_plugin,
            audio::audio_plugin,
            starfield::starfield_plugin,
            rapier_events::plugin,
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
    jet_particles: Handle<Particle2dEffect>,
}

#[derive(Resource)]
pub struct LoadedLevel {
    pub level: Handle<Level>,
}

#[derive(Resource)]
struct AudioAssets {
    laser: Handle<AudioSource>,
    ship_explosion: Handle<AudioSource>,
    asteroid_explosion: Handle<AudioSource>,
    start: Handle<AudioSource>,
    win: Handle<AudioSource>,
    lose: Handle<AudioSource>,
    game_loop: Handle<AudioSource>,
}
