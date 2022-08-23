mod intro;

use bevy::prelude::*;

use intro::IntroPlugin;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(IntroPlugin);
    }
}
