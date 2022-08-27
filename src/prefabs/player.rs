use crate::GameState;
use bevy::{math::vec3, prelude::*};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
            .add_system(control_player.run_in_state(GameState::Playing))
            .add_system_set_to_stage(
                PhysicsFixedUpdate,
                ConditionSet::new()
                    .run_in_state(PhysicsState::Running)
                    .before(PhysicsSystems::Resolve)
                    .after(PhysicsSystems::Drag)
                    .with_system(limit_player_rotation)
                    .into(),
            );
    }
}

// TODO: This is a hack, need to add actually axis locking in sly_physics
fn limit_player_rotation(mut player_query: Query<&mut AngularVelocity, With<Player>>) {
    for mut angular_velocity in player_query.iter_mut() {
        angular_velocity.0 = Vec3::ZERO;
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut collider_resources: ResMut<ColliderResources>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 1.0, -5.0),
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.3,
                depth: 1.4,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::CRIMSON,
                ..default()
            }),
            ..default()
        })
        .insert_bundle(RigidBodyBundle {
            collider: collider_resources.add_box(vec3(0.5, 2.0, 0.5)),
            mode: RigidBodyMode::Dynamic,
            elasticity: Elasticity(0.0),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                transform: Transform::from_xyz(0.0, 1.0, 1.0),
                mesh: meshes.add(Mesh::from(shape::Cube::new(0.2))),
                material: materials.add(StandardMaterial {
                    base_color: Color::CRIMSON,
                    ..default()
                }),
                ..default()
            });
        });
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerConfig {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
    pub walk_velocity: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            back: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
            walk_velocity: 1.0,
        }
    }
}

fn control_player(
    mut query: Query<&mut LinearVelocity, With<Player>>,
    input: Res<Input<KeyCode>>,
    player_config: Res<PlayerConfig>,
) {
    for mut lin_vel in query.iter_mut() {
        if input.just_pressed(player_config.jump) {
            lin_vel.0 += vec3(0.0, 9.8, 0.0);
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if input.pressed(player_config.forward) {
            axis_input.z += 1.0;
        }
        if input.pressed(player_config.back) {
            axis_input.z -= 1.0;
        }
        if input.pressed(player_config.right) {
            axis_input.x += 1.0;
        }
        if input.pressed(player_config.left) {
            axis_input.x -= 1.0;
        }

        if axis_input != Vec3::ZERO {
            lin_vel.0 += axis_input.normalize() * player_config.walk_velocity;
        }

        lin_vel
            .0
            .clamp(Vec3::ZERO, Vec3::splat(player_config.walk_velocity));
    }
}
