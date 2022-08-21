mod nexus;

use bevy::prelude::*;
pub use nexus::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NexusPlugin);   
    }
}
