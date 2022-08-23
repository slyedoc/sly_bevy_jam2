use bevy::{prelude::*, math::vec3};
use sly_physics::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_resources: ResMut<ColliderResources>,
) {
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 1.0, -10.0),
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 0.3,
            depth: 1.4,
            ..default()
        })),        
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        ..default()
    }).insert_bundle(RigidBodyBundle {
        collider: collider_resources.add_box(vec3(0.5, 2.0, 0.5)),
        mode: RigidBodyMode::Dynamic,
        ..default()
    }).insert(Name::new("Player"));
}

#[derive(Component)]
pub struct PlayerController {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub walk_velocity: f32,
    pub jump_velocity: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            back: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            walk_velocity: 1.0,
            jump_velocity: 1.0,
        }
    }
}