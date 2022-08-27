mod audio;
pub use audio::*;
use bevy_kira_audio::AudioSource;

use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_kira_audio::prelude::*;
use bevy_tweening::{lens::*, *};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{
    cursor::{CursorInteraction, InteractionTime},
    GameState,
};

pub struct NexusPlugin;

impl Plugin for NexusPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<NexuxAudioChannel>()
            .add_event::<NexusAnnoyEvent>()
            .add_enter_system(GameState::Playing, setup_annoy_config)
            .add_system(spawn_nexus.run_in_state(GameState::Playing))
            .add_system(interaction_check.run_in_state(GameState::Playing));

        app.register_inspectable::<Nexus>();
    }
}

#[derive(Component, Inspectable)]
pub enum Nexus {
    Busy,
    Idle,
}

pub fn spawn_nexus(
    mut commands: Commands,

    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut collider_resources: ResMut<ColliderResources>,
    query: Query<Entity, Added<Nexus>>,
) {
    for e in query.iter() {
        let shape = make_nexus_shape();
        let collider_index = collider_resources.add_convex(&shape);
        let collider = collider_resources.get_convex(collider_index.index());

        let mesh = meshes.add(Mesh::from(collider.clone()));

        let child_mat = materials.add(StandardMaterial {
            base_color: Color::rgba(0.9, 0.9, 0.9, 0.5),
            unlit: true,
            alpha_mode: AlphaMode::Opaque,
            ..default()
        });

        commands.entity(e)            
            .insert_bundle(RigidBodyBundle {
                collider: collider_index,
                mode: RigidBodyMode::Static,
                mass: Mass(2.0),
                ..default()
            })
            .insert(InteractionTime::default())
            .insert(Nexus::Idle)
            .insert(Name::new("Nexus"))
            .with_children(|parent| {
                // main body
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: mesh.clone(),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.4, 0.4, 1.0),
                            ..default()
                        }),
                        ..default()
                    })
                    .insert(Animator::new(Tween::new(
                        EaseFunction::SineInOut,
                        TweeningType::PingPong,
                        std::time::Duration::from_secs(2),
                        TransformRotationLens {
                            start: Quat::IDENTITY,
                            end: Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI),
                        },
                    )))
                    .insert(Name::new("Body"));

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
                        let x_offset = 0.3;
                        let y_offset = 0.5;
                        for pos in [
                            // top
                            vec3(-x_offset, y_offset, -x_offset),
                            vec3(x_offset, y_offset, -x_offset),
                            vec3(-x_offset, y_offset, x_offset),
                            vec3(x_offset, y_offset, x_offset),
                            // bottom
                            vec3(-x_offset, -y_offset, -x_offset),
                            vec3(x_offset, -y_offset, -x_offset),
                            vec3(-x_offset, -y_offset, x_offset),
                            vec3(x_offset, -y_offset, x_offset),
                        ] {
                            parent
                                .spawn_bundle(PbrBundle {
                                    transform: Transform {
                                        translation: pos,
                                        scale: Vec3::splat(0.1),
                                        ..default()
                                    },
                                    mesh: mesh.clone(),
                                    material: child_mat.clone(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(PointLightBundle {
                                        point_light: PointLight {
                                            intensity: 40.0,
                                            shadows_enabled: true,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        }
                    });
            });
    }
}

fn interaction_check(
    mut query: Query<(&Nexus, &CursorInteraction, &mut InteractionTime)>,
    mut annoy_config: ResMut<NexusAnnoyConfig>,
    channel: Res<AudioChannel<NexuxAudioChannel>>,
    audio_sources: Res<Assets<AudioSource>>,
) {
    for (_nexus, cursor_interaction, mut interaction_time) in query.iter_mut() {
        match cursor_interaction {
            CursorInteraction::Clicked => {
                let handle = annoy_config.next();
                let source = audio_sources.get(&handle).unwrap();
                interaction_time.timer = Timer::new(source.sound.duration(), false);

                channel.play(handle).with_volume(0.4);
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
