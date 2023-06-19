use bevy::prelude::*;

mod camera_plugin;
mod config_plugin;
mod loading_plugin;
mod world_plugin;
mod ui_plugin;

use camera_plugin::CameraPlugin;
use config_plugin::ConfigPlugin;
use loading_plugin::LoadingPlugin;
use world_plugin::WorldPlugin;
use ui_plugin::UiPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(UiPlugin);
    }
}
