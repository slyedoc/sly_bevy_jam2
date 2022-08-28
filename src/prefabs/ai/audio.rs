use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::assets::AIAudioAssets;

pub struct AIAudioChannel;

pub struct AIAnnoyEvent;

pub struct AIAnnoyConfig {
    pub list: Vec<Handle<AudioSource>>,
    pub last: Option<usize>,
}

impl AIAnnoyConfig {
    // TODO: make this random
    pub fn next(&mut self) -> Handle<AudioSource> {
        let last = self.last.unwrap_or(0);
        let next = (last + 1) % self.list.len();
        self.last = Some(next);
        self.list[next].clone()
    }
}

pub fn setup_annoy_config(mut commands: Commands, assets: Res<AIAudioAssets>) {
    commands.insert_resource(AIAnnoyConfig {
        list: vec![
            assets.annoyed0.clone(),
            assets.annoyed1.clone(),
            assets.annoyed2.clone(),
        ],
        last: None,
    });
}

pub fn stop_audio(channel: Res<AudioChannel<AIAudioChannel>>) {
    channel.pause().fade_out(AudioTween::new(
        Duration::from_secs_f32(1.0),
        AudioEasing::Linear,
    ));
}



pub struct AIIntroConfig {
    pub list: Vec<Handle<AudioSource>>,
    pub step: usize,
}

impl AIIntroConfig {
    // TODO: make this random
    pub fn next(&mut self) -> Option<Handle<AudioSource>> {        
        if self.list.len() - 1 < self.step {
            return None;
        }

        let result = Some(self.list[self.step].clone());
        self.step += 1;
        result
    }
}

pub fn setup_intro_config(mut commands: Commands, assets: Res<AIAudioAssets>) {
    commands.insert_resource(AIIntroConfig {
        list: vec![
            assets.intro_1.clone(),
            assets.intro_2.clone(),
            assets.intro_check_2.clone(),
            assets.intro_3.clone(),
            assets.intro_check_3.clone(),
            assets.intro_4.clone(),
            assets.intro_5.clone(),
            assets.intro_6.clone(),
            assets.intro_7.clone(),
            assets.intro_8.clone(),
            assets.intro_9.clone(),
            assets.intro_10.clone(),
        ],
        step: 0,
    });
}