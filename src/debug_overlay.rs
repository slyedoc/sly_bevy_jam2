use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use crate::{assets::FontAssets, GameState, Keep};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum DebugOverlayState {
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
struct PhysicsStateText;

pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_loopless_state(DebugOverlayState::Paused)
            .add_system(toggle_debug_overlay)
            .add_enter_system(DebugOverlayState::Running, setup_overlay)
            .add_system(update_fps)
            .add_system(update_state)
            .add_system(update_physcis_debug)
            .add_exit_system(DebugOverlayState::Running, despawn_overlay);
    }
}

fn toggle_debug_overlay(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    overlay_state: Res<CurrentState<DebugOverlayState>>,
) {
    if input.just_pressed(KeyCode::Key1) {
        let target = match overlay_state.0 {
            DebugOverlayState::Paused => DebugOverlayState::Running,
            DebugOverlayState::Running => DebugOverlayState::Paused,
        };
        commands.insert_resource(NextState(target));
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

fn update_state(
    state: Res<CurrentState<GameState>>,
    mut query: Query<&mut Text, With<GameStateText>>,
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
