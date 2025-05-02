use std::{f32::consts::TAU, time::Duration};

use avian2d::prelude::*;
use bevy::{audio::Volume, prelude::*};
use bevy_enhanced_input::prelude::*;
use bevy_enoki::prelude::*;
use rand::Rng;

use crate::{AudioAssets, GameAssets, GameState, LoadedLevel, audio::AudioStart, level::Level};

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
            (
                // collision,
                laser_range,
                tick_explosion,
                has_won,
                follow_player,
                closest,
            )
                .run_if(in_state(GameState::Game)),
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
    audio_assets: Res<AudioAssets>,
    loaded_level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&loaded_level.level).unwrap();

    commands.insert_resource(LivesRemaining(level.lives - 1));

    commands.spawn((
        AudioPlayer::<AudioSource>(audio_assets.game_loop.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Decibels(-5.0)),
        StateScoped(GameState::Game),
    ));

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
    spawn_player(&mut commands, game_assets.as_ref(), Vec2::ZERO);

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
            // CollisionEventsEnabled,
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

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
struct FireLaser;

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

// fn collision(
//     collisions: Collisions,
//     player: Query<(&Transform, Entity), With<Player>>,
//     mut commands: Commands,
//     game_assets: Res<GameAssets>,
// ) -> Result {
//     let Ok((transform, entity)) = player.single() else {
//         return Ok(());
//     };

//     if collisions.collisions_with(entity).next().is_some() {
//         commands.spawn((
//             Sprite::from_image(game_assets.explosion.clone()),
//             (*transform).with_scale(Vec3::splat(0.2)),
//             Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
//             StateScoped(GameState::Game),
//         ));
//         commands.entity(entity).despawn();
//     }

//     Ok(())
// }

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
    mut explosions: Query<(Entity, &mut Explosion, &Transform)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio: EventWriter<AudioStart>,
) {
    for (entity, mut timer, transform) in explosions.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            if lives_remaining.0 == 0 {
                next_state.set(GameState::StartMenu);
                audio.write(AudioStart::Lose);
            } else {
                commands.entity(entity).despawn();
                lives_remaining.0 -= 1;
                spawn_player(
                    &mut commands,
                    game_assets.as_ref(),
                    transform.translation.xy(),
                );
            }
        }
    }
}

fn spawn_player(commands: &mut Commands, game_assets: &GameAssets, position: Vec2) {
    let mut actions = Actions::<ShipController>::default();

    actions.bind::<Rotate>().to(Bidirectional {
        positive: KeyCode::KeyA,
        negative: KeyCode::KeyD,
    });
    actions.bind::<Thrust>().to(KeyCode::KeyW);
    actions.bind::<FireLaser>().to(KeyCode::Space);

    commands
        .spawn((
            Sprite::from_image(game_assets.player_ship.clone()),
            RigidBody::Dynamic,
            Collider::circle(40.0),
            AngularDamping(5.0),
            Player,
            Transform::from_translation(position.extend(0.0)),
            // PlayerVelocity(Vec2::ZERO),
            CollisionEventsEnabled,
            StateScoped(GameState::Game),
            children![
                (
                    // Sprite::from_image(game_assets.jets.clone()),
                    Sprite {
                        image: game_assets.jets.clone(),
                        color: Color::srgb(2.0, 2.0, 1.0),
                        ..default()
                    },
                    Transform::from_xyz(0.0, -40.0, -1.0),
                    Visibility::Hidden,
                ),
                (
                    ParticleSpawner::default(),
                    ParticleSpawnerState {
                        active: false,
                        ..default()
                    },
                    ParticleEffectHandle(game_assets.jet_particles.clone()),
                    Transform::from_xyz(0.0, -40.0, 0.0),
                )
            ],
            actions,
        ))
        .observe(rotate)
        .observe(thrust)
        .observe(thrust_stop)
        .observe(asteroid_collision)
        .observe(fire_laser);
}

fn rotate(
    trigger: Trigger<Fired<Rotate>>,
    mut player: Query<&mut AngularVelocity>,
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
    mut player: Query<(&Transform, &mut LinearVelocity, &Children)>,
    mut visibility: Query<&mut Visibility>,
    mut particle_state: Query<&mut ParticleSpawnerState>,
) -> Result {
    let (transform, mut linear_velocity, children) = player.get_mut(trigger.target())?;
    linear_velocity.0 += transform.local_y().xy() * 2.0;
    linear_velocity.0 = linear_velocity.0.clamp_length_max(300.0);
    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Visible);

    particle_state
        .get_mut(children[1])?
        .map_unchanged(|s| &mut s.active)
        .set_if_neq(true);

    Ok(())
}

