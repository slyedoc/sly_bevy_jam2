mod ai;
mod dispenser;
mod door;
mod pellet;
mod polarity_blaster;
mod reactor;
mod rooms;
mod space_kit;
mod switch;

pub use self::reactor::*;
pub use ai::*;
pub use dispenser::*;
pub use door::*;
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
            .add_plugin(DispenserPlugin)
            .add_plugin(ReactorPlugin);
    }
}
