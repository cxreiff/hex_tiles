use bevy::prelude::*;

mod camera_plugin;
mod config_plugin;
mod kitty_plugin;
mod loading_plugin;

use camera_plugin::CameraPlugin;
use config_plugin::ConfigPlugin;
use kitty_plugin::KittyPlugin;
use loading_plugin::LoadingPlugin;

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
            .add_plugin(KittyPlugin);
    }
}
