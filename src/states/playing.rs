use crate::assets::*;
use crate::characters::*;
use crate::{cleanup, GameState};

use bevy::math::vec3;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;
use std::f32::consts::*;


pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, spawn_room)
            .add_enter_system(GameState::Playing, spawn_nexus)
            .add_enter_system(GameState::Playing, setup_buttons)
            .add_system(hotkeys.run_in_state(GameState::Playing))
            .add_system(click_button.run_in_state(GameState::Playing))
            .add_exit_system(GameState::Playing, cleanup);
    }
}

#[derive(Component, Copy, Clone)]
enum PlayingButton {
    Exit,    
}

impl Into<String> for PlayingButton {
    fn into(self) -> String {
        match self {
            PlayingButton::Exit => "Exit".to_string(),            
        }
    }
}

fn setup_buttons(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                position: UiRect::<Val> {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..Default::default()
                },

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![font_assets.h1( PlayingButton::Exit.into(), Color::rgb(0.9, 0.9, 0.9))],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        })
        .insert(PlayingButton::Exit);
}

pub fn hotkeys(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Menu));
    }
}

#[allow(clippy::type_complexity)]
fn click_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &PlayingButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match btn {
                PlayingButton::Exit => commands.insert_resource(NextState(GameState::Menu)),
            }
        }
    }
}

pub fn spawn_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_resources: ResMut<ColliderResources>,
) {
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    let floor_size = 100.0;
    let wall_height = 10.0;
    let floor_half = floor_size * 0.5;
    let wall_height_half = wall_height * 0.5;

    // floor
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            mesh: meshes.add(Mesh::from(shape::Box::new(floor_size, 1.0, floor_size))),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(floor_size, 1.0, floor_size)),
            mode: RigidBodyMode::Static,
            ..default()
        })
        .insert(Name::new("Floor"));

    // walls
    for wall in 0..4 {
        let mut transform = Transform::from_xyz(0.0, wall_height_half, floor_half);
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_axis_angle(Vec3::Y, wall as f32 * FRAC_PI_2),
        );

        commands
            .spawn_bundle(PbrBundle {
                transform,
                mesh: meshes.add(Mesh::from(shape::Box::new(floor_size, wall_height, 1.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::ALICE_BLUE,
                    ..default()
                }),
                ..default()
            })
            .insert_bundle(RigidBodyBundle {
                collider: collider_resources.add_box(vec3(floor_size, wall_height, 1.0)),
                mode: RigidBodyMode::Static,
                ..default()
            })
            .insert(Name::new("Wall"));
    }
}
