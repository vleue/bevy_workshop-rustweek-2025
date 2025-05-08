use std::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use rand::Rng;

use crate::{GameAssets, GameState, LoadedLevel, level::Level};

pub fn game_plugin(app: &mut App) {
    app.add_input_context::<ShipController>()
        .add_systems(OnEnter(GameState::Game), display_level)
        // .add_systems(
        //     FixedUpdate,
        //     // (control_player, move_player).run_if(in_state(GameState::Game)),
        //     control_player.run_if(in_state(GameState::Game)),
        // )
        .add_systems(
            Update,
            (collision, tick_explosion).run_if(in_state(GameState::Game)),
        );
    // .add_observer(rotate);
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
pub struct LivesRemaining(pub u32);

// #[derive(Component)]
// struct PlayerVelocity(Vec2);

#[derive(Component)]
pub struct Asteroid;
// {
//     direction: Vec2,
//     speed: f32,
// }

#[derive(Component)]
struct Explosion(Timer);

fn display_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    loaded_level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&loaded_level.level).unwrap();

    commands.insert_resource(LivesRemaining(level.lives - 1));

    // commands.spawn((
    //     Sprite::from_image(game_assets.player_ship.clone()),
    //     RigidBody::Dynamic,
    //     Collider::circle(40.0),
    //     AngularDamping(5.0),
    //     Player,
    //     // PlayerVelocity(Vec2::ZERO),
    //     StateScoped(GameState::Game),
    //     children![(
    //         Sprite::from_image(game_assets.jets.clone()),
    //         Transform::from_xyz(0.0, -40.0, -1.0),
    //         Visibility::Hidden,
    //     )],
    // ));
    spawn_player(&mut commands, game_assets.as_ref());

    let mut rng = rand::thread_rng();
    for (x, y) in std::iter::repeat(())
        .filter_map(|_| {
            let x = rng.gen_range(-(level.width as f32) / 2.0..(level.width as f32) / 2.0);
            let y = rng.gen_range(-(level.height as f32) / 2.0..(level.height as f32) / 2.0);

            if Vec2::new(x, y).distance(Vec2::ZERO) < 200.0 {
                return None;
            }

            Some((x, y))
        })
        .take(level.asteroids as usize)
        .collect::<Vec<_>>()
    {
        commands.spawn((
            Sprite::from_image(game_assets.asteroid.clone()),
            Transform::from_xyz(x, y, 0.0),
            RigidBody::Dynamic,
            Collider::circle(45.0),
            LinearVelocity(Vec2::from_angle(rng.gen_range(0.0..TAU)) * rng.gen_range(50.0..100.0)),
            AngularVelocity(rng.gen_range(-2.0..2.0)),
            // Asteroid {
            //     direction: Vec2::from_angle(rng.gen_range(0.0..TAU)),
            //     speed: rng.gen_range(0.5..2.0),
            // },
            Asteroid,
            StateScoped(GameState::Game),
        ));
    }
    // let mut rng = rand::thread_rng();
    // for (x, y) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
    //     commands.spawn((
    //         Sprite::from_image(game_assets.asteroid.clone()),
    //         Transform::from_xyz(300.0 * x, 200.0 * y, 0.0),
    //         RigidBody::Dynamic,
    //         Collider::circle(45.0),
    //         LinearVelocity(Vec2::from_angle(rng.gen_range(0.0..TAU)) * rng.gen_range(10.0..100.0)),
    //         AngularVelocity(rng.gen_range(-1.5..1.5)),
    //         // Asteroid {
    //         //     direction: Vec2::from_angle(rng.gen_range(0.0..TAU)),
    //         //     speed: rng.gen_range(0.5..2.0),
    //         // },
    //         Asteroid,
    //         StateScoped(GameState::Game),
    //     ));
    // }
}

// fn control_player(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut player: Query<(&mut Transform, &mut PlayerVelocity, &Children), With<Player>>,
//     mut visibility: Query<&mut Visibility>,
// ) -> Result {
//     let Ok((mut player_transform, mut player_velocity, children)) = player.single_mut() else {
//         // No player at the moment, skip control logic
//         return Ok(());
//     };
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         player_transform.rotate_z(FRAC_PI_8 / 4.0);
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         player_transform.rotate_z(-FRAC_PI_8 / 4.0);
//     }
//     if keyboard_input.pressed(KeyCode::KeyW) {
//         let forward = player_transform.local_y().xy();
//         player_velocity.0 = forward;
//         *visibility.get_mut(children[0])? = Visibility::Visible;
//     } else {
//         visibility
//             .get_mut(children[0])?
//             .set_if_neq(Visibility::Hidden);
//     }
//     Ok(())
// }
// fn control_player(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut player: Query<
//         (
//             &Transform,
//             &mut AngularVelocity,
//             &mut LinearVelocity,
//             &Children,
//         ),
//         With<Player>,
//     >,
//     mut visibility: Query<&mut Visibility>,
// ) -> Result {
//     let Ok((transform, mut angular_velocity, mut linear_velocity, children)) = player.single_mut()
//     else {
//         // No player at the moment, skip control logic
//         return Ok(());
//     };
//     if keyboard_input.pressed(KeyCode::KeyA) {
//         angular_velocity.0 += 0.2;
//     }
//     if keyboard_input.pressed(KeyCode::KeyD) {
//         angular_velocity.0 -= 0.2;
//     }
//     if keyboard_input.pressed(KeyCode::KeyW) {
//         linear_velocity.0 += transform.local_y().xy() * 2.0;
//         linear_velocity.0 = linear_velocity.0.clamp_length_max(200.0);
//         *visibility.get_mut(children[0])? = Visibility::Visible;
//     } else {
//         visibility
//             .get_mut(children[0])?
//             .set_if_neq(Visibility::Hidden);
//     }
//     Ok(())
// }

