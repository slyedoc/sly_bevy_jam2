use std::time::Duration;

use crate::{
    cursor::{CursorConfig, CursorInteraction, InteractionTime},
    GameState,
};
use bevy::{math::vec3, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_mod_outline::*;
use iyes_loopless::prelude::*;
use sly_physics::prelude::*;

pub struct SwitchPlugin;

pub enum SwitchState {
    Enabled,
    Disabled,
}

#[derive(Component)]
pub struct Switch {
    pub target: Entity,
    pub state: SwitchState,
}

pub struct SwitchEvent(pub Entity);

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SwitchConfig>()
            .add_event::<SwitchEvent>()
            .add_audio_channel::<SwitchAudioChannel>()
            .add_system(spawn_switch)
            .add_system_to_stage(CoreStage::PostUpdate, interaction_check.run_in_state(GameState::Playing));
    }
}

pub struct SwitchAudioChannel;

#[derive(AssetCollection)]
pub struct SwitchAudioAssets {
    #[asset(path = "audio/switch11.ogg")]
    pub flip: Handle<AudioSource>,
}

pub struct SwitchConfig {
    size: Vec3,
    button_size: Vec3,
    boarder_mesh: Handle<Mesh>,
    button_mesh: Handle<Mesh>,

    boarder_mat: Handle<StandardMaterial>,
    on_mat: Handle<StandardMaterial>,
    off_mat: Handle<StandardMaterial>,
    collider: Collider,
}

impl FromWorld for SwitchConfig {
    fn from_world(world: &mut World) -> Self {
        let size = vec3(0.2, 0.3, 0.2);
        let button_size = vec3(0.1, 0.1, 0.1);

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mut base = Mesh::from(shape::Box::new(size.x, size.y, size.z));
        base.generate_outline_normals().unwrap();
        let boarder_mesh = meshes.add(base);

        let button_mesh = meshes.add(Mesh::from(shape::Box::new(
            button_size.x,
            button_size.y,
            button_size.z,
        )));

        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let boarder_mat = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        });
        let on_mat = materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        });
        let off_mat = materials.add(StandardMaterial {
            base_color: Color::RED,
            ..default()
        });

        let mut collider_resources = world.get_resource_mut::<ColliderResources>().unwrap();
        let collider = collider_resources.add_box(size);

        Self {
            size,
            button_size,
            boarder_mesh,
            button_mesh,
            boarder_mat,
            on_mat,
            off_mat,
            collider,
        }
    }
}

pub fn spawn_switch(
    mut commands: Commands,
    query: Query<Entity, Added<Switch>>,
    config: Res<SwitchConfig>,
    cursor_config: Res<CursorConfig>,
) {
    for e in query.iter() {
        // add switch boarder
        commands
            .entity(e)
            .insert(Name::new("Switch"))
            .insert(config.boarder_mesh.clone())
            .insert(config.boarder_mat.clone())
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .insert_bundle(OutlineBundle {
                outline: Outline {
                    visible: false,
                    colour: cursor_config.hover,
                    width: cursor_config.width,
                },
                ..default()
            })
            .insert(CursorInteraction::None)
            .insert(InteractionTime::default())
            .insert_bundle(RigidBodyBundle {
                mode: RigidBodyMode::Static,
                collider: config.collider.clone(),
                ..default()
            })
            .with_children(|parent| {
                let offset = 0.06;
                parent.spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(0.0, offset, 0.1),
                    mesh: config.button_mesh.clone(),
                    material: config.on_mat.clone(),
                    ..default()
                });
                parent.spawn_bundle(PbrBundle {
                    transform: Transform::from_xyz(0.0, -offset, 0.1),
                    mesh: config.button_mesh.clone(),
                    material: config.off_mat.clone(),
                    ..default()
                });
            });
    }
}

fn interaction_check(
    mut query: Query<(
        &Switch,
        &CursorInteraction,
        &mut InteractionTime,
        &mut Outline,
    )>,
    audio_assets: Res<SwitchAudioAssets>,
    channel: Res<AudioChannel<SwitchAudioChannel>>,
    mut switch_events: EventWriter<SwitchEvent>,
    cursor_config: Res<CursorConfig>,
) {
    for (switch, cursor_interaction, mut interaction_time, mut outline) in query.iter_mut() {
        match cursor_interaction {
            CursorInteraction::Clicked => {
                match switch.state {
                    SwitchState::Enabled => {
                        // Play sound
                        let handle = audio_assets.flip.clone();
                        channel.play(handle).with_volume(0.4);

                        // Set interaction timer
                        interaction_time
                            .timer
                            .set_duration(Duration::from_secs_f32(1.0));
                        interaction_time.timer.reset();

                        // send event to target
                        switch_events.send(SwitchEvent(switch.target));
                    }
                    SwitchState::Disabled => {},
                }
            }
            CursorInteraction::Hovered => {
                outline.visible = true;
                outline.colour = match switch.state {
                    SwitchState::Enabled => cursor_config.hover,
                    SwitchState::Disabled => cursor_config.disabled,
                };
            }
            CursorInteraction::None => {
                outline.visible = false;
            }
        }
    }
}
