// for jam we dont care about these
#![allow(clippy::type_complexity)]
#![allow(dead_code)] 

mod assets;
mod camera_controller;
mod debug;
mod states;
mod level;
mod prefabs;
mod cursor;

//use crate::actions::ActionsPlugin;
//use crate::audio::InternalAudioPlugin;
use crate::states::*;

use assets::ButtonColors;
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::{prelude::*, plugin::InspectorWindows, WorldInspectorParams};
use bevy_kira_audio::AudioPlugin;
use camera_controller::{CameraController, CameraControllerPlugin};
use level::LevelPlugin;
use prefabs::PrefabPlugin;
use cursor::CursorPlugin;
use debug::DebugPlugin;
use iyes_loopless::prelude::*;
use bevy_tweening::TweeningPlugin;
use sly_physics::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    PreLoading, // loads font for loading screen
    Loading,    // load rest of the assets
    Menu,
    Playing,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LevelState {
    None,
    Intro,
    One,
}

// Marker for things to keep around between states
#[derive(Component)]
pub struct Keep;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::PreLoading)
            .add_loopless_state(LevelState::None)
            .add_plugin(WorldInspectorPlugin::default())
            .insert_resource(WorldInspectorParams {
                enabled: false,
                ..default()
            })            
            .add_plugin(HanabiPlugin)
            .add_plugin(TweeningPlugin)
            .add_plugin(AudioPlugin)
            // physics plugins
            .add_plugin(PhysicsPlugin)
            .add_plugin(GravityPlugin)
            .add_plugin(PhysicsDebugPlugin)
            .add_plugin(PhysicsBvhCameraPlugin)

            // local plugins
            .add_plugin(CameraControllerPlugin)
            .add_plugin(CursorPlugin)
            .add_plugin(PrefabPlugin)
            
            //.add_plugin(ActionsPlugin)
            //.add_plugin(InternalAudioPlugin)
            
            // game states
            .add_plugin(StatePlugin)
            .add_plugin(LevelPlugin)
            // for debugging
            .add_plugin(DebugPlugin)

            .add_startup_system(setup_clearcolor)
            .add_startup_system(setup_cameras)
            .add_system(update_buttons);

        // #[cfg(debug_assertions)]
        // {
        //     app
        //     .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //     .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
        // }
    }
}

fn setup_clearcolor(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}

fn cleanup(mut commands: Commands, q: Query<Entity, Without<Keep>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup_cameras(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController::default())
        .insert(BvhCamera::new(256, 256)) // only used for physics debug
        .insert(Keep);
}

#[allow(clippy::type_complexity)]
fn update_buttons(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
            _ => {}
        }
    }
}

pub fn hide_window<T: Inspectable + Send + Sync + 'static>(
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<T>();
    inspector_window_data.visible = false;
}

pub fn show_window<T: Inspectable + Send + Sync + 'static>(
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<T>();
    inspector_window_data.visible = true;
}
