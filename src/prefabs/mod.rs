mod door;
mod nexus;
mod player;
mod polarity_blaster;
mod rooms;
mod space_kit;
mod switch;

pub use door::*;
pub use nexus::*;
pub use player::*;
pub use polarity_blaster::*;
pub use rooms::*;
pub use space_kit::*;
pub use switch::*;

use bevy::prelude::*;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NexusPlugin)
            //.add_plugin(PlayerPlugin)
            .add_plugin(SwitchPlugin)
            .add_plugin(DoorPlugin)
            .add_plugin(SpaceKitPlugin)
            .add_plugin(RoomPlugin)
            .add_plugin(PolarityBlasterPlugin);
    }
}
