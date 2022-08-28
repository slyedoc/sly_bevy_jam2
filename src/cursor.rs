use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContext, prelude::*};
use bevy_mod_outline::Outline;
use iyes_loopless::state::CurrentState;
use sly_physics::prelude::*;

use crate::{camera::CameraState, hide_window};

use super::Keep;

pub struct CursorPlugin;

#[derive(Component)]
pub enum CursorInteraction {
    /// The node has been clicked
    Clicked,
    /// The node has been hovered over
    Hovered,
    /// Nothing has happened
    None,
}

pub struct CursorEvent(pub Entity);

impl Default for InteractionTime {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, false),
        }
    }
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CursorEvent>()
            .init_resource::<CursorConfig>()
            .add_plugin(InspectorPlugin::<Inspector>::new())
            .add_startup_system(hide_window::<Inspector>)
            .add_startup_system(setup_cursor)
            .add_system(cursor_raycast)
            .add_system(advance_interaction_timers.after(cursor_raycast))
            .add_system(clear_interactions.after(advance_interaction_timers))
            .add_system(interaction_check.after(clear_interactions));
    }
}

#[derive(Inspectable, Default)]
pub struct Inspector {
    #[inspectable(deletable = true)]
    active: Option<Entity>,
}

pub struct CursorConfig {
    pub hover: Color,
    pub clicked: Color,
    pub disabled: Color,
    pub width: f32,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            hover: Color::LIME_GREEN,
            clicked: Color::GREEN,
            disabled: Color::RED,
            width: 10.0,
        }
    }
}

#[derive(Component)]
pub struct Cursor;

fn setup_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.01,
                sectors: 8,
                stacks: 8,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 0.0, 0.0, 0.2),
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            //visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(Cursor)
        .insert(NotShadowCaster)
        .insert(Keep)
        .insert(Name::new("Cursor"));
}

fn cursor_raycast(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &Transform), Without<Cursor>>,
    mut cusror_query: Query<(&mut Transform, &mut Visibility), With<Cursor>>,
    tlas: Res<Tlas>,
    mut egui_context: ResMut<EguiContext>,

    mut interaction_event: EventWriter<CursorEvent>,
    camera_state: Res<CurrentState<CameraState>>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        if !camera.is_active {
            continue;
        }

        let window = windows.primary();
        if egui_context.ctx_mut().wants_pointer_input() {
            return;
        }
        if let Some(mouse_pos) = window.cursor_position() {
            let (mut cursor_trans, mut cursor_vis) = cusror_query.single_mut();

            // create a ray
            let mut ray = match camera_state.0 {
                CameraState::Player => {
                    Ray::new(camera_transform.translation, camera_transform.forward())
                }
                _ => Ray::from_camera(camera, camera_transform, mouse_pos),
            };

            // test ray agaist tlas and see if we hit
            if let Some(hit) = ray.intersect_tlas(&tlas) {
                cursor_trans.translation = ray.origin + ray.direction * hit.distance;
                cursor_vis.is_visible = true;
                interaction_event.send(CursorEvent(hit.entity));
            } else {
                cursor_vis.is_visible = false;
            }
        }
    }
}

// Used to limit when entity can be interacted with
// TODO: I hate the name
#[derive(Component)]
pub struct InteractionTime {
    pub timer: Timer,
}

fn advance_interaction_timers(mut query: Query<&mut InteractionTime>, time: Res<Time>) {
    for mut interaction_timer in query.iter_mut() {
        interaction_timer.timer.tick(time.delta());
    }
}

pub fn clear_interactions(mut query: Query<(&mut CursorInteraction, Option<&mut Outline>)>) {
    for (mut interaction, outline) in query.iter_mut() {
        *interaction = CursorInteraction::None;
        if let Some(mut outline) = outline {
            outline.visible = false;
        }
    }
}

pub fn interaction_check(
    mut cursor_events: EventReader<CursorEvent>,
    mut query: Query<(
        &mut CursorInteraction,
        Option<&InteractionTime>,
    )>,
    mouse_input: Res<Input<MouseButton>>,
    mut inspector: ResMut<Inspector>,
) {
    for event in cursor_events.iter() {
        let mut clicked = mouse_input.just_pressed(MouseButton::Left);

        // see if the entity is interactable
        if let Ok((mut interaction, interaction_time_maybe)) = query.get_mut(event.0)
        {
            // ignore click if timer is active
            if let Some(interaction_time) = interaction_time_maybe {
                if !interaction_time.timer.finished() {
                    clicked = false;
                }
            }

            if clicked {
                inspector.active = Some(event.0);
                *interaction = CursorInteraction::Clicked;

            } else {
                inspector.active = None;
                *interaction = CursorInteraction::Hovered;

            }
        }
    }
}
