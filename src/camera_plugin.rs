use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::config_plugin::WIDTH;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedHorizontal(WIDTH),
            ..default()
        },
        ..default()
    });
}
