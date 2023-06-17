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
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-2.0, 15.0, 0.0),
        point_light: PointLight {
            intensity: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-4.0, 10.0, 12.0)
                .looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 5.8,
                scaling_mode: ScalingMode::FixedVertical(2.),
                ..default()
            }),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
}
