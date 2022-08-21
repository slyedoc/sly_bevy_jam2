use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub const CLEAR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub struct ButtonColors {
    pub normal: UiColor,
    pub hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

impl FontAssets {
    pub fn h1(&self, text: String, color: Color) -> TextSection {
        TextSection {
            value: text,
            style: TextStyle {
                font: self.fira_sans.clone(),
                font_size: 30.0,
                color,
            },
        }
    }

    pub fn title(&self, text: String, color: Color) -> TextSection {
        TextSection {
            value: text,
            style: TextStyle {
                font: self.fira_sans.clone(),
                font_size: 90.0,
                color,
            },
        }
    }

    pub fn sub_title(&self, text: String, color: Color) -> TextSection {
        TextSection {
            value: text,
            style: TextStyle {
                font: self.fira_sans.clone(),
                font_size: 16.0,
                color,
            },
        }
    }
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/intro.ogg")]
    pub intro: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}


#[derive(AssetCollection)]
pub struct NexusAudioAssets {

    #[asset(path = "audio/nexus/chris.ogg")]
    pub annoyed0: Handle<AudioSource>,

    #[asset(path = "audio/nexus/hey_stop_that.ogg")]
    pub annoyed1: Handle<AudioSource>,

    #[asset(path = "audio/nexus/whats_wrong_with_you.ogg")]
    pub annoyed2: Handle<AudioSource>,
}