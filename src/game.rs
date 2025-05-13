use std::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{GameAssets, GameState, LoadedLevel, level::Level};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            Update,
            (control_player, collision, tick_explosion).run_if(in_state(GameState::Game)),
        );
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid;

#[derive(Component)]
struct Explosion(Timer);

fn display_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    loaded_level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&loaded_level.level).unwrap();

    commands.spawn((
        Sprite::from_image(game_assets.player_ship.clone()),
        RigidBody::Dynamic,
        Collider::circle(40.0),
        AngularDamping(5.0),
        Player,
        StateScoped(GameState::Game),
        children![(
            Sprite::from_image(game_assets.jets.clone()),
            Transform::from_xyz(0.0, -40.0, -1.0),
            Visibility::Hidden,
        )],
    ));
    let mut rng = rand::thread_rng();

    for (x, y) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
            RigidBody::Dynamic,
            Collider::circle(50.0),
            LinearVelocity(Vec2::from_angle(rng.gen_range(0.0..TAU)) * rng.gen_range(10.0..100.0)),
            AngularVelocity(rng.gen_range(-1.5..1.5)),
            Asteroid,
            StateScoped(GameState::Game),
        ));
    }
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            &Transform,
            &mut AngularVelocity,
            &mut LinearVelocity,
            &Children,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let Ok((player_transform, mut angular_velocity, mut linear_velocity, children)) =
        player.single_mut()
    else {
        return Ok(());
    };

    let fixed_rotation_rate = 0.2;
    let rotation_rate = fixed_rotation_rate / (1.0 / (60.0 * time.delta().as_secs_f32()));

    if keyboard_input.pressed(KeyCode::KeyA) {
        angular_velocity.0 += rotation_rate;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        angular_velocity.0 -= rotation_rate;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = player_transform.local_y();
        linear_velocity.0 += forward.xy() * 2.0;
        linear_velocity.0 = linear_velocity.0.clamp_length_max(300.0);
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
    collisions: Collisions,
    player: Query<(&Transform, Entity), With<Player>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) -> Result {
    let Ok((transform, entity)) = player.single() else {
        return Ok(());
    };

    if collisions.collisions_with(entity).next().is_some() {
        commands.spawn((
            Sprite::from_image(game_assets.explosion.clone()),
            (*transform).with_scale(Vec3::splat(0.2)),
            Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
            StateScoped(GameState::Game),
        ));
        commands.entity(entity).despawn();
    }

    Ok(())
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
