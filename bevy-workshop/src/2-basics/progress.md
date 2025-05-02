# Progress Report

## What You've learned

- Loading sprites and displaying them
  - with the [`AssetServer::load`](https://docs.rs/bevy/0.16.0/bevy/asset/struct.AssetServer.html#method.load) function
  - by adding the [`Sprite`](https://docs.rs/bevy/0.16.0/bevy/prelude/struct.Sprite.html) component
- Defining components
  - With [required components](https://docs.rs/bevy/0.16.0/bevy/ecs/component/trait.Component.html#required-components) to simplify adding related components
  - And using Zero Sized Types as tag components to filter entities in queries
- Handling user input
  - reading the [`ButtonInput<T>`](https://docs.rs/bevy/0.16.0/bevy/input/struct.ButtonInput.html) resource
  - for the input [`KeyCode`](https://docs.rs/bevy/0.16.0/bevy/input/keyboard/enum.KeyCode.html)
- Writing more complex queries, and updating components
  - the [`With`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.With.html) and [`Without`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Without.html) query filters
  - and using [`&mut`](https://docs.rs/bevy/0.16.0/bevy/ecs/change_detection/struct.Mut.html) to query data mutably
- Error handling in systems
  - using [`Result`](https://docs.rs/bevy/0.16.0/bevy/ecs/error/type.Result.html) to handle errors in systems
  - setting the [global error handler](https://docs.rs/bevy/0.16.0/bevy/ecs/error/static.GLOBAL_ERROR_HANDLER.html) to something else than panic
- Third Party Plugins
  - explore [community assets](https://bevyengine.org/assets/#assets)
  - integrate a third party plugin
