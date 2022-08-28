use std::f32::consts::*;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{GameState, assets::TextureAssets};

use super::{Door, DoorConfig, Switch, SwitchState};

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoomConfig>()
            .init_resource::<WallConfig>()
            .add_system(spawn_wall.run_in_state(GameState::Playing));
    }
}

pub struct RoomConfig {
    pub thinkness: f32,
    pub half_thinkess: f32,

    pub intro_floor_size: f32,
    pub intro_floor_half: f32,

    pub landing_floor_size: Vec2,

    pub wall_height: f32,
    pub wall_height_half: f32,

    pub reactor_radius: f32,
    pub reactor_length: f32,
    pub reactor_center_z: f32,
}

impl Default for RoomConfig {
    fn default() -> Self {
        let intro_floor_size = 15.0;
        let thinkess = 1.0;
        let half_thinkess = thinkess * 0.5;
        let wall_height = 4.0;
        let intro_floor_half = intro_floor_size * 0.5;
        let wall_height_half = wall_height * 0.5;
        let landing_floor_size = vec2(intro_floor_size, 7.0);

        let reactor_radius = 8.0;
        let reactor_length = 40.0;
        let reactor_center_z = intro_floor_half + landing_floor_size.x - half_thinkess;

        RoomConfig {
            intro_floor_size,
            landing_floor_size: vec2(intro_floor_size, 7.0),
            thinkness: thinkess,
            half_thinkess,
            wall_height,
            intro_floor_half,
            wall_height_half,
            reactor_radius,
            reactor_length,
            reactor_center_z,
        }
    }
}

