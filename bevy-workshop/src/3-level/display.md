# Displaying the Level

TODO

Loading an asset is an asynchronous process. As it involves file or network access, it doesn't happen immediately. This is why the asset server is returning an [`Handle`](https://docs.rs/bevy/0.16.0/bevy/asset/enum.Handle.html) instead of the data.

Accessing the data from the [`Assets<T>`](https://docs.rs/bevy/0.16.0/bevy/asset/struct.Assets.html) resource returns an `Option<T>` as the data may not be present yet. In our case, we're using the 2 second delay of the splash screen to be sure that assets are done loading, so we can `unwrap()` the `Option`.

```rust
# extern crate bevy;
# use bevy::prelude::*;
# #[derive(Asset, TypePath)]
# struct Level {pub tiles: Vec<Vec<Tile>>}
# enum Tile {Empty, Ground}
# #[derive(Resource)]
# struct GameAssets {
#     player_image: Handle<Image>,
#     player_layout: Handle<TextureAtlasLayout>,
#     ground_image: Handle<Image>,
#     ground_layout: Handle<TextureAtlasLayout>,
# }
# #[derive(Resource)]
# pub struct LoadedLevel { pub level: Handle<Level> }
# #[derive(Component)]
# struct Player;
# #[derive(Component)]
# struct Ground;
# #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
# enum GameState { #[default] Game }

```
