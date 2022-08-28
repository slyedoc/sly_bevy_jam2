
use bevy::{
    prelude::*,
};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;
use crate::GameState;


pub struct PelletPlugin;

impl Plugin for PelletPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PelletConfig>()
        .add_system(spawn_pellet.run_in_state(GameState::Playing))
        .add_system(update_pellet_color.run_in_state(GameState::Playing));
    }
}

fn update_pellet_color(
    query: Query<(&Pellet, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ( pellet, material_handle) in query.iter() {
        let mat = materials.get_mut(material_handle).unwrap();
        mat.base_color = pellet.color(); 
    }
}
pub struct PelletConfig {
    radius: f32,
    mesh: Handle<Mesh>,
    collider: Collider,
}

impl FromWorld for PelletConfig {
    fn from_world(world: &mut World) -> Self {
        let radius = 0.2;
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Mesh::from(shape::UVSphere {
            radius: radius,
            sectors: 8,
            stacks: 8,
        }));

        let mut collider_resources = world.get_resource_mut::<ColliderResources>().unwrap();
        let collider = collider_resources.add_sphere(radius);
        
        Self {
            radius,
            mesh,
            collider,
        }
    }
}

#[derive(Component)]
pub struct Pellet {
    pub value: f32, // 0-1
}

impl Pellet {
    pub fn color(&self) -> Color {
        let value = self.value;
        let r = (1.0 - value) * 0.5;
        let g = value * 0.5;
        let b = 0.0;
        Color::rgb(r, g, b)
    }
}

fn spawn_pellet(
    mut commands: Commands,
    query: Query<(Entity, &Pellet)>,
    config: Res<PelletConfig>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (e, p) in query.iter() {        
        commands.entity(e)
        .insert(config.mesh.clone())
        .insert(materials.add(StandardMaterial {
            base_color: p.color(),
            ..default()
        }))
        .insert_bundle(RigidBodyBundle {
            collider: config.collider.clone(),
            mass: Mass(1.0),
            ..default()
        });

    }
}