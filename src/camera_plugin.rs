use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_mod_picking::prelude::*;

static MAX_ANGLE: f32 = PI * 0.5 * 0.99;
static MIN_ANGLE: f32 = 0.05;
static FOCUS: Vec3 = Vec3::new(0.0, 0.8, 0.0);

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup)
            .add_system(camera_control);
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
            transform: Transform::from_xyz(0.0, 10.0, 12.0).looking_at(FOCUS, Vec3::Y),
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 6.0,
                scaling_mode: ScalingMode::FixedVertical(2.),
                ..default()
            }),
            ..default()
        },
        RaycastPickCamera::default(),
    ));
}

fn camera_control(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut camera_transform = camera.single_mut();
    if keys.pressed(KeyCode::Left) {
        camera_transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(-time.delta_seconds() * 0.8),
        );
    }
    if keys.pressed(KeyCode::Right) {
        camera_transform.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(time.delta_seconds() * 0.8),
        );
    }
    if keys.pressed(KeyCode::Up) {
        let camera_direction = (camera_transform.translation - FOCUS).normalize();
        let theta = camera_direction.y.asin();
        if theta < MAX_ANGLE {
            let rotation_axis_x = -camera_direction.z;
            let rotation_axis_z = camera_direction.x;
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(
                    Vec3::new(rotation_axis_x, 0.0, rotation_axis_z).normalize(),
                    time.delta_seconds() * 0.5,
                ),
            );
        }
    }
    if keys.pressed(KeyCode::Down) {
        let camera_direction = (camera_transform.translation - FOCUS).normalize();
        let theta = camera_direction.y.asin();
        if theta > MIN_ANGLE {
            let rotation_axis_x = -camera_direction.z;
            let rotation_axis_z = camera_direction.x;
            camera_transform.rotate_around(
                Vec3::ZERO,
                Quat::from_axis_angle(
                    Vec3::new(rotation_axis_x, 0.0, rotation_axis_z).normalize(),
                    -time.delta_seconds() * 0.5,
                ),
            );
        }
    }
}
