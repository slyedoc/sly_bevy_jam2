use std::f32::consts::*;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::GameState;

use super::{Door, DoorConfig, Switch, SwitchState};

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TrainingRoomConfig>()
            .init_resource::<WallConfig>()
            .add_system(spawn_wall.run_in_state(GameState::Playing));
    }
}

pub struct TrainingRoomConfig {
    floor_size: f32,
    thinkness: f32,
    half_thinkess: f32,
    wall_height: f32,
    floor_half: f32,
    wall_height_half: f32,
}

impl Default for TrainingRoomConfig {
    fn default() -> Self {
        let floor_size = 15.0;
        let thinkess = 1.0;
        let half_thinkess = thinkess * 0.5;
        let wall_height = 4.0;
        let floor_half = floor_size * 0.5;
        let wall_height_half = wall_height * 0.5;

        TrainingRoomConfig {
            floor_size,
            thinkness: thinkess,
            half_thinkess,
            wall_height,
            floor_half,
            wall_height_half,
        }
    }
}

pub fn spawn_training_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_resources: ResMut<ColliderResources>,
    training_room_config: Res<TrainingRoomConfig>,
    door_config: Res<DoorConfig>,
) {
    // floor
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, -training_room_config.half_thinkess, 0.0),
            mesh: meshes.add(Mesh::from(shape::Box::new(
                training_room_config.floor_size,
                training_room_config.thinkness,
                training_room_config.floor_size,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(
                training_room_config.floor_size,
                training_room_config.thinkness,
                training_room_config.floor_size,
            )),
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Floor"));

    // cieling
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(
                0.0,
                training_room_config.half_thinkess + training_room_config.wall_height,
                0.0,
            ),
            mesh: meshes.add(Mesh::from(shape::Box::new(
                training_room_config.floor_size,
                training_room_config.thinkness,
                training_room_config.floor_size,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(
                training_room_config.floor_size,
                training_room_config.thinkness,
                training_room_config.floor_size,
            )),
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Floor"));

    //light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, training_room_config.wall_height, 0.0),
        ..default()
    });

    // walls
    for wall in 0..4 {
        if wall == 0 {
            // Front Wall with Door
            let part_size =
                (training_room_config.floor_size - door_config.width - door_config.frame_width)
                    * 0.5;
            let part_offset = (door_config.width * 0.5) + part_size * 0.5 + door_config.frame_width;

            for offset in [-part_offset, part_offset] {
                commands
                    .spawn_bundle(SpatialBundle {
                        transform: Transform {
                            translation: vec3(
                                offset,
                                training_room_config.wall_height_half,
                                training_room_config.floor_half,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Wall {
                        size: vec2(part_size, training_room_config.wall_height),
                    });
            }
            // door seal
            let seal_height = training_room_config.wall_height
                - (door_config.frame_width * 0.5 + door_config.height);
            commands
                .spawn_bundle(SpatialBundle {
                    transform: Transform {
                        translation: vec3(
                            0.0,
                            door_config.height
                                + (door_config.frame_width * 0.5)
                                + (seal_height * 0.5),
                            training_room_config.floor_half,
                        ),
                        ..default()
                    },
                    ..default()
                })
                .insert(Wall {
                    size: vec2(
                        door_config.width + (door_config.frame_width * 2.0),
                        seal_height,
                    ),
                });
        } else {
            let mut transform = Transform::from_xyz(
                0.0,
                training_room_config.wall_height_half,
                training_room_config.floor_half,
            );
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(Vec3::Y, wall as f32 * FRAC_PI_2),
            );

            commands
                .spawn_bundle(SpatialBundle {
                    transform,
                    ..default()
                })
                .insert(Wall {
                    size: vec2(
                        training_room_config.floor_size,
                        training_room_config.wall_height,
                    ),
                });
        }
    }

    // door
    let door_id = commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(0.0, 0.0, training_room_config.floor_half),
                rotation: Quat::from_axis_angle(Vec3::Y, PI),
                ..default()
            },

            ..default()
        })
        .insert(Door::Closed)
        .id();

    // switch
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(
                    door_config.width * 0.5 + 0.5,
                    training_room_config.thinkness,
                    training_room_config.floor_half - training_room_config.half_thinkess,
                ),
                rotation: Quat::from_axis_angle(Vec3::Y, PI),
                ..default()
            },
            ..default()
        })
        .insert(Switch {
            target: door_id,
            state: SwitchState::Off,
        });
}

pub struct WallConfig {
    pub thickness: f32,
    wall_mat: Handle<StandardMaterial>,
}

impl FromWorld for WallConfig {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let wall_mat = materials.add(StandardMaterial {
            base_color: Color::ALICE_BLUE,
            ..default()
        });

        WallConfig {
            thickness: 1.0,
            wall_mat,
        }
    }
}

#[derive(Component)]
pub struct Wall {
    size: Vec2,
}

fn spawn_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Wall), Added<Wall>>,
    config: Res<WallConfig>,
    mut collider_resources: ResMut<ColliderResources>,
) {
    for (e, wall) in query.iter() {
        commands
            .entity(e)
            .insert(meshes.add(Mesh::from(shape::Box::new(
                wall.size.x,
                wall.size.y,
                config.thickness,
            ))))
            .insert(config.wall_mat.clone())
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .insert_bundle(RigidBodyBundle {
                collider: collider_resources.add_box(vec3(
                    wall.size.x,
                    wall.size.y,
                    config.thickness,
                )),
                mode: RigidBodyMode::Static,
                ..default()
            })
            .insert(Name::new("Wall"));
    }
}
