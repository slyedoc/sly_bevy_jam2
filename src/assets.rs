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
pub struct SpaceKitAssets {
    #[asset(path = "spacekit/alien.glb#Scene0")]
    pub alien: Handle<Scene>,

    #[asset(path = "spacekit/astronautA.glb#Scene0")]
    pub astronaut_a: Handle<Scene>,

    #[asset(path = "spacekit/astronautB.glb#Scene0")]
    pub astronaut_b: Handle<Scene>,

    #[asset(path = "spacekit/barrel.glb#Scene0")]
    pub barrel: Handle<Scene>,

    #[asset(path = "spacekit/barrels.glb#Scene0")]
    pub barrels: Handle<Scene>,


    #[asset(path = "spacekit/barrels_rail.glb#Scene0")]
    pub barrels_rail: Handle<Scene>,

    #[asset(path = "spacekit/bones.glb#Scene0")]
    pub bones: Handle<Scene>,


    #[asset(path = "spacekit/desk_chairArms.glb#Scene0")]
    pub desk_chair_arms: Handle<Scene>,
    #[asset(path = "spacekit/desk_chairStool.glb#Scene0")]
    pub desk_chair_stool: Handle<Scene>,
    #[asset(path = "spacekit/desk_chair.glb#Scene0")]
    pub desk_chair: Handle<Scene>,

    #[asset(path = "spacekit/desk_computerCorner.glb#Scene0")]
    pub desk_computer_corner: Handle<Scene>,

    #[asset(path = "spacekit/desk_computer.glb#Scene0")]
    pub desk_computer: Handle<Scene>,

    #[asset(path = "spacekit/desk_computerScreen.glb#Scene0")]
    pub desk_computer_screen: Handle<Scene>,
}
