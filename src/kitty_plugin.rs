use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

use crate::config_plugin::get_world_position;
use crate::loading_plugin::LoadedAssets;
use crate::GameState;

#[derive(Component)]
struct KittyFlag;

#[derive(Bundle)]
struct KittyBundle {
    _kitty_flag: KittyFlag,
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct KittySpawnerTracker {
    timer: Timer,
    count: u8,
}

pub struct KittyPlugin;

impl Plugin for KittyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(kitty_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((kitty_spawner, kitty_mover).in_set(OnUpdate(GameState::Playing)));
    }
}

fn kitty_setup(mut commands: Commands, textures: Res<LoadedAssets>) {
    commands.insert_resource(KittySpawnerTracker {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        count: 0,
    });
    commands.spawn(KittyBundle {
        _kitty_flag: KittyFlag,
        sprite: SpriteBundle {
            texture: textures.kitty.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(0.8, 0.8, 0.),
                ..default()
            },
            ..default()
        },
    });
}

fn kitty_spawner(mut tracker: ResMut<KittySpawnerTracker>, time: Res<Time>) {
    tracker.timer.tick(time.delta());

    if tracker.timer.finished() && tracker.count < 1 {
        tracker.count += 1;
    }
}

fn kitty_mover(
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&GlobalTransform, With<Camera>>,
    mut q_kitty: Query<&mut Transform, With<KittyFlag>>,
) {
    let window = q_window.single();
    let camera_transform = q_camera.single();

    let world_position = if let Some(position) = window.cursor_position() {
        get_world_position(position, window, camera_transform)
    } else {
        Vec3::new(0., 0., 0.)
    };

    for mut transform in q_kitty.iter_mut() {
        let diff = world_position - transform.translation;
        transform.translation.x += diff.x * time.delta_seconds() * 2.;
        transform.translation.y += diff.y * time.delta_seconds() * 2.;
    }
}
