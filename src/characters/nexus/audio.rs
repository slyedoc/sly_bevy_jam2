use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::assets::NexusAudioAssets;

pub struct NexuxAudioChannel;

pub struct NexusAnnoyEvent;

pub struct NexusAnnoyConfig {    
    pub list: Vec<Handle<AudioSource>>,
    pub last: Option<usize>,
}

impl NexusAnnoyConfig {
    // TODO: make this random
    pub fn next(&mut self) -> Handle<AudioSource> {
        let last = self.last.unwrap_or(0);
        let next = (last + 1) % self.list.len();
        self.last = Some(next);
        self.list[next].clone()
    }
}

pub fn setup_annoy_config(
    mut commands: Commands,
    assets: Res<NexusAudioAssets>,
) {
    commands.insert_resource(NexusAnnoyConfig {
        list: vec![
            assets.annoyed0.clone(),
            assets.annoyed1.clone(),
            assets.annoyed2.clone(),
        ],
        last: None,
    });
}

pub fn stop_audio(channel: Res<AudioChannel<NexuxAudioChannel>>) {
    channel.pause().fade_out(AudioTween::new(
        Duration::from_secs_f32(1.0),
        AudioEasing::Linear,
    ));
}