pub fn spawn_training_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_resources: ResMut<ColliderResources>,
    room_config: Res<RoomConfig>,
    door_config: Res<DoorConfig>,
    texture_assets: Res<TextureAssets>,
) {
    // floor
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, -room_config.half_thinkess, 0.0),
            mesh: meshes.add(Mesh::from(shape::Box::new(
                room_config.intro_floor_size,
                room_config.thinkness,
                room_config.intro_floor_size,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                base_color_texture: Some(texture_assets.pattern_01.clone()),
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(
                room_config.intro_floor_size,
                room_config.thinkness,
                room_config.intro_floor_size,
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
                room_config.half_thinkess + room_config.wall_height,
                0.0,
            ),
            mesh: meshes.add(Mesh::from(shape::Box::new(
                room_config.intro_floor_size,
                room_config.thinkness,
                room_config.intro_floor_size,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(
                room_config.intro_floor_size,
                room_config.thinkness,
                room_config.intro_floor_size,
            )),
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Floor"));

    //light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, room_config.wall_height, 0.0),

        ..default()
    });

    // walls
    for wall in 0..4 {
        if wall == 0 {
            // Front Wall with Door
            let part_size =
                (room_config.intro_floor_size - door_config.width - door_config.frame_width) * 0.5;
            let part_offset = (door_config.width * 0.5) + part_size * 0.5 + door_config.frame_width;

            for offset in [-part_offset, part_offset] {
                commands
                    .spawn_bundle(SpatialBundle {
                        transform: Transform {
                            translation: vec3(
                                offset,
                                room_config.wall_height_half,
                                room_config.intro_floor_half,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Wall {
                        size: vec2(part_size, room_config.wall_height),
                        ..default()
                    });
            }
            // door seal
            let seal_height =
                room_config.wall_height - (door_config.frame_width * 0.5 + door_config.height);
            commands
                .spawn_bundle(SpatialBundle {
                    transform: Transform {
                        translation: vec3(
                            0.0,
                            door_config.height
                                + (door_config.frame_width * 0.5)
                                + (seal_height * 0.5),
                            room_config.intro_floor_half,
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
                    ..default()
                });
        } else {
            let mut transform = Transform::from_xyz(
                0.0,
                room_config.wall_height_half,
                room_config.intro_floor_half,
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
                    size: vec2(room_config.intro_floor_size, room_config.wall_height),
                    ..default()
                });
        }
    }

    // door
    let door_id = commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(0.0, 0.0, room_config.intro_floor_half),
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
                    room_config.thinkness,
                    room_config.intro_floor_half - room_config.half_thinkess,
                ),
                rotation: Quat::from_axis_angle(Vec3::Y, PI),
                ..default()
            },
            ..default()
        })
        .insert(Switch {
            target: door_id,
            state: SwitchState::Disabled,
        });
}

pub fn spawn_reactor_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_resources: ResMut<ColliderResources>,
    room_config: Res<RoomConfig>,
) {
    let landing_offset = room_config.intro_floor_size * 0.5;

    // floor
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(
                0.0,
                -room_config.half_thinkess,
                landing_offset + room_config.landing_floor_size.y * 0.5,
            ),
            mesh: meshes.add(Mesh::from(shape::Box::new(
                room_config.landing_floor_size.x,
                room_config.thinkness,
                room_config.landing_floor_size.y,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(
                room_config.landing_floor_size.x,
                room_config.thinkness,
                room_config.landing_floor_size.y,
            )),
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Landing Floor"));

    // cieling
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(
                0.0,
                room_config.half_thinkess + room_config.wall_height,
                landing_offset + room_config.landing_floor_size.y * 0.5,
            ),
            mesh: meshes.add(Mesh::from(shape::Box::new(
                room_config.landing_floor_size.x,
                room_config.thinkness,
                room_config.landing_floor_size.y,
            ))),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(
                room_config.landing_floor_size.x,
                room_config.thinkness,
                room_config.landing_floor_size.y,
            )),
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Landing Cieling"));

    // walls
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(
                    -room_config.landing_floor_size.x * 0.5 - room_config.half_thinkess,
                    room_config.wall_height_half,
                    landing_offset + room_config.landing_floor_size.y * 0.5,
                ),
                rotation: Quat::from_axis_angle(Vec3::Y, FRAC_PI_2),
                ..default()
            },
            ..default()
        })
        .insert(Wall {
            size: vec2(room_config.landing_floor_size.y, room_config.wall_height),
            ..default()
        });

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(
                    room_config.landing_floor_size.x * 0.5 + room_config.half_thinkess,
                    room_config.wall_height_half,
                    landing_offset + room_config.landing_floor_size.y * 0.5,
                ),
                rotation: Quat::from_axis_angle(Vec3::Y, FRAC_PI_2),
                ..default()
            },
            ..default()
        })
        .insert(Wall {
            size: vec2(room_config.landing_floor_size.y, room_config.wall_height),
            ..default()
        });

    // reactor
    for i in 0..6 {
        if i == 3 {
            continue;
        }
        let mut transform = Transform::from_xyz(
            0.0,
            room_config.wall_height_half,
            room_config.reactor_center_z + room_config.reactor_radius,
        );

        transform.rotate_around(
            vec3(
                0.0,
                room_config.wall_height_half,
                room_config.reactor_center_z,
            ),
            Quat::from_axis_angle(Vec3::X, i as f32 * FRAC_PI_3),
        );

        commands
            .spawn_bundle(SpatialBundle {
                transform,
                ..default()
            })
            .insert(Wall {
                size: vec2(
                    room_config.reactor_length,
                    (room_config.reactor_radius + 1.0) * FRAC_PI_3,
                ),
                wall_type: WallType::Reactor,
            });
    }

    // reactor end caps
    for sign in [-1.0, 1.0] {
        commands.spawn_bundle(PointLightBundle {
            transform: Transform {
                translation: vec3(
                    sign * ((room_config.reactor_length - 1.0) * 0.5),
                    room_config.wall_height_half,
                    room_config.reactor_center_z,
                ),
                ..default()
            },
            ..default()
        });

        commands
            .spawn_bundle(SpatialBundle {
                transform: Transform {
                    translation: vec3(
                        sign * (room_config.reactor_length * 0.5),
                        room_config.wall_height_half,
                        room_config.reactor_center_z,
                    ),
                    rotation: Quat::from_axis_angle(Vec3::Y, FRAC_PI_2),
                    ..default()
                },
                ..default()
            })
            .insert(Wall {
                size: vec2(
                    room_config.reactor_radius * 3.0,
                    room_config.reactor_radius * 3.0,
                ),
                wall_type: WallType::Reactor,
            });
    }
}

pub struct WallConfig {
    pub thickness: f32,
    wall_mat: Handle<StandardMaterial>,
    reactor_mat: Handle<StandardMaterial>,
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
        let reactor_mat = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });

        WallConfig {
            thickness: 1.0,
            wall_mat,
            reactor_mat,
        }
    }
}

#[derive(Component)]
pub struct Wall {
    pub size: Vec2,
    pub wall_type: WallType,
}

impl Default for Wall {
    fn default() -> Self {
        Wall {
            size: vec2(1.0, 1.0),
            wall_type: WallType::Default,
        }
    }
}

pub enum WallType {
    Default,
    Reactor,
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
            .insert(match wall.wall_type {
                WallType::Default => config.wall_mat.clone(),
                WallType::Reactor => config.reactor_mat.clone(),
            })
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
