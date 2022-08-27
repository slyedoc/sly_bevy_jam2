use std::f32::consts::*;

use crate::camera::MainCamera;
use crate::cleanup;
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
            .add_enter_system(LevelState::Intro, setup)
            .add_exit_system(LevelState::Intro, cleanup);
    }
}

pub fn setup(
    mut commands: Commands,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera_trans = camera_query.single_mut();
    camera_trans.translation = vec3(0.0, 1.0, -3.0);
    camera_trans.look_at(vec3(0.0, 0.8, 0.0), Vec3::Y);

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
                    translation: vec3(-3.0, 0.0, 6.0),
                    ..default()
                },
                ..default()
            })
            .insert(SpaceKit::Desk(Desk::ComputerScreen));

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
            transform: Transform::from_xyz(-3.0, 0.0, 3.0),
            ..default()
        })
        .insert(SpaceKit::Character(Character::AstronautA));
}
