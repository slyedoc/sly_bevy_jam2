// from personal crate, copied it here since I will be changing it

#![allow(clippy::type_complexity)]
use std::f32::consts::FRAC_PI_2;

use bevy::{
    input::{mouse::MouseMotion, Input},
    prelude::*,
    window::{WindowFocused, Windows},
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{
    assets::{TextureAssets, CLEAR},
    prefabs::{Player, RoomConfig},
    GameState, Keep,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CameraPlayerConfig>()
            .init_resource::<CameraEditorConfig>()
            .add_loopless_state(CameraState::Static)
            .add_system(toggle_camera)
            // setup one camera for entire app
            .add_startup_system(setup_camera)
            .add_enter_system(GameState::Playing, set_camera_player)
            .add_exit_system(GameState::Playing, set_camera_static)
            .add_enter_system(CameraState::Player, setup_player_camera)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_player_camera.run_in_state(CameraState::Player),
            )
            .add_exit_system(CameraState::Player, exit_player_camera)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_editor_camera.run_in_state(CameraState::Editor),
            );
    }
}

fn set_camera_player(mut commands: Commands) {
    commands.insert_resource(NextState(CameraState::Player));
}

fn set_camera_static(mut commands: Commands) {
    commands.insert_resource(NextState(CameraState::Static));
}

fn setup_player_camera(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    texture_assets: Res<TextureAssets>
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }


    info!("setup crosshair");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                position: UiRect::<Val> {
                    top:  Val::Percent(40.0),                                         
                    left: Val::Percent(40.0),                                        
                    ..default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: CLEAR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                image: UiImage(texture_assets.crossair_black.clone()),
                style: Style {
                    align_self: AlignSelf::Center,
                    size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                    ..default()
                },
                ..default()
            });
        });
}

fn exit_player_camera(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum CameraState {
    Static,
    Editor,
    Player,
}

#[derive(Component)]
pub struct CameraMain;

fn toggle_camera(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    camera_state: Res<CurrentState<CameraState>>,
) {
    if input.just_pressed(KeyCode::F3) {
        match camera_state.0 {
            CameraState::Static => {
                commands.insert_resource(NextState(CameraState::Editor));
            }
            CameraState::Editor => {
                commands.insert_resource(NextState(CameraState::Player));
            }
            CameraState::Player => {
                commands.insert_resource(NextState(CameraState::Static));
            }
        };
    }
}

fn disable_player_camera(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform, &GlobalTransform), With<CameraMain>>,
    player_query: Query<Entity, With<Player>>,
) {
    let (camera_entity, mut camera_transform, global_trans) = camera_query.single_mut();
    let player_entity = player_query.single();
    camera_transform.translation = global_trans.translation();

    commands
        .entity(player_entity)
        .remove_children(&[camera_entity]);
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
        .insert(CameraMain)
        .insert(BvhCamera::new(256, 256))
        .insert(Keep);
}

pub struct CameraEditorConfig {
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

impl Default for CameraEditorConfig {
    fn default() -> Self {
        CameraEditorConfig {
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

fn update_editor_camera(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<CameraMain>>,
    mut windows: ResMut<Windows>,
    mut controller: ResMut<CameraEditorConfig>,
) {
    let dt = time.delta_seconds();
    if let Some(window) = windows.get_primary_mut() {
        for mut transform in query.iter_mut() {
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

pub struct CameraPlayerConfig {
    pub height: f32,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub walk_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraPlayerConfig {
    fn default() -> Self {
        CameraPlayerConfig {
            height: 1.0,
            sensitivity: 0.2,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            walk_speed: 10.0,
            friction: 0.3,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

fn update_player_camera(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<CameraMain>>,
    mut config: ResMut<CameraPlayerConfig>,
    tlas: Res<Tlas>,
    mut windows: ResMut<Windows>,
    mut window_focus_events: EventReader<WindowFocused>,
    mut cursor_left_events: EventReader<CursorLeft>,
    room_config: Res<RoomConfig>,
) {
    let dt = time.delta_seconds();

    // stop stealing cursor if window is unfocused
    // TODO: this is last min hacks, find better way, but it kinda this works
    if let Some(window) = windows.get_primary_mut() {
        for _e in window_focus_events.iter() {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        }

        for _e in cursor_left_events.iter() {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }

        if key_input.just_pressed(KeyCode::LAlt) {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        } else if key_input.just_released(KeyCode::LAlt) {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        } else if key_input.pressed(KeyCode::LAlt) {
            return;
        }
    }

    for mut transform in query.iter_mut() {
        // Handle key input
        let right = transform.right();
        let forward = -right.cross(Vec3::Y);

        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(config.key_forward) && ray_check(&tlas, transform.translation, forward)
        {
            axis_input.z += 1.0;
        }
        if key_input.pressed(config.key_back) && ray_check(&tlas, transform.translation, -forward) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(config.key_right) && ray_check(&tlas, transform.translation, right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(config.key_left) && ray_check(&tlas, transform.translation, -right) {
            axis_input.x -= 1.0;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            config.velocity = axis_input.normalize() * config.walk_speed;
        } else {
            let friction = config.friction.clamp(0.0, 1.0);
            config.velocity *= 1.0 - friction;
            if config.velocity.length_squared() < 1e-6 {
                config.velocity = Vec3::ZERO;
            }
        }

        transform.translation += config.velocity.x * dt * right
            + config.velocity.y * dt * Vec3::Y
            + config.velocity.z * dt * forward;

        // Hack, keep user from falling into reactor
        transform.translation.z = transform.translation.z.clamp(
            -100.0,
            room_config.reactor_center_z - room_config.reactor_radius,
        );

        // use tlas to find correct height
        let mut ground_ray = Ray::new(transform.translation, -Vec3::Y);
        if let Some(hit) = ground_ray.intersect_tlas(&tlas) {
            let change = config.height - hit.distance;
            transform.translation.y += change;
        } else {
            transform.translation.y -= 0.1;
        }

        // Handle mouse look on mouse button
        let mut mouse_delta = Vec2::ZERO;
        for mouse_event in mouse_motion.iter() {
            mouse_delta += mouse_event.delta;
        }

        if mouse_delta != Vec2::ZERO {
            let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            yaw -= mouse_delta.x * config.sensitivity * time.delta_seconds();
            pitch -= mouse_delta.y * config.sensitivity * time.delta_seconds();

            let pitch = pitch.clamp(-FRAC_PI_2, FRAC_PI_2);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0)
        }
    }
}

fn ray_check(tlas: &Tlas, origin: Vec3, direction: Vec3) -> bool {
    let mut ray = Ray::new(origin, direction);
    if let Some(hit) = ray.intersect_tlas(tlas) {
        if hit.distance < 1.0 {
            return false;
        }
    }
    true
}
