use crate::GameState;
use bevy::{math::vec3, prelude::*};
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

use super::{space_kit::*, Pellet, RoomConfig};

pub struct ReactorPlugin;

impl Plugin for ReactorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_reactor.run_in_state(GameState::Playing))
            .add_system(pellet_gravity_system.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
pub struct Reactor;

fn spawn_reactor(mut commands: Commands, query: Query<Entity, Added<Reactor>>) {
    for e in query.iter() {
        commands
            .entity(e)
            .insert(Name::new("Reactor"))
            .insert(SpaceKit::Rocket(Rocket::BaseB));
    }
}

pub fn pellet_gravity_system(
    mut query: Query<(&mut LinearVelocity, &Mass, &InverseMass, &Transform), With<Pellet>>,
    room_config: Res<RoomConfig>,
    physics_config: Res<PhysicsConfig>,
) {
    for (mut linear_velocity, mass, inv_mass, trans) in query.iter_mut() {
        let distance_from_center = vec3(0.0, trans.translation.y, trans.translation.z)
            - vec3(0.0, 2.0, room_config.reactor_center_z);
        let impulse = -distance_from_center * mass.0 * physics_config.time;
        linear_velocity.0 += impulse * inv_mass.0;
    }
}
