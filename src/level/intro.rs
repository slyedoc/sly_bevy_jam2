use std::f32::consts::*;

use crate::camera::CameraMain;
use crate::prefabs::*;
use crate::LevelState;

use bevy::math::vec3;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(LevelState::Intro, spawn_training_room)
            .add_enter_system(LevelState::Intro, spawn_reactor_room)
            .add_enter_system(LevelState::Intro, setup);
        //.add_exit_system(LevelState::Intro, cleanup);
    }
}

pub fn setup(mut commands: Commands, mut camera_query: Query<&mut Transform, With<CameraMain>>) {
    let mut camera_trans = camera_query.single_mut();
    camera_trans.translation = vec3(-1.0, 1.7, -3.0);
    camera_trans.look_at(vec3(-1.0, 1.5, 0.0), Vec3::Y);


    
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(2.0, 2.0, 0.0),
            ..default()
        })
        .insert(Pellet {
            value: 0.0,
        });
        


    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(AI::Intro);

    // right of door
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(-6.0, 0.0, 6.0),
            ..default()
        })
        .insert(SpaceKit::Desk(Desk::ComputerCorner));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(-5.0, 0.0, 4.5),
                rotation: Quat::from_rotation_y(FRAC_2_PI),
                ..default()
            },
            ..default()
        })
        .insert(SpaceKit::Desk(Desk::Chair));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(-3.0, 0.0, 6.5),
                ..default()
            },
            ..default()
        })
        .insert(SpaceKit::Desk(Desk::ComputerScreen));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(-3.1, 1.5, 6.6),
                rotation: Quat::from_rotation_y(FRAC_PI_2),
                ..default()
            },
            ..default()
        })
        .insert(PolarityBlaster::Disabled);

    // left of door
    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(3.0, 0.0, 6.0),
                ..default()
            },
            ..default()
        })
        .insert(SpaceKit::Desk(Desk::Computer));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform {
                translation: vec3(3.0, 0.0, 5.0),
                ..default()
            },
            ..default()
        })
        .insert(SpaceKit::Desk(Desk::ChairArms));

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(6.0, 0.0, -6.0),
            ..default()
        })
        .insert(SpaceKit::Barrel(Barrel::Rail));

        commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(6.0, 0.0, -3.0),
            ..default()
        })
        .insert(SpaceKit::Barrel(Barrel::Multiple));
}
