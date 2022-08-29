use bevy::{gltf::Gltf, prelude::*};
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
    #[asset(path = "textures/crossair_black.png")]
    pub crossair_black: Handle<Image>,

    #[asset(path = "textures/crossair_red.png")]
    pub crossair_red: Handle<Image>,

    #[asset(path = "textures/crossair_white.png")]
    pub crossair_white: Handle<Image>,

    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "textures/pattern_74.png")]
    pub pattern_74: Handle<Image>,

    #[asset(path = "textures/pattern_78.png")]
    pub pattern_78: Handle<Image>,
   
}

#[derive(AssetCollection)]
pub struct AIAudioAssets {
    #[asset(path = "audio/ai/intro_1.ogg")]
    pub intro_1: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_2.ogg")]
    pub intro_2: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_3.ogg")]
    pub intro_3: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_4.ogg")]
    pub intro_4: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_5.ogg")]
    pub intro_5: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_6.ogg")]
    pub intro_6: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_7.ogg")]
    pub intro_7: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_8.ogg")]
    pub intro_8: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_9.ogg")]
    pub intro_9: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_10.ogg")]
    pub intro_10: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_check_2.ogg")]
    pub intro_check_2: Handle<AudioSource>,

    #[asset(path = "audio/ai/intro_check_3.ogg")]
    pub intro_check_3: Handle<AudioSource>,

    #[asset(path = "audio/ai/chris.ogg")]
    pub annoyed0: Handle<AudioSource>,

    #[asset(path = "audio/ai/hey_stop_that.ogg")]
    pub annoyed1: Handle<AudioSource>,

    #[asset(path = "audio/ai/whats_wrong_with_you.ogg")]
    pub annoyed2: Handle<AudioSource>,

    #[asset(path = "audio/ai/high_1.ogg")]
    pub high_1: Handle<AudioSource>,

    #[asset(path = "audio/ai/high_2.ogg")]
    pub high_2: Handle<AudioSource>,

    #[asset(path = "audio/ai/high_3.ogg")]
    pub high_3: Handle<AudioSource>,

    #[asset(path = "audio/ai/start.ogg")]
    pub start: Handle<AudioSource>,
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

    #[asset(path = "spacekit/machine_barrel.glb#Scene0")]
    pub machine_barrel: Handle<Scene>,

    #[asset(path = "spacekit/machine_barrelLarge.glb#Scene0")]
    pub machine_barrel_large: Handle<Scene>,

    #[asset(path = "spacekit/machine_generator.glb#Scene0")]
    pub machine_generator: Handle<Scene>,

    #[asset(path = "spacekit/machine_generatorLarge.glb#Scene0")]
    pub machine_generator_large: Handle<Scene>,

    #[asset(path = "spacekit/machine_wireless.glb#Scene0")]
    pub machine_wireless: Handle<Scene>,

    #[asset(path = "spacekit/machine_wirelessCable.glb#Scene0")]
    pub machine_wireless_cable: Handle<Scene>,

    #[asset(path = "spacekit/weapon_gun.glb#Scene0")]
    pub weapon_gun: Handle<Scene>,

    #[asset(path = "spacekit/weapon_rifle.glb#Scene0")]
    pub weapon_rifle: Handle<Scene>,

    #[asset(path = "spacekit/weapon_blasterR.glb#Scene0")]
    pub weapon_blaster_r: Handle<Scene>,

    #[asset(path = "spacekit/weapon_blasterR.glb")]
    pub weapon_blaster_r_gltf: Handle<Gltf>,

    #[asset(path = "spacekit/rocket_baseA.glb#Scene0")]
    pub rocket_base_a: Handle<Scene>,

    #[asset(path = "spacekit/rocket_baseB.glb#Scene0")]
    pub rocket_base_b: Handle<Scene>,

    #[asset(path = "spacekit/rocket_finsA.glb#Scene0")]
    pub rocket_fins_a: Handle<Scene>,

    #[asset(path = "spacekit/rocket_finsB.glb#Scene0")]
    pub rocket_fins_b: Handle<Scene>,

    #[asset(path = "spacekit/rocket_fuelA.glb#Scene0")]
    pub rocket_fuel_a: Handle<Scene>,

    #[asset(path = "spacekit/rocket_fuelB.glb#Scene0")]
    pub rocket_fuel_b: Handle<Scene>,

    #[asset(path = "spacekit/rocket_sidesA.glb#Scene0")]
    pub rocket_sides_a: Handle<Scene>,

    #[asset(path = "spacekit/rocket_sidesB.glb#Scene0")]
    pub rocket_sides_b: Handle<Scene>,

    #[asset(path = "spacekit/rocket_topA.glb#Scene0")]
    pub rocket_top_a: Handle<Scene>,

    #[asset(path = "spacekit/rocket_topB.glb#Scene0")]
    pub rocket_top_b: Handle<Scene>,
}
