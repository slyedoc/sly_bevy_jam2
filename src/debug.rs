use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{WorldInspectorParams};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{assets::FontAssets, GameState, Keep, show_window, cursor::Inspector, hide_window, LevelState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Debug {
    Running,
    Paused,
}

#[derive(Component)]
struct DebugOverlay;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct GameStateText;

#[derive(Component)]
struct LevelStateText;

#[derive(Component)]
struct PhysicsStateText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_loopless_state(Debug::Paused)
            .add_system(toggle_debug)
            .add_system(toggle_physics_debug)
            .add_enter_system(Debug::Running, setup_overlay)
            .add_enter_system(Debug::Running, show_window::<Inspector>)
            .add_system(update_fps.run_in_state(Debug::Running))
            .add_system(update_game_state.run_in_state(Debug::Running))
            .add_system(update_level_state.run_in_state(Debug::Running))
            .add_system(update_physcis_debug.run_in_state(Debug::Running))
            .add_exit_system(Debug::Running, despawn_overlay)
            .add_exit_system(Debug::Running, hide_window::<Inspector>);
    }
}

fn toggle_debug(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    overlay_state: Res<CurrentState<Debug>>,
    mut world_inspector: ResMut<WorldInspectorParams>,

) {
    if input.just_pressed(KeyCode::F1) {
        match overlay_state.0 {
            Debug::Paused => {
                commands.insert_resource(NextState(Debug::Running));                
                world_inspector.enabled = true;
            }
            Debug::Running => {
                commands.insert_resource(NextState(Debug::Paused));                
                world_inspector.enabled = false;
            },
        }; 
    }
}

fn toggle_physics_debug(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    state: Res<CurrentState<PhysicsDebugState>>,
) {
    if input.just_pressed(KeyCode::F2) {
        match state.0 {
            PhysicsDebugState::Paused => {
                commands.insert_resource(NextState(PhysicsDebugState::Running));
                
            }
            PhysicsDebugState::Running => {
                commands.insert_resource(NextState(PhysicsDebugState::Paused));                
            },
        }; 
    }
}

fn despawn_overlay(mut commands: Commands, query: Query<Entity, With<DebugOverlay>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup_overlay(mut commands: Commands, font_assets: Res<FontAssets>) {
    let mut offset = 10.0;
    let offset_change = 25.0;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    left: Val::Px(10.0),
                    bottom: Val::Px(offset),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    font_assets.h1("FPS: ".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::GREEN),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui FPS"))
        .insert(FpsText)
        .insert(Keep)
        .insert(DebugOverlay);

    offset += offset_change;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    left: Val::Px(10.0),
                    bottom: Val::Px(offset),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    font_assets.h1("Physics State: ".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::GREEN),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui Physics State"))
        .insert(PhysicsStateText)
        .insert(Keep)
        .insert(DebugOverlay);

    offset += offset_change;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    left: Val::Px(10.0),
                    bottom: Val::Px(offset),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    font_assets.h1("Game State: ".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::GREEN),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui Game State"))
        .insert(GameStateText)
        .insert(Keep)
        .insert(DebugOverlay);

        offset += offset_change;

        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::<Val> {
                        left: Val::Px(10.0),
                        bottom: Val::Px(offset),
                        ..Default::default()
                    },
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                // Use `Text` directly
                text: Text {
                    // Construct a `Vec` of `TextSection`s
                    sections: vec![
                        font_assets.h1("Level State: ".into(), Color::WHITE),
                        font_assets.h1("".into(), Color::GREEN),
                    ],
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("ui Level State"))
            .insert(LevelStateText)
            .insert(Keep)
            .insert(DebugOverlay);
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.0}", average);
                text.sections[1].style.color = match average {
                    x if x >= 50.0 => Color::GREEN,
                    x if x > 40.0 && x < 50.0 => Color::YELLOW,
                    x if x <= 40.0 => Color::RED,
                    _ => Color::WHITE,
                };
            }
        }
    }
}

fn update_game_state(
    state: Res<CurrentState<GameState>>,
    mut query: Query<&mut Text, With<GameStateText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{:?}", state.0);
    }
}

fn update_level_state(
    state: Res<CurrentState<LevelState>>,
    mut query: Query<&mut Text, With<LevelStateText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = format!("{:?}", state.0);
    }
}


fn update_physcis_debug(
    state: Res<CurrentState<PhysicsState>>,
    mut query: Query<&mut Text, With<PhysicsStateText>>,
) {
    for mut text in query.iter_mut() {
        text.sections[1].value = match state.0 {
            PhysicsState::Running => "Enabled".to_string(),
            PhysicsState::Paused => "Disabled".to_string(),
        };
        text.sections[1].style.color = match state.0 {
            PhysicsState::Running => Color::GREEN,
            PhysicsState::Paused => Color::RED,
        };
    }
}
