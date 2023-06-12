use bevy::prelude::*;
use bevy::gltf::{Gltf, GltfMesh};

use crate::loading_plugin::LoadedAssets;
use crate::GameState;

pub static HEX_X: f32 = 1.78;
pub static HEX_Y: f32 = 0.5;
pub static HEX_Z: f32 = HEX_X * 1.18;

#[derive(Component)]
struct Hexagon;

#[derive(Resource, Default)]
struct WorldTracker {
    _tiles: Vec<Vec<u32>>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(world_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(world_update.in_set(OnUpdate(GameState::Playing)));
    }
}

fn world_setup(
    mut commands: Commands,
    assets: Res<LoadedAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_meshes: Res<Assets<GltfMesh>>,
    mut assets_materials: ResMut<Assets<StandardMaterial>>,
) {
    #[rustfmt::skip]
    let initial_tiles = vec![
        vec![ 0, 0, 1, 2, 0, 0 ],
        vec![  0, 0, 3, 2, 0   ],
        vec![ 0, 1, 1, 2, 1, 0 ],
        vec![  0, 1, 2, 2, 0   ],
        vec![ 0, 0, 1, 1, 0, 0 ],
    ];

    commands.insert_resource(WorldTracker { _tiles: initial_tiles.clone() });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 5.0),
        point_light: PointLight { intensity: 3000.0, shadows_enabled: true, ..default() },
        ..default()
    });

    if let Some(hexagon) = assets_gltf.get(&assets.hexagon) {
        if let Some(hexagon) = assets_meshes.get(&hexagon.named_meshes["hexagon"]) {
            let hexagon = &hexagon.primitives[0].mesh;
            for (i, tile_row) in initial_tiles.iter().enumerate() {
                for (j, tile) in tile_row.iter().enumerate() {
                    commands.spawn((
                        PbrBundle {
                            transform: hex_grid_transform(i, j, tile),
                            mesh: hexagon.clone(),
                            material: assets_materials.add(Color::rgb(0.3, 0.5, 0.4).into()),
                            ..default()
                        },
                    ));
                }
            }
        };
    }
}

fn world_update(
    time: Res<Time>,
    mut hexagons: Query<&mut Transform, With<Hexagon>>,
) {
    for mut hexagon in hexagons.iter_mut() {
        hexagon.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.))
    }
}

fn hex_grid_transform(i: usize, j: usize, height: &u32) -> Transform {
    let x = j as f32 * HEX_X + (i as f32 % 2.) * (HEX_X / 2.);
    let y = *height as f32 * HEX_Y / 2.0;
    let z = i as f32 * HEX_Z * 0.75;
    let scale = Vec3::new(1.0, *height as f32 * 0.25, 1.0);
    Transform::from_xyz(x, y, z).with_scale(scale)
}

