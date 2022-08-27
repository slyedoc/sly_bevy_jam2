use crate::GameState;
use bevy::{math::vec3, prelude::*};

use bevy_inspector_egui::Inspectable;
use bevy_tweening::{lens::*, *};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use super::SwitchEvent;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DoorConfig>()
            .add_system(spawn_door)
            .add_system(switch_event.run_in_state(GameState::Playing));
        //.add_system(interaction_check.run_in_state(GameState::Playing));
    }
}

fn switch_event(
    mut commands: Commands,
    mut switch_events: EventReader<SwitchEvent>,
    mut door_query: Query<&mut Door>,
    mut child_animator: Query<(Entity, &Parent, &Transform), With<DoorSlider>>,
    config: Res<DoorConfig>,
) {
    for switch_event in switch_events.iter() {
        let door_entity = switch_event.0;
        if let Ok(mut door) = door_query.get_mut(door_entity) {
            for (slider_entity, parent, _transform) in child_animator.iter_mut() {
                if parent.get() == door_entity {
                    *door = match *door {
                        Door::Open => {
                            commands
                                .entity(slider_entity)
                                .insert(Animator::new(Tween::new(
                                    EaseMethod::EaseFunction(EaseFunction::SineInOut),
                                    TweeningType::Once,
                                    std::time::Duration::from_secs(1),
                                    TransformPositionLens {
                                        start: Vec3::new(0.0, config.height * 1.5, 0.0),
                                        end: Vec3::new(0.0, config.height * 0.5, 0.0),
                                    },
                                )));
                            commands.entity(door_entity).insert_bundle(RigidBodyBundle {
                                mode: RigidBodyMode::Static,
                                collider: config.door_collider.clone(),
                                ..default()
                            });

                            Door::Closed
                        }
                        Door::Closed => {
                            commands
                                .entity(slider_entity)
                                .insert(Animator::new(Tween::new(
                                    EaseMethod::EaseFunction(EaseFunction::SineInOut),
                                    TweeningType::Once,
                                    std::time::Duration::from_secs(1),
                                    TransformPositionLens {
                                        start: Vec3::new(0.0, config.height * 0.5, 0.0),
                                        end: Vec3::new(0.0, config.height * 1.5, 0.0),
                                    },
                                )));
                            commands
                                .entity(door_entity)
                                .remove_bundle::<RigidBodyBundle>();
                            Door::Open
                        }
                    };
                }
            }
        }
    }
}

#[derive(Component, Inspectable, Debug, PartialEq, Eq)]
pub enum Door {
    Open,
    Closed,
}

pub struct DoorConfig {
    pub height: f32,
    pub width: f32,
    pub frame_width: f32,
    pub frame_depth: f32,
    pub door_thickness: f32,

    frame_side_mesh: Handle<Mesh>,
    frame_top_mesh: Handle<Mesh>,
    door_mesh: Handle<Mesh>,

    frame_mat: Handle<StandardMaterial>,
    door_mat: Handle<StandardMaterial>,

    door_collider: Collider,
}

impl FromWorld for DoorConfig {
    fn from_world(world: &mut World) -> Self {
        let height = 2.5;
        let width = 1.5;
        let frame_width = 0.2;
        let frame_depth = 1.1;
        let door_thickness = 0.2;

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        let frame_side_mesh = meshes.add(Mesh::from(shape::Box::new(
            frame_width,
            height,
            frame_depth,
        )));
        let frame_top_mesh = meshes.add(Mesh::from(shape::Box::new(
            width + (frame_width * 2.0),
            frame_width,
            frame_depth,
        )));
        let door_mesh = meshes.add(Mesh::from(shape::Box::new(width, height, door_thickness)));

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let frame_mat = materials.add(StandardMaterial {
            base_color: Color::GRAY,
            ..default()
        });
        let door_mat = materials.add(StandardMaterial {
            base_color: Color::DARK_GRAY,
            ..default()
        });

        let mut collider_resources = world.get_resource_mut::<ColliderResources>().unwrap();
        let collider = collider_resources.add_box(vec3(width, height * 2.0, door_thickness));

        Self {
            height,
            width,
            frame_width,
            frame_depth,
            door_thickness,
            frame_side_mesh,
            frame_top_mesh,
            door_mesh,
            frame_mat,
            door_mat,
            door_collider: collider,
        }
    }
}

#[derive(Component)]
struct DoorSlider;

pub fn spawn_door(
    mut commands: Commands,
    query: Query<(Entity, &Door), Added<Door>>,
    config: Res<DoorConfig>,
) {
    for (e, door) in query.iter() {
        // add switch boarder
        commands
            .entity(e)
            .insert(Name::new("Door"))
            .with_children(|parent| {
                // add frame
                let half_width = config.width * 0.5;
                let half_height = config.height * 0.5;
                let half_frame_width = config.frame_width * 0.5;

                // left frame
                parent.spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(half_width + half_frame_width, half_height, 0.0),
                    mesh: config.frame_side_mesh.clone(),
                    material: config.frame_mat.clone(),
                    ..PbrBundle::default()
                });

                // right frame
                parent.spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(
                        -half_width - half_frame_width,
                        half_height,
                        0.0,
                    ),
                    mesh: config.frame_side_mesh.clone(),
                    material: config.frame_mat.clone(),
                    ..default()
                });

                // top frame
                parent.spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(0.0, config.height, 0.0),
                    mesh: config.frame_top_mesh.clone(),
                    material: config.frame_mat.clone(),
                    ..default()
                });

                // door

                parent
                    .spawn_bundle(PbrBundle {
                        transform: Transform::from_xyz(
                            0.0,
                            match door {
                                Door::Open => half_height * 1.5,
                                Door::Closed => half_height,
                            },
                            0.0,
                        ),
                        mesh: config.door_mesh.clone(),
                        material: config.door_mat.clone(),
                        ..default()
                    })
                    .insert(DoorSlider);
            });

        if *door == Door::Closed {
            commands.entity(e).insert_bundle(RigidBodyBundle {
                mode: RigidBodyMode::Static,
                collider: config.door_collider.clone(),
                ..default()
            });
        }
    }
}
