use bevy::prelude::*;
use bevy::gltf::Gltf;
use bevy_asset_loader::prelude::*;

use crate::GameState;

#[derive(AssetCollection, Resource)]
pub struct LoadedAssets {
    #[asset(key = "meshes.hexagon")]
    pub hexagon: Handle<Gltf>,
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
            )
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::Loading,
                "manifest.assets.ron",
            )
            .add_collection_to_loading_state::<_, LoadedAssets>(GameState::Loading);
    }
}
