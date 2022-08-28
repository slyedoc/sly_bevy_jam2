mod door;
mod ai;
mod polarity_blaster;
mod rooms;
mod space_kit;
mod switch;
mod pellet;
mod despencer;
mod reactor;

pub use reactor::*;
pub use despencer::*;
pub use door::*;
pub use ai::*;
pub use pellet::*;
pub use polarity_blaster::*;
pub use rooms::*;
pub use space_kit::*;
pub use switch::*;

use bevy::prelude::*;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AIPlugin)
            .add_plugin(SwitchPlugin)
            .add_plugin(DoorPlugin)
            .add_plugin(SpaceKitPlugin)
            .add_plugin(RoomPlugin)
            .add_plugin(PolarityBlasterPlugin)
            .add_plugin(PelletPlugin)
            .add_plugin(DespencerPlugin)
            .add_plugin(ReactorPlugin);
    }
}
