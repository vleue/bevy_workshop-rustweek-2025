use bevy::prelude::*;

use crate::AudioAssets;

#[derive(Event)]
pub enum AudioStart {
    Laser,
    ShipExplosion,
    AsteroidExplosion,
    Start,
    Win,
    Lose,
}

pub fn audio_plugin(app: &mut App) {
    app.add_event::<AudioStart>()
        .add_systems(Update, play_audio);
}

// fn play_audio(
//     mut commands: Commands,
//     mut audio_triggers: EventReader<AudioStart>,
//     sound_assets: Res<AudioAssets>,
// ) {
//     for trigger in audio_triggers.read() {
//         match trigger {
//             AudioStart::Laser => {
//                 commands.spawn((
//                     AudioPlayer::<AudioSource>(sound_assets.laser.clone()),
//                     PlaybackSettings::DESPAWN,
//                 ));
//             }
//             AudioStart::ShipExplosion => {
//                 commands.spawn((
//                     AudioPlayer::<AudioSource>(sound_assets.ship_explosion.clone()),
//                     PlaybackSettings::DESPAWN,
//                 ));
//             }
//             AudioStart::AsteroidExplosion => {
//                 commands.spawn((
//                     AudioPlayer::<AudioSource>(sound_assets.asteroid_explosion.clone()),
//                     PlaybackSettings::DESPAWN,
//                 ));
//             }
//         }
//     }
// }

impl AudioStart {
    fn to_handle(&self, audio_assets: &AudioAssets) -> Handle<AudioSource> {
        match self {
            AudioStart::Laser => audio_assets.laser.clone(),
            AudioStart::ShipExplosion => audio_assets.ship_explosion.clone(),
            AudioStart::AsteroidExplosion => audio_assets.asteroid_explosion.clone(),
            AudioStart::Start => audio_assets.start.clone(),
            AudioStart::Win => audio_assets.win.clone(),
            AudioStart::Lose => audio_assets.lose.clone(),
        }
    }
}

fn play_audio(
    mut commands: Commands,
    mut audio_triggers: EventReader<AudioStart>,
    sound_assets: Res<AudioAssets>,
) {
    for trigger in audio_triggers.read() {
        commands.spawn((
            AudioPlayer::<AudioSource>(trigger.to_handle(sound_assets.as_ref())),
            PlaybackSettings::DESPAWN,
        ));
    }
}
