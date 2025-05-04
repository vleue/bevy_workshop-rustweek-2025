# Controlling With Input

We'll control our player with the `A` and `D` keys on the keyboard to turn, and `W` for thrust.

```rust,no_run
# extern crate bevy;
# use bevy::prelude::*;
# use std::f32::consts::FRAC_PI_8;
# #[derive(Component)]
# struct Player;
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
```

<div class="warning">

Don't forget to add the new `control_player` system to the `game_plugin`, on `FixedUpdate` in the `GameState::Game` state.

</div>

## Keyboard controls

TODO

## Modifying transforms

TODO

## Error handling in systems

TODO
new in 0.16 :tada:
https://bevyengine.org/news/bevy-0-16/#unified-ecs-error-handling
