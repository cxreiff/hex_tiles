use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_mod_picking::prelude::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 15.0)
                .looking_at(Vec3::new(0.0, 1.2, 0.0), Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 4.,
                scaling_mode: ScalingMode::FixedVertical(2.),
                ..default()
            }),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
}
