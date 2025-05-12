use bevy::prelude::*;

use crate::{GameAssets, GameState};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            Update,
            (control_player, collision).run_if(in_state(GameState::Game)),
        );
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid;

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

    for (x, y) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            Asteroid,
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
    let (mut player_transform, children) = player.single_mut()?;

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
    player: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
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
            println!("Collision detected!");
        }
    }

    Ok(())
}
