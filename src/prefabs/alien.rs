use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{assets::SpaceKitAssets, GameState};

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_alien.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
pub struct Alien;

fn spawn_alien(
    mut commands: Commands,
    query: Query<(Entity, &Alien), Added<Alien>>,
    spacekit: Res<SpaceKitAssets>,
    
) {
    for (e, _alien) in query.iter() {
    
            info!("spawning alien");
            commands
                .entity(e)
                .insert(Name::new("Alien"))
                .with_children(|parent| {
                    parent.spawn_bundle(SceneBundle {
                        scene: spacekit.alien.clone(),
                        ..default()
                    });
                });
        
    }
}
