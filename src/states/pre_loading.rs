use crate::{assets::*, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct PreLoadingPlugin;

// load just ui assets, so we can use it on the loading screen
impl Plugin for PreLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::PreLoading)
                .with_collection::<FontAssets>()
                .continue_to_state(GameState::Loading),
        );
    }
}
