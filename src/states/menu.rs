mod buttons;
mod audio;

use buttons::*;
use audio::*;

use crate::assets::*;
use crate::cleanup;
use crate::GameState;

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_kira_audio::*;
use iyes_loopless::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_audio_channel::<MenuBackgroundAudio>()
            .add_enter_system(GameState::Menu, setup_menu)
            .add_enter_system(GameState::Menu, start_audio)
            .add_system(click_button.run_in_state(GameState::Menu))
            .add_exit_system(GameState::Menu, stop_audio)
            .add_exit_system(GameState::Menu, cleanup);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_system(exit_window.run_in_state(GameState::Menu));
    }
}


fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(40.0),
                    left: Val::Percent(30.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(40.0), Val::Percent(40.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: CLEAR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            for b in MenuButton::iter() {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        color: button_colors.normal,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![font_assets.h1(b.into(), Color::rgb(0.9, 0.9, 0.9))],
                                alignment: Default::default(),
                            },
                            ..Default::default()
                        });
                    })
                    .insert(b);
            }
        });
}

#[allow(clippy::type_complexity)]
fn click_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    #[cfg(not(target_arch = "wasm32"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match btn {
                MenuButton::Play => commands.insert_resource(NextState(GameState::Playing)),
                #[cfg(not(target_arch = "wasm32"))]
                MenuButton::Exit => app_exit.send(AppExit),
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn exit_window(input: Res<Input<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit);
    }
}
