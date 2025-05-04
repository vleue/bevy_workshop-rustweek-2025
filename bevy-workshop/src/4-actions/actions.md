# Action Mapper

Goal of the game is to destroy all the asteroids.

Let's give the player a way to do that! But first let's refactor how we handle the player's actions so that it's easier to extend.

## Using an Input Manager

TODO

- refactor movements using https://crates.io/crates/bevy_enhanced_input

## Triggers

They can be global with [`Commands::trigger`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.Commands.html#method.trigger) and [`App::add_observer`](https://docs.rs/bevy/0.16.0/bevy/app/struct.App.html#method.add_observer), or specific to an entity with [`EntityCommands::trigger`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.EntityCommands.html#method.trigger) and [`EntityCommands::observe`](https://docs.rs/bevy/0.16.0/bevy/ecs/prelude/struct.EntityCommands.html#method.observe).

TODO

## Observers

To react to the trigger, we use a system that takes a [`Trigger`](https://docs.rs/bevy/0.16.0/bevy/ecs/observer/struct.Trigger.html) as a system parameter, plus any other parameter needed.

TODO
