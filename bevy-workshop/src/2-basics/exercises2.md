# Exercises

It's very easy to avoid collisions with the asteroids as they don't move... Let's make this game a bit harder!

## Moving the Asteroids

Make the asteroids move in random directions, at random speeds.

Tips:

- Add information about direction and speed to the `Asteroid` component.
- Add a system to update the position of the asteroids based on their direction and speed.

## Losing the Game

Let's go back to the menu when colliding with an asteroid.

Tips:

- Use the `ResMut<NextState<GameStates>>` system parameter to change the current state on collision

## Explosion Effect on Collision

TODO

- display an explosion for one second
- after the delay, despawn it and change state
