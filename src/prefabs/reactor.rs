
use bevy::{
    math::vec3,
    prelude::*,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;
use rand::prelude::*;
use crate::GameState;


pub struct ReactorPlugin;

impl Plugin for ReactorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_reactor.run_in_state(GameState::Playing));
    }
}

fn spawn_reactor() {

}