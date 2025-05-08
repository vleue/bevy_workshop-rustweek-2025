# Exercises

## Adding a Start Menu

We'll add a new plugin to handle the start menu. It will be very similar to the splash screen plugin, with different text and with a different condition to change state.

Tips:

- Create a new file for the new plugin, you can copy `splash.rs` as a starting point
- Change the state conditions and state scopes to `GameState::StartMenu`
- Modify the text to display a start menu instead of a splash screen
- Create a new variant of `GameState` for the game
- Modify the condition to change state to check for a key press instead of a timer

  - The system parameter for key press is [`Res<ButtonInput<KeyCode>>`](https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html)
  - Checking that a key was just pressed can be done with `input.just_pressed(KeyCode::Space)`

- Add the new plugin to the application
