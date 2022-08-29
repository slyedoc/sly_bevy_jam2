use crate::GameState;
use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::prelude::*;
use bevy_mod_outline::{Outline, OutlineMeshExt};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

pub struct PelletPlugin;

impl Plugin for PelletPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PelletConfig>()
            //.add_system(spawn_pellet.run_in_state(GameState::Playing))
            .add_system_to_stage(
                CoreStage::Last,
                update_pellet.run_in_state(GameState::Playing),
            )
            .add_system_to_stage(CoreStage::First, clear_hit.run_in_state(GameState::Playing))
            .register_inspectable::<Pellet>();
    }
}

fn clear_hit(mut query: Query<&mut Pellet>) {
    for mut pellet in query.iter_mut() {
        pellet.hit = false;
    }
}

fn update_pellet(
    mut query: Query<(
        &Pellet,
        &mut Outline,
        &mut LinearVelocity,
        &Handle<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pellet_config: Res<PelletConfig>,
) {
    for (pellet, mut outline, mut lin_vel, material_handle) in query.iter_mut() {
        let mat = materials.get_mut(material_handle).unwrap();
        mat.base_color = pellet.color();

        if (pellet.value - 0.5).abs() < pellet_config.allow_range {
            outline.colour = Color::GREEN;
        } else {
            outline.colour = Color::RED;
        };

        // speed up pellet if it's to slow
        if lin_vel.0.x.abs() < 1.0 {
            if lin_vel.0.x.is_sign_positive() {
                lin_vel.0.x += 0.01;
            } else {
                lin_vel.0.x -= 0.01;
            }
        }
    }
}
pub struct PelletConfig {
    pub radius: f32,
    pub allow_range: f32,
    pub mesh: Handle<Mesh>,
    pub collider: Collider,
}

impl FromWorld for PelletConfig {
    fn from_world(world: &mut World) -> Self {
        let radius = 0.2;
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mut mesh = Mesh::from(shape::UVSphere {
            radius,
            sectors: 16,
            stacks: 16,
        });
        mesh.generate_outline_normals().unwrap();
        let mesh_handle = meshes.add(mesh);

        let mut collider_resources = world.get_resource_mut::<ColliderResources>().unwrap();
        let collider = collider_resources.add_sphere(radius);

        Self {
            radius,
            allow_range: 0.1,
            mesh: mesh_handle,
            collider,
        }
    }
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Pellet {
    pub hit: bool,
    pub value: f32, // 0-1
}

impl Default for Pellet {
    fn default() -> Self {
        Self {
            hit: false,
            value: 0.0,
        }
    }
}

impl Pellet {
    // lerp between blue and yellow based on value
    pub fn color(&self) -> Color {
        let blue = vec3(0.0, 0.0, 1.0);
        let yellow = vec3(1.0, 1.0, 0.0);
        let green = vec3(0.0, 1.0, 0.0);

        let result = if self.value < 0.5 {
            blue.lerp(green, self.value * 2.0)
        } else {
            green.lerp(yellow, (self.value - 0.5) * 2.0)
        };

        Color::rgb(result.x, result.y, result.z)
    }
}

// fn spawn_pellet(
//     mut commands: Commands,
//     query: Query<(Entity, &Pellet)>,
//     pellet_config: Res<PelletConfig>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     cursor_config: Res<CursorConfig>,
// ) {
//     for (e, p) in query.iter() {
//         commands
//             .entity(e)
//             .insert(config.mesh.clone())
//             .insert(materials.add(StandardMaterial {
//                 base_color: p.color(),
//                 ..default()
//             }))
//             .insert_bundle(RigidBodyBundle {
//                 collider: config.collider.clone(),
//                 mass: Mass(1.0),
//                 ..default()
//             })
//             .insert_bundle(OutlineBundle {
//                 outline: Outline {
//                     visible: false,
//                     colour: cursor_config.hover,
//                     width: cursor_config.width,
//                 },
//                 ..default()
//             });
//     }
// }
