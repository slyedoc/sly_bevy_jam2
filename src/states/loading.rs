use std::time::Duration;

use crate::{assets::*, cleanup, prefabs::*, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_loopless::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .with_collection::<AudioAssets>()
                    .with_collection::<TextureAssets>()
                    .with_collection::<AIAudioAssets>()
                    .with_collection::<SwitchAudioAssets>()
                    .with_collection::<SpaceKitAssets>()
                    .continue_to_state(GameState::Menu),
            )
            .add_enter_system(GameState::Loading, setup)
            .add_stage_before(
                CoreStage::Update,
                "loading_update",
                FixedTimestepStage::new(Duration::from_secs_f64(0.5))
                    .with_stage(SystemStage::parallel().with_system(update_text)),
            )
            .add_exit_system(GameState::Loading, cleanup);
    }
}

#[derive(Component)]
struct LoadingText;

fn setup(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::BLACK;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    left: Val::Percent(85.0),
                    bottom: Val::Percent(15.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexStart,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    font_assets.h1("Loading".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::WHITE),
                ],
            },
            ..Default::default()
        })
        .insert(Name::new("ui Loading"))
        .insert(LoadingText);
}

fn update_text(mut query: Query<&mut Text, With<LoadingText>>, mut count: Local<usize>) {
    for mut text in query.iter_mut() {
        // Update the value of the second section
        let str = match *count {
            0 => ".",
            1 => "..",
            _ => "...",
        };
        text.sections[1].value = str.to_string();
        *count += 1;
        *count %= 3;
    }
}
