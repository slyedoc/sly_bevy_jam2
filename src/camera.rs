// from personal crate, copied it here since I will be changing it

#![allow(clippy::type_complexity)]
use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::{mouse::MouseMotion, Input},
    prelude::*,
    window::Windows,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::BvhCamera;

use crate::{Keep, prefabs::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_loopless_state(CameraState::Main)
        .add_startup_system(setup_camera)
        //.add_system(toggle_camera)
        //.add_enter_system(CameraState::Player, setup_player_camera)
        //.add_exit_system(CameraState::Player, disable_player_camera)
        .add_system_to_stage(CoreStage::PostUpdate, update_camera_controller.run_in_state(CameraState::Main));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum CameraState {
    Main,
    Editor,
    Player,
}

#[derive(Component)]
pub struct MainCamera;


fn toggle_camera(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    camera_state: Res<CurrentState<CameraState>>,    
) {
    if input.just_pressed(KeyCode::F3) {
        match camera_state.0 {
            CameraState::Main => {
                commands.insert_resource(NextState(CameraState::Editor));
            }
            CameraState::Editor => {
                commands.insert_resource(NextState(CameraState::Player));
            }
            CameraState::Player => {
                commands.insert_resource(NextState(CameraState::Main));
            },
        };
    }
}

fn setup_player_camera(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform),  With<MainCamera>>,
    player_query: Query<Entity, With<Player>>,
) {
    let (camera_entity, mut camera_transform) = camera_query.single_mut();
    let player_entity = player_query.single();

    commands.entity(player_entity).push_children(&[camera_entity]);
    *camera_transform = Transform::default();
}

fn disable_player_camera(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform, &GlobalTransform),  With<MainCamera>>,
    player_query: Query<Entity, With<Player>>,
) {
    let (camera_entity, mut camera_transform, global_trans) = camera_query.single_mut();
    let player_entity = player_query.single();
    camera_transform.translation = global_trans.translation();    


    commands.entity(player_entity).remove_children(&[camera_entity]);
}



fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            camera: Camera {
                is_active: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 1.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera)
        .insert(CameraController::default())
        .insert(BvhCamera::new(256, 256))
        .insert(Keep);
}

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_look: MouseButton,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.2,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            mouse_look: MouseButton::Right,
            walk_speed: 10.0,
            run_speed: 30.0,
            friction: 0.3,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

fn update_camera_controller(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mut windows: ResMut<Windows>,
) {
    let dt = time.delta_seconds();
    if let Some(window) = windows.get_primary_mut() {
        for (mut transform, mut controller) in query.iter_mut() {
            if !controller.enabled {
                continue;
            }

            // Handle key input
            let mut axis_input = Vec3::ZERO;
            if key_input.pressed(controller.key_forward) {
                axis_input.z += 1.0;
            }
            if key_input.pressed(controller.key_back) {
                axis_input.z -= 1.0;
            }
            if key_input.pressed(controller.key_right) {
                axis_input.x += 1.0;
            }
            if key_input.pressed(controller.key_left) {
                axis_input.x -= 1.0;
            }
            if key_input.pressed(controller.key_up) {
                axis_input.y += 1.0;
            }
            if key_input.pressed(controller.key_down) {
                axis_input.y -= 1.0;
            }

            // Apply movement update
            if axis_input != Vec3::ZERO {
                let max_speed = if key_input.pressed(controller.key_run) {
                    controller.run_speed
                } else {
                    controller.walk_speed
                };
                controller.velocity = axis_input.normalize() * max_speed;
            } else {
                let friction = controller.friction.clamp(0.0, 1.0);
                controller.velocity *= 1.0 - friction;
                if controller.velocity.length_squared() < 1e-6 {
                    controller.velocity = Vec3::ZERO;
                }
            }
            let forward = transform.forward();
            let right = transform.right();
            transform.translation += controller.velocity.x * dt * right
                + controller.velocity.y * dt * Vec3::Y
                + controller.velocity.z * dt * forward;

            // Handle mouse look on mouse button
            let mut mouse_delta = Vec2::ZERO;
            if mouse_input.pressed(controller.mouse_look) {
                #[cfg(not(target = "wasm32"))]
                window.set_cursor_lock_mode(true);
                window.set_cursor_visibility(false);
            }
            if mouse_input.just_released(controller.mouse_look) {
                #[cfg(not(target = "wasm32"))]
                window.set_cursor_lock_mode(false);
                window.set_cursor_visibility(true);
            }
            if mouse_input.pressed(controller.mouse_look) {
                for mouse_event in mouse_motion.iter() {
                    mouse_delta += mouse_event.delta;
                }
            }

            if mouse_delta != Vec2::ZERO {
                let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
                yaw -= mouse_delta.x * controller.sensitivity * time.delta_seconds();
                pitch -= mouse_delta.y * controller.sensitivity * time.delta_seconds();

                let pitch = pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
                transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0)
            }
        }
    }
}
