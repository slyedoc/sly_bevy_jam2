use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{assets::SpaceKitAssets, GameState};

pub struct AstronautAPlugin;

impl Plugin for AstronautAPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_astronaut_a.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
pub struct AstronautA;

fn spawn_astronaut_a(
    mut commands: Commands,
    query: Query<(Entity, &AstronautA), Added<AstronautA>>,
    spacekit: Res<SpaceKitAssets>,
) {
    for (e, _astronaut_a) in query.iter() {
    
            info!("spawning astronaut a");
            commands
                .entity(e)
                .insert(Name::new("AstronautA"))
                .with_children(|parent| {
                    parent.spawn_bundle(SceneBundle {
                        scene: spacekit.astronaut_a.clone(),
                        ..default()
                    });
                });
        
    }
}
