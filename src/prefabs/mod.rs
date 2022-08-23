mod nexus;
mod rooms;
mod player;

pub use nexus::*;
pub use rooms::*;
pub use player::*;

use bevy::prelude::*;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NexusPlugin);   
    }
}
