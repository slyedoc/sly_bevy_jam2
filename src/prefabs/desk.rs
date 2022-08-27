use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{assets::SpaceKitAssets, GameState};

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_desk.run_in_state(GameState::Playing));
    }
}

#[derive(Component, Debug)]
pub enum Desk {
    Chair,
    ArmChair,
    Stool,
    Computer,
    ComputerCorner,
    ComputerScreen,
}

fn spawn_desk(
    mut commands: Commands,
    query: Query<(Entity, &Desk), Added<Desk>>,
    spacekit: Res<SpaceKitAssets>,
) {
    for (e, desk) in query.iter() {
        info!("spawning desk");
        commands
            .entity(e)
            .insert(Name::new(format!("{:?}", desk)))
            .insert(match desk {
                Desk::Chair => spacekit.desk_chair.clone(),
                Desk::ArmChair => spacekit.desk_chair_arms.clone(),
                Desk::Stool => spacekit.desk_chair_stool.clone(),
                Desk::Computer => spacekit.desk_computer.clone(),
                Desk::ComputerCorner => spacekit.desk_computer_corner.clone(),
                Desk::ComputerScreen => spacekit.desk_computer_screen.clone(),
            })
            .insert(Visibility::default())
            .insert(ComputedVisibility::default());
    }
}
