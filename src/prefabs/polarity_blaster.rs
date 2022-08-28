use bevy::{    
    prelude::*, gltf::{Gltf, GltfMesh}, math::vec3,
};
use bevy_mod_outline::{OutlineMeshExt, OutlineBundle, Outline};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{GameState, assets::SpaceKitAssets, cursor::*};


#[derive(Component)]
pub struct PolarityBlaster;

pub struct PolarityBlasterPlugin;

impl Plugin for PolarityBlasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_blaster.run_in_state(GameState::Playing));
    }
}

pub fn spawn_blaster(
    mut commands: Commands,
    query: Query<(Entity, &mut PolarityBlaster), Added<PolarityBlaster>>,    
    spacekit: Res<SpaceKitAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_mesh: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    cursor_config: Res<CursorConfig>,
    mut collider_resources: ResMut<ColliderResources>,
) {
    for (e, mut blaster) in query.iter() {
        if let Some(gltf) = assets_gltf.get(&spacekit.weapon_blaster_r_gltf) {
            if let Some(gltf_mesh) = assets_gltf_mesh.get(&gltf.meshes[0]) {
                let mesh_handle = gltf_mesh.primitives[0].mesh.clone();

                let mesh = meshes.get_mut(&mesh_handle).unwrap();                
                mesh.generate_outline_normals().unwrap();
         
                commands.entity(e)
                    .insert(mesh_handle)
                    .insert(gltf.materials[0].clone())
                    .insert(Visibility::default())
                    .insert(ComputedVisibility::default())
                    .insert_bundle(OutlineBundle {
                        outline: Outline {
                            visible: false,
                            colour: cursor_config.hover,
                            width: cursor_config.width,
                        },
                        ..default()
                    })
                    .insert(CursorInteraction::None)
                    .insert(InteractionTime::default())
                    .insert_bundle(RigidBodyBundle {
                        collider: collider_resources.add_box(vec3(0.2, 0.5, 0.1)),
                        ..default()
                    })
                    .insert(Name::new("Blaster"));
                
            }
            
        }
    }
}