#[derive(InputContext)]
struct ShipController;

#[derive(Debug, InputAction)]
#[input_action(output = f32)]
struct Rotate;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct Thrust;

// fn move_player(mut player: Query<(&mut Transform, &PlayerVelocity)>) {
//     for (mut player_transform, player_velocity) in player.iter_mut() {
//         player_transform.translation += player_velocity.0.extend(0.0) * 5.0;
//     }
// }

// fn inertia(mut asteroids: Query<(&mut Transform, &Asteroid)>) {
//     for (mut asteroid_transform, asteroid) in asteroids.iter_mut() {
//         asteroid_transform.translation += (asteroid.direction * asteroid.speed).extend(0.0);
//     }
// }

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

// fn collision(
//     asteroids: Query<&Transform, With<Asteroid>>,
//     player: Query<(&Transform, Entity), With<Player>>,
//     mut gizmos: Gizmos,
//     mut commands: Commands,
//     game_assets: Res<GameAssets>,
// ) -> Result {
//     let player_radius = 40.0;
//     let asteroid_radius = 50.0;
//     let Ok((player_transform, player_entity)) = player.single() else {
//         return Ok(());
//     };
//     gizmos.circle_2d(
//         player_transform.translation.xy(),
//         player_radius,
//         Color::linear_rgb(1.0, 0.0, 0.0),
//     );
//     for asteroid_transform in &asteroids {
//         gizmos.circle_2d(
//             asteroid_transform.translation.xy(),
//             asteroid_radius,
//             Color::linear_rgb(0.0, 0.0, 1.0),
//         );
//         let distance = asteroid_transform
//             .translation
//             .distance(player_transform.translation);
//         if distance < (asteroid_radius + player_radius) {
//             commands.spawn((
//                 Sprite::from_image(game_assets.explosion.clone()),
//                 player_transform.clone().with_scale(Vec3::splat(0.2)),
//                 Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
//                 StateScoped(GameState::Game),
//             ));
//             commands.entity(player_entity).despawn();
//         }
//     }

//     Ok(())
// }

fn tick_explosion(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut lives_remaining: ResMut<LivesRemaining>,
    mut explosions: Query<(Entity, &mut Explosion)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, mut timer) in explosions.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if lives_remaining.0 == 0 {
                next_state.set(GameState::StartMenu);
            } else {
                commands.entity(entity).despawn();
                lives_remaining.0 -= 1;
                spawn_player(&mut commands, game_assets.as_ref());
            }
        }
    }
}

fn spawn_player(commands: &mut Commands, game_assets: &GameAssets) {
    let mut actions = Actions::<ShipController>::default();

    actions.bind::<Rotate>().to(Bidirectional {
        positive: KeyCode::KeyA,
        negative: KeyCode::KeyD,
    });
    actions.bind::<Thrust>().to(KeyCode::KeyW);

    commands
        .spawn((
            Sprite::from_image(game_assets.player_ship.clone()),
            RigidBody::Dynamic,
            Collider::circle(40.0),
            AngularDamping(5.0),
            Player,
            // PlayerVelocity(Vec2::ZERO),
            StateScoped(GameState::Game),
            children![(
                Sprite::from_image(game_assets.jets.clone()),
                Transform::from_xyz(0.0, -40.0, -1.0),
                Visibility::Hidden,
            )],
            actions,
        ))
        .observe(rotate)
        .observe(thrust)
        .observe(thrust_stop);
}

fn rotate(
    trigger: Trigger<Fired<Rotate>>,
    mut player: Query<&mut AngularVelocity, With<Player>>,
    time: Res<Time>,
) -> Result {
    let fixed_rate = 0.2;
    let delta = time.delta().as_secs_f32();
    let rate = fixed_rate / (1.0 / (60.0 * delta));
    let mut angular_velocity = player.get_mut(trigger.target())?;
    angular_velocity.0 += trigger.value.signum() * rate;

    Ok(())
}

fn thrust(
    trigger: Trigger<Fired<Thrust>>,
    mut player: Query<(&Transform, &mut LinearVelocity, &Children), With<Player>>,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let (transform, mut linear_velocity, children) = player.get_mut(trigger.target())?;
    linear_velocity.0 += transform.local_y().xy() * 2.0;
    linear_velocity.0 = linear_velocity.0.clamp_length_max(200.0);
    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Visible);
    Ok(())
}

fn thrust_stop(
    trigger: Trigger<Completed<Thrust>>,
    player: Query<&Children, With<Player>>,
    mut visibility: Query<&mut Visibility>,
) -> Result {
    let children = player.get(trigger.target())?;

    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Hidden);

    Ok(())
}
