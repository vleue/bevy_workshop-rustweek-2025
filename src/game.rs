use std::f32::consts::{FRAC_PI_8, TAU};

use bevy::prelude::*;
use rand::Rng;

use crate::{GameAssets, GameState};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            FixedUpdate,
            (control_player, inertia).run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, collision.run_if(in_state(GameState::Game)));
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid {
    direction: Vec2,
    speed: f32,
}

fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Player,
        StateScoped(GameState::Game),
    ));

    let mut rng = rand::thread_rng();

    for (x, y) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            Asteroid {
                direction: Vec2::from_angle(rng.gen_range(0.0..TAU)),
                speed: rng.gen_range(0.5..2.0),
            },
            StateScoped(GameState::Game),
        ));
    }
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) -> Result {
    let mut player_transform = player.single_mut()?;
    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.rotate_z(FRAC_PI_8 / 4.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.rotate_z(-FRAC_PI_8 / 4.0);
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = player_transform.local_y();
        player_transform.translation += forward * 5.0;
    }
    Ok(())
}

fn inertia(mut asteroids: Query<(&mut Transform, &Asteroid)>) {
    for (mut asteroid_transform, asteroid) in asteroids.iter_mut() {
        asteroid_transform.translation += (asteroid.direction * asteroid.speed).extend(0.0);
    }
}

fn collision(
    asteroids: Query<&Transform, With<Asteroid>>,
    player: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
    mut next_state: ResMut<NextState<GameState>>,
) -> Result {
    let player_radius = 40.0;
    let asteroid_radius = 50.0;
    let player_transform = player.single()?;
    gizmos.circle_2d(
        player_transform.translation.xy(),
        player_radius,
        Color::linear_rgb(1.0, 0.0, 0.0),
    );
    for asteroid_transform in &asteroids {
        gizmos.circle_2d(
            asteroid_transform.translation.xy(),
            asteroid_radius,
            Color::linear_rgb(0.0, 0.0, 1.0),
        );
        let distance = asteroid_transform
            .translation
            .distance(player_transform.translation);
        if distance < (asteroid_radius + player_radius) {
            next_state.set(GameState::StartMenu);
        }
    }

    Ok(())
}
