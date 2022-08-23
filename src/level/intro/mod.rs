use crate::LevelState;
use crate::prefabs::*;
use crate::{cleanup};

use bevy::prelude::*;
use iyes_loopless::prelude::*;


pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        info!("Building IntroPlugin");
        app.add_enter_system(LevelState::Intro, spawn_room)
            .add_enter_system(LevelState::Intro, spawn_nexus)                        
            .add_enter_system(LevelState::Intro, spawn_player)                        
            .add_enter_system(LevelState::Intro, test)                        
            .add_exit_system(LevelState::Intro, cleanup);
    }
}

fn test() {
    info!("test");
}