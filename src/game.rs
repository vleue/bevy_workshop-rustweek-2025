use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;

use crate::{GameAssets, GameState};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            Update,
            (control_player, collision, inertia, tick_explosion).run_if(in_state(GameState::Game)),
        );
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid {
    direction: Vec2,
    speed: f32,
}

#[derive(Component)]
struct Explosion(Timer);

fn display_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        Player,
        StateScoped(GameState::Game),
        children![(
            Sprite::from_image(game_assets.jets.clone()),
            Transform::from_xyz(0.0, -40.0, -1.0),
            Visibility::Hidden,
        ),],
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
    mut player: Query<(&mut Transform, &Children), With<Player>>,
    time: Res<Time>,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let Ok((mut player_transform, children)) = player.single_mut() else {
        return Ok(());
    };

    let fixed_rotation_rate = 0.2;
    let rotation_rate = fixed_rotation_rate / (1.0 / (60.0 * time.delta().as_secs_f32()));

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_transform.rotate_z(rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player_transform.rotate_z(-rotation_rate);
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = player_transform.local_y();
        player_transform.translation += forward * 5.0;
        visibility
            .get_mut(children[0])?
            .set_if_neq(Visibility::Visible);
    } else {
        visibility
            .get_mut(children[0])?
            .set_if_neq(Visibility::Hidden);
    }

    Ok(())
}

fn collision(
    asteroids: Query<&Transform, With<Asteroid>>,
    player: Query<(&Transform, Entity), With<Player>>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) -> Result {
    let player_radius = 40.0;
    let asteroid_radius = 50.0;
    let Ok((player_transform, player_entity)) = player.single() else {
        return Ok(());
    };
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
            commands.spawn((
                Sprite::from_image(game_assets.explosion.clone()),
                player_transform.clone().with_scale(Vec3::splat(0.2)),
                Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
                StateScoped(GameState::Game),
            ));
            commands.entity(player_entity).despawn();
        }
    }

    Ok(())
}

fn inertia(mut asteroids: Query<(&mut Transform, &Asteroid)>) {
    for (mut asteroid_transform, asteroid) in asteroids.iter_mut() {
        asteroid_transform.translation += (asteroid.direction * asteroid.speed).extend(0.0);
    }
}

fn tick_explosion(
    mut explosions: Query<&mut Explosion>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for mut timer in explosions.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            next_state.set(GameState::StartMenu);
        }
    }
}