fn thrust_stop(
    trigger: Trigger<Completed<Thrust>>,
    player: Query<&Children>,
    mut visibility: Query<&mut Visibility>,
    mut particle_state: Query<&mut ParticleSpawnerState>,
) -> Result {
    let Ok(children) = player.get(trigger.target()) else {
        return Ok(());
    };

    visibility
        .get_mut(children[0])?
        .set_if_neq(Visibility::Hidden);

    particle_state
        .get_mut(children[1])?
        .map_unchanged(|s| &mut s.active)
        .set_if_neq(false);

    Ok(())
}

#[derive(Component)]
struct Laser(Timer);

fn fire_laser(
    trigger: Trigger<Fired<FireLaser>>,
    player: Query<&Transform>,
    mut commands: Commands,
    time: Res<Time>,
    mut last_fired: Local<Duration>,
    game_assets: Res<GameAssets>,
    mut audio: EventWriter<AudioStart>,
) -> Result {
    let mut transform = *player.get(trigger.target())?;
    transform.translation += transform.local_y() * 40.0;
    transform.scale = Vec3::ONE / 2.0;

    if time.elapsed() > *last_fired + Duration::from_secs_f32(0.5) {
        commands
            .spawn((
                // Sprite::from_image(game_assets.laser.clone()),
                Sprite {
                    image: game_assets.laser.clone(),
                    color: Color::srgb(5.0, 1.0, 1.0),
                    ..default()
                },
                transform,
                RigidBody::Dynamic,
                Collider::rectangle(4.0, 15.0),
                LinearVelocity(transform.local_y().xy() * 1000.0),
                Laser(Timer::from_seconds(1.0, TimerMode::Once)),
                CollisionEventsEnabled,
                StateScoped(GameState::Game),
            ))
            .observe(laser_attack);
        *last_fired = time.elapsed();
        audio.write(AudioStart::Laser);
    }
    Ok(())
}

fn laser_range(mut commands: Commands, mut lasers: Query<(Entity, &mut Laser)>, time: Res<Time>) {
    for (entity, mut laser) in &mut lasers {
        if laser.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn laser_attack(
    collision: Trigger<OnCollisionStart>,
    is_asteroid: Query<(), With<Asteroid>>,
    mut commands: Commands,
    mut audio: EventWriter<AudioStart>,
) {
    if is_asteroid.get(collision.collider).is_ok() {
        commands.entity(collision.collider).despawn();
        commands.entity(collision.target()).despawn();
        audio.write(AudioStart::AsteroidExplosion);
    }
}

fn asteroid_collision(
    collision: Trigger<OnCollisionStart>,
    is_asteroid: Query<(), With<Asteroid>>,
    player: Query<&Transform>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut audio: EventWriter<AudioStart>,
) -> Result {
    if is_asteroid.get(collision.collider).is_ok() {
        let transform = player.get(collision.target())?;
        commands.spawn((
            Sprite::from_image(game_assets.explosion.clone()),
            (*transform).with_scale(Vec3::splat(0.2)),
            Explosion(Timer::from_seconds(1.0, TimerMode::Once)),
            StateScoped(GameState::Game),
        ));
        commands.entity(collision.target()).despawn();
        commands.entity(collision.collider).despawn();
        audio.write(AudioStart::ShipExplosion);
    }
    Ok(())
}

fn has_won(
    asteroids: Query<(), With<Asteroid>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio: EventWriter<AudioStart>,
) {
    if asteroids.is_empty() {
        next_state.set(GameState::Won);
        audio.write(AudioStart::Win);
    }
}

fn follow_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };
    camera_transform.translation = player_transform.translation;
}

fn closest(
    asteroids: Query<(Entity, &Transform), With<Asteroid>>,
    player: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos,
    mut commands: Commands,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let player_position = player_transform.translation.xy();
    let Some((entity, nearest)) = asteroids.iter().reduce(|a, b| {
        if a.1.translation.xy().distance_squared(player_position)
            < b.1.translation.xy().distance_squared(player_position)
        {
            a
        } else {
            b
        }
    }) else {
        return;
    };
    let nearest_position = nearest.translation.xy();
    let distance = nearest_position - player_position;

    let direction = distance.normalize();
    if distance.length() > 1000.0 {
        commands
            .entity(entity)
            .insert(LinearVelocity(direction.normalize() * -100.0));
    }
    gizmos.arrow_2d(
        player_position + direction * 45.0,
        player_position + direction * 70.0,
        // nearest_position,
        Color::hsl(0.0, 1.0, 0.5),
    );
}
