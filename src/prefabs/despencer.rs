
use bevy::{
    prelude::*,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;
use crate::GameState;


pub struct DespencerPlugin;

impl Plugin for DespencerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_despencer.run_in_state(GameState::Playing));
    }
}

fn spawn_despencer() {

}