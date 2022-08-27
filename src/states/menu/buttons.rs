#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{GameState, LevelState};

#[derive(Component, Copy, Clone)]
pub enum MenuButton {
    Intro,
    Play,
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

impl Into<String> for MenuButton {
    fn into(self) -> String {
        match self {
            MenuButton::Intro => "Intro (if first time)".to_string(),
            MenuButton::Play => "Play".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Exit => "Exit".to_string(),
        }
    }
}

impl MenuButton {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            MenuButton::Intro,
            MenuButton::Play,
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Exit,
        ]
        .into_iter()
    }
}

pub fn button_click(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    #[cfg(not(target_arch = "wasm32"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match btn {
                MenuButton::Intro => {
                    commands.insert_resource(NextState(GameState::Playing));
                    commands.insert_resource(NextState(LevelState::Intro));
                }
                MenuButton::Play => {
                    commands.insert_resource(NextState(GameState::Playing));
                    commands.insert_resource(NextState(LevelState::One));
                }
                #[cfg(not(target_arch = "wasm32"))]
                MenuButton::Exit => app_exit.send(AppExit),
            }
        }
    }
}
