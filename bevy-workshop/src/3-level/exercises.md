# Exercises

Don't forget to checkout the branch:

```sh
git checkout 07-level-loading
```

Let's review what was changed: <https://github.com/vleue/bevy_workshop-rustweek-2025/compare/06-basic-game..07-level-loading>

## Spawn the correct number of asteroids

TODO

Tips:

- Avoid spawning an asteroid on top of the player

## Display informations about the level

TODO

Tips:

- Number of asteroids remaining
- Lives remaining
- Time spent in the level

## Player can have multiple lives

TODO

Tips:

- Add the number of lives to a component
- On collision, decrement the number of lives
- After a collision, wait for a few seconds before respawning the player
- If the number of lives is 0, game over

## Try Hot Reloading

Tips:

- It needs to enable a feature on Bevy: `file_watcher`
- Check if the asset changed, then despawn the level and respawn it from the updated file
