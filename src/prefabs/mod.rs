mod nexus;
mod player;
mod rooms;
mod switch;
mod door;
mod alien;
mod astronaut_a;
mod desk;

pub use nexus::*;
pub use player::*;
pub use switch::*;
pub use rooms::*;
pub use door::*;
pub use alien::*;
pub use astronaut_a::*;
pub use desk::*;

use bevy::prelude::*;

use self::alien::AlienPlugin;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NexusPlugin)
            //.add_plugin(PlayerPlugin)
            .add_plugin(SwitchPlugin)
            .add_plugin(DoorPlugin)
            .add_plugin(AlienPlugin)
            .add_plugin(AstronautAPlugin)
            .add_plugin(DeskPlugin)
            .add_plugin(RoomPlugin);
    }
}
