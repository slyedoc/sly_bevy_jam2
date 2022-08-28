use bevy::{
    gltf::{Gltf, GltfMesh},
    math::vec3,
    prelude::*,
};
use bevy_mod_outline::{Outline, OutlineBundle, OutlineMeshExt};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{
    assets::{SpaceKitAssets, CLEAR},
    camera::CameraMain,
    cursor::*,
    lines::*,
    GameState,
};

use super::Pellet;

#[derive(Component)]
pub enum PolarityBlaster {
    Disabled,
    Enabled,
}

pub struct PolarityBlasterPlugin;

impl Plugin for PolarityBlasterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PolarityBlasterConfig>()
            .add_system(spawn_blaster.run_in_state(GameState::Playing))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                interaction_check.run_in_state(GameState::Playing),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate, fire_blaster.run_in_state(GameState::Playing));
    }
}

pub struct PolarityBlasterConfig {
    pub offset: Vec3,
    collider: Vec3,
    hit_change: f32
}

impl Default for PolarityBlasterConfig {
    fn default() -> Self {
        Self {
            offset: vec3(0.3, -0.3, -0.7),
            collider: vec3(0.3, 0.4, 0.9),
            hit_change: 0.01,
        }
    }
}

#[derive(Component)]
pub struct Laser;

fn fire_blaster(
    mut commands: Commands,
    query: Query<(Entity, &PolarityBlaster), With<Parent>>,
    camera_query: Query<&Transform, With<CameraMain>>,
    mouse_input: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut laser_query: Query<(Entity, &Handle<LineMaterial>), With<Laser>>,
    tlas: Res<Tlas>,
    mut pellet_query: Query<&mut Pellet>,
    config: Res<PolarityBlasterConfig>,
) {
    let camera_trans = camera_query.single();
    
    for (_entity, _blaster) in query.iter() {
        let mut fire = false;
        let mut color = Color::NONE;
        let mut change = 0.0;
        if mouse_input.pressed(MouseButton::Left) {
            fire = true;
            color = Color::BLUE;
            change = config.hit_change;
        } else if mouse_input.pressed(MouseButton::Right) {
            fire = true;
            color = Color::YELLOW;
            change = -config.hit_change;
        }

        let (laser_entity, laser_material_handle) = laser_query.single_mut();
        let  laser_material = materials.get_mut(&laser_material_handle).unwrap();
        if fire {
            // create laser
            laser_material.color = color;
            commands.entity(laser_entity).insert(meshes.add(Mesh::from(LineStrip {
                points: vec![
                    Vec3::ZERO,
                    -Vec3::Z * 20.0,
                ],
            })));

            // update pellet if hit
            let mut ray = Ray::new(camera_trans.translation, camera_trans.forward());
            if let Some(hit) = ray.intersect_tlas(&tlas) {
                if let Ok(mut pellet) = pellet_query.get_mut(hit.entity) {
                    pellet.value = (pellet.value + change).clamp(0.0, 1.0);
                }
            }
        } else {
            // clear laser
            commands.entity(laser_entity).insert(meshes.add(Mesh::from(LineStrip {
                points: vec![
                ],
            })));
        }

    }
}


pub fn spawn_blaster(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PolarityBlaster), Added<PolarityBlaster>>,
    spacekit: Res<SpaceKitAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_mesh: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    cursor_config: Res<CursorConfig>,
    mut collider_resources: ResMut<ColliderResources>,
    config: Res<PolarityBlasterConfig>,

) {
    for (e, mut _blaster) in query.iter_mut() {
        if let Some(gltf) = assets_gltf.get(&spacekit.weapon_blaster_r_gltf) {
            if let Some(gltf_mesh) = assets_gltf_mesh.get(&gltf.meshes[0]) {
                commands
                    .entity(e)
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
                        collider: collider_resources.add_box(config.collider),
                        mode: RigidBodyMode::Static,
                        ..default()
                    })
                    .insert(Name::new("Blaster"))
                    .with_children(|parent| {

                        //add laser
                        parent
                        .spawn_bundle(MaterialMeshBundle {
                            mesh: meshes.add(Mesh::from(LineStrip {
                                points: vec![
                                ],
                            })),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            material: materials.add(LineMaterial { color: CLEAR }),
                            ..default()
                        })
                        .insert(Laser);

                        for prim in &gltf_mesh.primitives {
                            let mesh_handle = prim.mesh.clone();

                            let mesh = meshes.get_mut(&mesh_handle).unwrap();
                            mesh.generate_outline_normals().unwrap();

                            parent
                                .spawn_bundle(PbrBundle {
                                    mesh: mesh_handle.clone(),
                                    material: prim.material.as_ref().unwrap().clone(),
                                    ..default()
                                })
                                .insert_bundle(OutlineBundle {
                                    outline: Outline {
                                        visible: false,
                                        colour: cursor_config.hover,
                                        width: cursor_config.width,
                                    },
                                    ..default()
                                });
                        }
                    });
            }
        }
    }
}

fn interaction_check(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &PolarityBlaster,
        &mut Transform,
        &Children,
        &CursorInteraction,
        &mut InteractionTime,
    )>,
    mut outline_query: Query<&mut Outline>,
    cursor_config: Res<CursorConfig>,
    camera_query: Query<Entity, With<CameraMain>>,
    config: Res<PolarityBlasterConfig>,
) {
    for (e, blaster, mut trans, children, cursor_interaction, mut _interaction_time) in
        query.iter_mut()
    {
        match cursor_interaction {
            CursorInteraction::Hovered => {
                for c in children.iter() {
                    if let Ok(mut outline) = outline_query.get_mut(*c) {
                        outline.visible = true;
                        outline.colour = match blaster {
                            PolarityBlaster::Disabled => cursor_config.disabled,
                            PolarityBlaster::Enabled => cursor_config.hover,
                        };
                    }
                }
            }
            CursorInteraction::Clicked => {
                match blaster {
                    PolarityBlaster::Disabled => {}
                    PolarityBlaster::Enabled => {
                        // pick up
                        let camera_entity = camera_query.single();
                        commands.entity(camera_entity).push_children(&[e]);

                        trans.translation = config.offset;
                        trans.rotation = Quat::IDENTITY;
                    }
                }
            }
            CursorInteraction::None => {
                for c in children.iter() {
                    if let Ok(mut outline) = outline_query.get_mut(*c) {
                        outline.visible = false;
                    }
                }
            }
        }
    }
}
