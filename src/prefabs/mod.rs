mod door;
mod nexus;
mod player;
mod rooms;
mod switch;
mod space_kit;

pub use door::*;
pub use nexus::*;
pub use player::*;
pub use rooms::*;
pub use switch::*;
pub use space_kit::*;

use bevy::prelude::*;


pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NexusPlugin)
            //.add_plugin(PlayerPlugin)
            .add_plugin(SwitchPlugin)
            .add_plugin(DoorPlugin)
            .add_plugin(SpaceKitPlugin)
            .add_plugin(RoomPlugin);
    }
}
