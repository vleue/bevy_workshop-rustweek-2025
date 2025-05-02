# Exercises

Don't forget to checkout the branch:

```sh
git checkout 06-basic-game
```

Let's review what was changed: <https://github.com/vleue/bevy_workshop-rustweek-2025/compare/05-intro-to-bevy..06-basic-game>

## Displaying an Asteroid

File `meteorBrown_big1.png` has a sprite for an asteroid. Use it instead of the blue box.

Tips:

- Add a new field to the `GameAssets` resource for the asteroid

## Player Sprite Animation

Display engine jets behind the ship when moving forward.

Tips:

- Spawn the jets as children sprite of the player entity with the `children!` macro
- When the player moves forward, make the sprite visible and start or refresh a timer
- When the timer expires, make the sprite invisible

## Player Acceleration

In space, there's no friction. Pressing `W` should make the ship accelerate in a direction, and movements should continue after the key is released.

Tips:

- Store the player velocity in a new component
- When the `W` key is pressed, change the velocity
- In a separate system, move the player according to its current velocity
