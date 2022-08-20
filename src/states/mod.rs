mod loading;
mod menu;
mod playing;
mod pre_loading;

use bevy::prelude::*;
pub use loading::*;
pub use menu::*;
pub use playing::*;
pub use pre_loading::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PreLoadingPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(PlayingPlugin);
    }
}
