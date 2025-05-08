# Displaying the Level

Loading an asset is an asynchronous process. As it involves file or network access, it doesn't happen immediately. This is why the asset server is returning an [`Handle`](https://docs.rs/bevy/0.16.0/bevy/asset/enum.Handle.html) instead of the data.

Accessing the data from the [`Assets<T>`](https://docs.rs/bevy/0.16.0/bevy/asset/struct.Assets.html) resource returns an `Option<T>` as the data may not be present yet. In our case, we're using the 2 second delay of the splash screen to be sure that assets are done loading, so we can `unwrap()` the `Option`.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Asset, TypePath)]
# struct Level {width: u32, height: u32, asteroids: u32, lives: u32}
# #[derive(Resource)]
# struct GameAssets {
# }
# #[derive(Resource)]
# pub struct LoadedLevel { pub level: Handle<Level> }
# #[derive(Component)]
# struct Asteroid;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }
fn display_level(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    loaded_level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&loaded_level.level).unwrap();

    // do something with the level
}
```
