use std::{ops::Range, time::Duration};

use super::{space_kit::*, AIAudioChannel, AIHighConfig, Pellet, PelletConfig, SwitchEvent};
use crate::{
    assets::{AIAudioAssets, CLEAR},
    states::{GameTimer, HighScore, Score},
    GameState,
};
use bevy::{math::vec3, prelude::*};
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_mod_outline::{Outline, OutlineBundle};
use iyes_loopless::prelude::*;
use rand::Rng;
use sly_physics::prelude::*;

pub struct DispenserPlugin;

impl Plugin for DispenserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTimer>()
            .add_system(spawn_dispenser.run_in_state(GameState::Playing))
            .add_system(switch_event.run_in_state(GameState::Playing))
            .add_system(dispense_pellets.run_in_state(GameState::Playing))
            .add_system(update_game_timer.run_in_state(GameState::Playing));
    }
}

#[derive(Component, Debug)]
pub struct Dispenser {
    pub count: usize,
    pub timer: Timer,
    pub pellet_delay: Range<f32>,
    pub pellet_velocity: Range<f32>,
    pub pellet_direction: Range<f32>,
}

impl Default for Dispenser {
    fn default() -> Self {
        Self {
            count: Default::default(),
            timer: Default::default(),
            pellet_delay: 0.3..1.5,
            pellet_velocity: -4.0..-3.0,
            pellet_direction: -1.0..1.0,
        }
    }
}

fn spawn_dispenser(mut commands: Commands, query: Query<Entity, Added<Dispenser>>) {
    for e in query.iter() {
        commands
            .entity(e)
            .insert(Name::new("Dispenser"))
            .insert(SpaceKit::Rocket(Rocket::BaseA));
    }
}

fn update_game_timer(
    mut commands: Commands,
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    pellet_query: Query<Entity, With<Pellet>>,
    mut high_config: ResMut<AIHighConfig>,
    channel: Res<AudioChannel<AIAudioChannel>>,
) {
    game_timer.0.tick(time.delta());

    if game_timer.0.just_finished() {
        if score.0 > high_score.0 {
            let handle = high_config.next();
            channel.play(handle).with_volume(0.4);
        }
        reset(&mut score, &mut high_score, &pellet_query, &mut commands);
    }
}

fn switch_event(
    mut commands: Commands,
    mut switch_events: EventReader<SwitchEvent>,
    mut dispenser_query: Query<&mut Dispenser>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    pellet_query: Query<Entity, With<Pellet>>,
    channel: Res<AudioChannel<AIAudioChannel>>,
    audio_assets: Res<AIAudioAssets>,
    mut game_timer: ResMut<GameTimer>,
) {
    for switch_event in switch_events.iter() {
        let dispenser_entity = switch_event.0;
        if let Ok(mut dispenser) = dispenser_query.get_mut(dispenser_entity) {
            info!("start");
            // start timer
            game_timer.0.set_duration(Duration::from_secs(30));
            game_timer.0.reset();

            channel.play(audio_assets.start.clone());

            reset(&mut score, &mut high_score, &pellet_query, &mut commands);

            dispenser.count = 20;
        }
    }
}

fn reset(
    score: &mut Score,
    high_score: &mut HighScore,
    pellet_query: &Query<Entity, With<Pellet>>,
    commands: &mut Commands,
) {
    // save high score
    if score.0 > high_score.0 {
        high_score.0 = score.0;
    }
    // reset score
    score.0 = 0;

    // clear any pellets
    for e in pellet_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn dispense_pellets(
    mut commands: Commands,
    mut query: Query<(&mut Dispenser, &Transform)>,
    pellet_config: Res<PelletConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (mut dispenser, dispenser_trans) in query.iter_mut() {
        dispenser.timer.tick(time.delta());
        if dispenser.timer.finished() && dispenser.count > 0 {
            let delay = rng.gen_range(dispenser.pellet_delay.clone());
            dispenser.timer.set_duration(Duration::from_secs_f32(delay));
            dispenser.timer.reset();

            commands
                .spawn_bundle(SpatialBundle {
                    transform: Transform::from_translation(
                        dispenser_trans.translation + vec3(-1.0, 0.0, 0.0),
                    ),
                    ..default()
                })
                .insert(pellet_config.mesh.clone())
                .insert(materials.add(StandardMaterial {
                    base_color: CLEAR,
                    unlit: true,
                    ..default()
                }))
                .insert_bundle(RigidBodyBundle {
                    collider: pellet_config.collider.clone(),
                    mass: Mass(1.0),
                    linear_velocity: LinearVelocity(vec3(
                        rng.gen_range(dispenser.pellet_velocity.clone()),
                        rng.gen_range(dispenser.pellet_direction.clone()),
                        rng.gen_range(dispenser.pellet_direction.clone()),
                    )),
                    ..default()
                })
                .insert_bundle(OutlineBundle {
                    outline: Outline {
                        visible: true,
                        width: 2.0,
                        ..default()
                    },
                    ..default()
                })
                .insert(Pellet {
                    value: match rng.gen::<bool>() {
                        true => 0.0,
                        false => 1.0,
                    },
                    ..default()
                })
                .insert(Name::new("Pellet"));

            if dispenser.count > 0 {
                dispenser.count -= 1;
            }
        }
    }
}
