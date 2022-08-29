use std::time::Duration;

use crate::assets::*;
use crate::{cleanup, GameState};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct PlayingPlugin;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component, Default)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct HighScoreText;

#[derive(Component, Default)]
pub struct HighScore(pub u32);

#[derive(Component)]
pub struct TimeText;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_enter_system(GameState::Playing, setup)
            .add_system(hotkeys.run_in_state(GameState::Playing))
            .add_system(click_button.run_in_state(GameState::Playing))
            .add_system(update_score_text.run_in_state(GameState::Playing))
            .add_system(update_high_score_text.run_in_state(GameState::Playing))
            .add_system(update_time_text.run_in_state(GameState::Playing))
            .add_exit_system(GameState::Playing, cleanup);
    }
}

fn update_score_text(mut score_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in score_query.iter_mut() {
        text.sections[1].value = score.0.to_string();
    }
}

fn update_high_score_text(
    mut high_score_query: Query<&mut Text, With<HighScoreText>>,
    high_score: Res<HighScore>,
) {
    for mut text in high_score_query.iter_mut() {
        text.sections[1].value = high_score.0.to_string();
    }
}

fn update_time_text(mut time_query: Query<&mut Text, With<TimeText>>, game_time: Res<GameTimer>) {
    for mut text in time_query.iter_mut() {
        let seconds = game_time.0.duration().as_secs_f32() - game_time.0.elapsed().as_secs_f32();

        text.sections[1].value = format!("{:.0}", seconds);
        text.sections[1].style.color = match seconds as u32 {
            0..=10 => Color::RED,
            _ => Color::GOLD,
        };
    }
}

#[derive(Component, Debug, Copy, Clone)]
enum PlayingButton {
    Exit,
}

impl From<PlayingButton> for String {
    fn from(b: PlayingButton) -> Self {
        match b {
            PlayingButton::Exit => "Esc".to_string(),
        }
    }
}

fn setup(mut commands: Commands, font_assets: Res<FontAssets>, button_colors: Res<ButtonColors>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                position: UiRect::<Val> {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
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
                style: Style {
                    padding: UiRect::<Val> {
                        top: Val::Px(2.0),
                        left: Val::Px(2.0),
                        right: Val::Px(2.0),
                        bottom: Val::Px(2.0),
                    },
                    ..default()
                },
                text: Text {
                    sections: vec![font_assets
                        .sub_title(PlayingButton::Exit.into(), Color::rgb(0.9, 0.9, 0.9))],
                    alignment: Default::default(),
                },
                ..default()
            });
        })
        .insert(PlayingButton::Exit)
        .insert(Name::new("ui Exit Button"));

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    left: Val::Px(10.0),
                    bottom: Val::Px(25.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![font_assets.sub_title("LAlt to unlock cursor".into(), Color::WHITE)],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui Alt helper"));

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    font_assets.sub_title("LAlt + S to skip intro dialog".into(), Color::WHITE)
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui Skip helper"));

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    top: Val::Px(10.0),
                    right: Val::Px(40.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    font_assets.h1("Score: ".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::GOLD),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui Score"))
        .insert(ScoreText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    top: Val::Px(40.0),
                    right: Val::Px(40.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    font_assets.h1("High Score: ".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::GOLD),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui High Score"))
        .insert(HighScoreText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    top: Val::Px(70.0),
                    right: Val::Px(40.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    font_assets.h1("Time: ".into(), Color::WHITE),
                    font_assets.h1("".into(), Color::GOLD),
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("ui High Score"))
        .insert(TimeText);
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

pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer(Timer::new(Duration::from_secs(0), false))
    }
}
