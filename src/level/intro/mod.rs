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
            .add_enter_system(LevelState::Intro, setup)
            .add_exit_system(LevelState::Intro, cleanup);
    }
}

pub fn setup(
    mut commands: Commands,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    training_room_config: Res<TrainingRoomConfig>,
) {
    let mut camera_trans = camera_query.single_mut();
    camera_trans.translation = vec3(0.0, 1.0, -3.0);
    camera_trans.look_at(vec3(0.0, 0.8, 0.0), Vec3::Y);

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(3.0, 1.0, 0.0),
            ..default()
        })
        .insert(Nexus::Idle);

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Alien);

    commands
        .spawn_bundle(SpatialBundle {
            transform: Transform::from_xyz(1.0, 0.0, 0.0),
            ..default()
        })
        .insert(Desk::ComputerScreen);

    // commands
    //     .spawn_bundle(SpatialBundle {
    //         transform: Transform::from_xyz(2.0, 0.0, 0.0),
    //         ..default()
    //     })
    //     .insert(Desk::ComputerCorner);

    // commands
    //     .spawn_bundle(SpatialBundle {
    //         transform: Transform::from_xyz(3.0, 0.0, 0.0),
    //         ..default()
    //     })
    //     .insert(Desk::ComputerScreen);
}
