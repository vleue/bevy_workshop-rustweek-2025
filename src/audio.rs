use bevy::prelude::*;

use crate::AudioAssets;

#[derive(Event)]
pub enum AudioStart {
    Laser,
}

pub fn audio_plugin(app: &mut App) {
    app.add_event::<AudioStart>()
        .add_systems(Update, play_audio);
}

fn play_audio(
    mut commands: Commands,
    mut audio_triggers: EventReader<AudioStart>,
    sound_assets: Res<AudioAssets>,
) {
    for trigger in audio_triggers.read() {
        match trigger {
            AudioStart::Laser => {
                commands.spawn((
                    AudioPlayer::<AudioSource>(sound_assets.laser.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}
