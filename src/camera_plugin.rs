use bevy::prelude::*;

use crate::world_plugin::HEX_X;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(HEX_X * 2.5, 10.0, 15.0)
            .looking_at(Vec3::new(HEX_X * 2.5, -0.5, 2.0), Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            ..default()
        }),
        ..default()
    });
}
