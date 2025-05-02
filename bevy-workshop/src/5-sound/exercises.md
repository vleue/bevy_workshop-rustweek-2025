# Exercises

Don't forget to checkout the branch:

```sh
git checkout 09-sound-effects
```

Let's review what was changed: <https://github.com/vleue/bevy_workshop-rustweek-2025/compare/08-action-zones..09-sound-effects>

## Explosions

Add sound for asteroid and ship explosions.

Tips:

- You can use [chiptone](https://sfbgames.itch.io/chiptone) or [jsfxr](https://sfxr.me) to quickly try new sound effects

## Other Events

Add sound for game start, winning and losing.

Tips:

- You can use [chiptone](https://sfbgames.itch.io/chiptone) or [jsfxr](https://sfxr.me) to quickly try new sound effects

## Background Music

Add a background music

Tips:

- You can use [`PlaybackSettings::LOOP`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.PlaybackSettings.html#associatedconstant.LOOP) to play a looping audio

## Audio Settings

Audio volume should always be configurable. This is important for accessibility. Add a way to control volume of all audio, or even better ways to control separately the volume of the background music and of the audio effects.

Tips:

- [`PlaybackSettings`](https://docs.rs/bevy/0.16.0/bevy/audio/struct.PlaybackSettings.html) can be used to control volume of an audio
- You can add +/- buttons on the menu screen that control the volume
- Store the current volume in a resource, and use it when spawning new entities to play audio
