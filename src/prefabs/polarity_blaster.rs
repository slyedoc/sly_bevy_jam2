use std::f32::consts::FRAC_PI_2;

use bevy::{
    gltf::{Gltf, GltfMesh},
    math::vec3,
    prelude::*,
};
use bevy_mod_outline::{Outline, OutlineBundle, OutlineMeshExt};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{
    assets::SpaceKitAssets,
    camera::{CameraMain, CameraState},
    cursor::*,
    GameState, states::Score,
};

use super::{Pellet, PelletConfig};

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
                CoreStage::PostUpdate,
                fire_blaster
                    .run_in_state(GameState::Playing)
                    .run_in_state(CameraState::Player),
            );
    }
}

pub struct PolarityBlasterConfig {
    pub offset: Vec3,
    collider: Vec3,
    hit_change: f32,
    laser_offset: Vec3,
    laser_length: f32,
    laser_mesh: Handle<Mesh>,
    laser_yellow_mat: Handle<StandardMaterial>,
    laser_blue_mat: Handle<StandardMaterial>,
}

impl FromWorld for PolarityBlasterConfig {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        let laser_length = 30.0;
        let laser_offset = vec3(0.0, 0.1, -laser_length * 0.5);

        let laser_mesh = meshes.add(Mesh::from(shape::Capsule {
            radius: 0.005,
            depth: laser_length,
            ..default()
        }));

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let laser_blue_mat = materials.add(StandardMaterial {
            base_color: Color::BLUE,
            unlit: true,
            ..default()
        });
        let laser_yellow_mat = materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            unlit: true,
            ..default()
        });

        Self {
            offset: vec3(0.3, -0.3, -0.7),
            collider: vec3(0.3, 0.4, 0.9),
            hit_change: 0.01,
            laser_length,
            laser_offset,
            laser_mesh,
            laser_yellow_mat,
            laser_blue_mat,
        }
    }
}

#[derive(Component)]
pub struct Laser;


#[derive(PartialEq, Eq)]
enum HitType {
    None,
    Blue,
    Yellow,
}

pub fn fire_blaster(
    mut commands: Commands,
    query: Query<(Entity, &PolarityBlaster), With<Parent>>,
    camera_query: Query<&Transform, With<CameraMain>>,
    mouse_input: Res<Input<MouseButton>>,
    mut laser_query: Query<Entity, With<Laser>>,
    tlas: Res<Tlas>,
    mut pellet_query: Query<&mut Pellet>,
    pellet_config: Res<PelletConfig>,
    config: Res<PolarityBlasterConfig>,
    mut score: ResMut<Score>,
) {
    let camera_trans = camera_query.single();

    for (_entity, _blaster) in query.iter() {
        let mut hit_type = HitType::None;
        if mouse_input.pressed(MouseButton::Left) && !mouse_input.pressed(MouseButton::Right) {
            hit_type = HitType::Blue;
            
        } else if mouse_input.pressed(MouseButton::Right) && !mouse_input.pressed(MouseButton::Left)
        {
            hit_type = HitType::Yellow;
        }

        let laser_entity = laser_query.single_mut();
        if hit_type != HitType::None {
            // create laser
            commands
                .entity(laser_entity)
                .insert(match hit_type {
                    HitType::Blue => config.laser_blue_mat.clone(),
                    HitType::Yellow => config.laser_yellow_mat.clone(),
                    HitType::None => unreachable!(),
                })
                .insert(config.laser_mesh.clone());

            // // update pellet if hit
            let mut ray = Ray::new(camera_trans.translation, camera_trans.forward());
            if let Some(hit) = ray.intersect_tlas(&tlas) {
                if let Ok(mut pellet) = pellet_query.get_mut(hit.entity) {
                    
                    let was_in_range = (pellet.value - 0.5).abs() < pellet_config.allow_range;
                    match hit_type {

                        HitType::Blue => {
                            pellet.value = (pellet.value - config.hit_change).clamp(0.0, 1.0);                        
                        },
                        HitType::Yellow => {
                            pellet.value = (pellet.value + config.hit_change).clamp(0.0, 1.0);
                        },
                        HitType::None => unreachable!(),
                    }
                    let in_range = (pellet.value - 0.5).abs() < pellet_config.allow_range;
                    if !was_in_range && in_range {
                        score.0 += 1;
                    }
                    if was_in_range && !in_range {
                        score.0 -= 1;
                    }
        
                    pellet.hit = true;
                }
            }
        } else {
            // clear laser
            commands.entity(laser_entity).remove::<Handle<Mesh>>();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_blaster(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PolarityBlaster), Added<PolarityBlaster>>,
    spacekit: Res<SpaceKitAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_mesh: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
                            .spawn_bundle(PbrBundle {
                                transform: Transform {
                                    translation: config.laser_offset,
                                    rotation: Quat::from_rotation_x(FRAC_PI_2),
                                    ..default()
                                },
                                material: config.laser_blue_mat.clone(),
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
