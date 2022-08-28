mod audio;
use std::time::Duration;

pub use audio::*;
use bevy_kira_audio::AudioSource;

use bevy::{input::mouse::MouseMotion, math::vec3, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_kira_audio::prelude::*;
use bevy_tweening::{lens::*, *};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{
    camera::CameraPlayerConfig,
    cursor::{CursorInteraction, InteractionTime},
    GameState,
};

use super::{PolarityBlaster, Switch, SwitchState};

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<AIAudioChannel>()
            .add_event::<AIAnnoyEvent>()
            .add_enter_system(GameState::Playing, setup_annoy_config)
            .add_enter_system(GameState::Playing, setup_intro_config)
            .add_system(advance_intro.run_in_state(GameState::Playing))
            .add_system(spawn_ai.run_in_state(GameState::Playing))
            .add_system(interaction_check.run_in_state(GameState::Playing));

        app.register_inspectable::<AI>();
    }
}

#[derive(Component, Inspectable, PartialEq, Eq)]
pub enum AI {
    Intro,
    Idle,
}

pub fn spawn_ai(
    mut commands: Commands,

    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut collider_resources: ResMut<ColliderResources>,
    query: Query<(Entity, &AI, &Transform), Added<AI>>,
) {
    for (e, ai, trans) in query.iter() {
        let child_mat = materials.add(StandardMaterial {
            base_color: Color::rgba(0.9, 0.9, 0.9, 0.5),
            unlit: true,
            alpha_mode: AlphaMode::Opaque,
            ..default()
        });

        let child_mesh = meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.1,
            ..default()
        }));

        commands
            .entity(e)
            .insert_bundle(RigidBodyBundle {
                collider: collider_resources.add_sphere(0.5),
                mode: RigidBodyMode::Static,
                mass: Mass(2.0),
                ..default()
            })
            .insert(CursorInteraction::None)
            .insert(InteractionTime::default())
            .insert(meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3,
                ..default()
            })))
            .insert(materials.add(StandardMaterial {
                base_color: Color::rgb(0.4, 0.4, 1.0),
                ..default()
            }))
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .insert(Animator::new(Tween::new(
                EaseFunction::SineInOut,
                TweeningType::PingPong,
                std::time::Duration::from_secs(2),
                TransformPositionLens {
                    start: vec3(trans.translation.x, 0.5, trans.translation.z),
                    end: vec3(trans.translation.x, 1.3, trans.translation.z),
                },
            )))
            .insert(Name::new("Nexus"))
            .with_children(|parent| {
                // children floating
                parent
                    .spawn_bundle(SpatialBundle::default())
                    .insert(Animator::new(Tween::new(
                        EaseMethod::Linear,
                        TweeningType::Loop,
                        std::time::Duration::from_secs_f32(3.0),
                        TransformRotationLens {
                            start: Quat::IDENTITY,
                            end: Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI),
                        },
                    )))
                    .with_children(|parent| {
                        let x_offset = 0.4;
                        let y_offset = 0.0;
                        for pos in [
                            // top
                            vec3(-x_offset, y_offset, -x_offset),
                            vec3(x_offset, y_offset, -x_offset),
                            vec3(-x_offset, y_offset, x_offset),
                            vec3(x_offset, y_offset, x_offset),
                            // bottom
                            // vec3(-x_offset, -y_offset, -x_offset),
                            // vec3(x_offset, -y_offset, -x_offset),
                            // vec3(-x_offset, -y_offset, x_offset),
                            // vec3(x_offset, -y_offset, x_offset),
                        ] {
                            parent
                                .spawn_bundle(PbrBundle {
                                    transform: Transform {
                                        translation: pos,
                                        scale: Vec3::splat(0.1),
                                        ..default()
                                    },
                                    mesh: child_mesh.clone(),
                                    material: child_mat.clone(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(PointLightBundle {
                                        point_light: PointLight {
                                            intensity: 5.0,
                                            shadows_enabled: false,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        }
                    });
            });

        if *ai == AI::Intro {
            commands
                .entity(e)
                .insert(IntroTimer(Timer::new(Duration::from_secs_f32(2.0), false)));
        }
    }
}

#[derive(Component)]
pub struct IntroTimer(pub Timer);

fn advance_intro(
    mut query: Query<(&mut AI, &mut IntroTimer)>,
    time: Res<Time>,
    mut blaster_query: Query<&mut PolarityBlaster, Without<Parent>>,
    mut switch_query: Query<&mut Switch>,
    mut intro_config: ResMut<AIIntroConfig>,
    channel: Res<AudioChannel<AIAudioChannel>>,
    audio_sources: Res<Assets<AudioSource>>,
    mut camera_config: ResMut<CameraPlayerConfig>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut paused: Local<bool>,

    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut ai, mut intro_timer) in query.iter_mut() {
        if intro_config.step == 0 {
            camera_config.disable_look = true;
            camera_config.disable_movement = true;
        }

        // hack to skip
        if keyboard_input.just_pressed(KeyCode::S) && keyboard_input.pressed(KeyCode::LAlt) {
            channel.stop();
            
            camera_config.disable_look = false;
            camera_config.disable_movement = false;

            *ai = AI::Idle;

            for mut b in blaster_query.iter_mut() {
                *b = PolarityBlaster::Enabled;
            }

            // enable switch
            let mut s = switch_query.single_mut();
            s.state = SwitchState::Enabled;
            
            intro_config.step = 12;
        }

        intro_timer.0.tick(time.delta());
        if intro_timer.0.finished() {
            if !*paused {
                if let Some(handle) = intro_config.next() {
                    let source = audio_sources.get(&handle).unwrap();
                    let time = source.sound.duration() + Duration::from_secs_f32(1.0);
                    intro_timer.0 = Timer::new(time, false);

                    channel.play(handle).with_volume(0.4);
                }
            }
        }

        match intro_config.step {
            2 => {
                // check for mouse movement
                camera_config.disable_look = false;
                *paused = true;
                let mut mouse_delta = Vec2::ZERO;
                for mouse_event in mouse_motion.iter() {
                    mouse_delta += mouse_event.delta;
                }
                if mouse_delta != Vec2::ZERO {
                    *paused = false;
                }
            }
            4 => {
                // check for movemnt
                camera_config.disable_movement = false;
                *paused = true;
                if keyboard_input.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
                    *paused = false;
                }
            }
            8 => {
                *paused = true;
                for mut b in blaster_query.iter_mut() {
                    *b = PolarityBlaster::Enabled;
                }
                if blaster_query.iter().count() == 0 {
                    *paused = false;
                }
            }

            9 => {
                // enable switch
                let mut s = switch_query.single_mut();
                s.state = SwitchState::Enabled;
            }
            11 => {
                // enable annoy mode
                *ai = AI::Idle;
            }
            _ => {}
        }
    }
}

fn interaction_check(
    mut query: Query<(&AI, &CursorInteraction, &mut InteractionTime)>,
    mut annoy_config: ResMut<AIAnnoyConfig>,
    channel: Res<AudioChannel<AIAudioChannel>>,
    audio_sources: Res<Assets<AudioSource>>,
) {
    for (ai, cursor_interaction, mut interaction_time) in query.iter_mut() {
        match cursor_interaction {
            CursorInteraction::Clicked => {
                if *ai == AI::Idle {
                    let handle = annoy_config.next();
                    let source = audio_sources.get(&handle).unwrap();
                    interaction_time.timer = Timer::new(source.sound.duration(), false);

                    channel.play(handle).with_volume(0.4);
                }
            }
            _ => {
                // do nothing
            }
        }
    }
}

fn make_nexus_shape() -> Vec<Vec3> {
    let half_height = 0.5;
    let half_width = 0.3;
    vec![
        vec3(0.0, half_height, 0.0),
        vec3(-half_width, 0.0, 0.0),
        vec3(half_width, 0.0, 0.0),
        vec3(0.0, 0.0, -half_width),
        vec3(0.0, 0.0, half_width),
        vec3(0.0, -half_height, 0.0),
    ]
}
