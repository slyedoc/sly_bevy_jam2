use crate::{assets::*, LevelState};
use crate::{cleanup, GameState};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, setup_buttons)
            .add_enter_system(GameState::Playing, set_level)
            .add_system(hotkeys.run_in_state(GameState::Playing))
            .add_system(click_button.run_in_state(GameState::Playing))
            .add_exit_system(GameState::Playing, cleanup);
    }
}

fn set_level(mut commands: Commands) {
    commands.insert_resource(NextState(LevelState::Intro));
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
                    sections: vec![
                        font_assets.h1(PlayingButton::Exit.into(), Color::rgb(0.9, 0.9, 0.9))
                    ],
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
                PlayingButton::Exit => {
                    commands.insert_resource(NextState(GameState::Menu));
                }
            }
        }
    }
}
