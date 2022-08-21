use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::assets::AudioAssets;

pub struct MenuBackgroundAudio;

// TODO: look at using instance_control
// https://github.com/NiklasEi/bevy_kira_audio/blob/main/examples/instance_control.rs
pub fn start_audio(
    audio_assets: Res<AudioAssets>,
    background: Res<AudioChannel<MenuBackgroundAudio>>,
    mut started: Local<bool>,
) {
    info!("Starting background audio");

    if !*started {
        background.play(audio_assets.intro.clone())
            .looped()
            .with_volume(0.3)
            .fade_in(AudioTween::new(
                Duration::from_secs_f32(1.0),
                AudioEasing::Linear,
            ));

        *started = true;
    } else {
        background.resume()
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(1.0),
            AudioEasing::Linear,
        ));
    }


}

pub fn stop_audio(background: Res<AudioChannel<MenuBackgroundAudio>>) {
    info!("stopping audio");
    background.pause().fade_out(AudioTween::new(
        Duration::from_secs_f32(1.0),
        AudioEasing::Linear,
    ));
}
