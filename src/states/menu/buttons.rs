use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum MenuButton {
    Play,
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

impl Into<String> for MenuButton {
    fn into(self) -> String {
        match self {
            MenuButton::Play => "Play".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Exit => "Exit".to_string(),
        }
    }
}

impl MenuButton {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            MenuButton::Play,
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Exit,
        ]
        .into_iter()
    }
}