use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::utils::HashMap;
use bevy::{pbr::NotShadowCaster, render::render_resource::PrimitiveTopology};
use bevy_mod_picking::prelude::*;
use derive_more::Constructor;
use hexx::*;

use super::{tile_events::TileEvent, TileVariant, GRID_MARGIN, GRID_RADIUS};

#[derive(Component)]
pub struct TileSelector;

#[derive(Clone, Constructor, Default)]
pub struct HexCoords {
    pub hex: Hex,
    pub layer: u32,
}

#[derive(Resource, Default)]
pub struct GridTracker {
    pub current_tile_variant: TileVariant,
    pub layout: HexLayout,
    pub tiles: HashMap<Entity, HexCoords>,
    pub tile_materials: HashMap<TileVariant, Handle<StandardMaterial>>,
    pub hidden_material_handle: Handle<StandardMaterial>,
    pub mesh_handle: Handle<Mesh>,
    pub last_hex: Hex,
    pub drag_layer: u32,
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let mesh_handle = meshes.add(compute_mesh(ColumnMeshBuilder::new(&layout, 0.5).build()));
    let empty_tile_material_handle = materials.add(Color::GRAY.with_a(0.5).into());
    let hidden_material_handle = materials.add(Color::RED.with_a(0.0).into());
    let selector_material_handle = materials.add(Color::rgb(0.66, 0.66, 0.66).with_a(0.3).into());

    let tile_materials = TileVariant::initialize_materials(&mut materials);

    let tiles =
        shapes::hexagon(Hex::ZERO, GRID_RADIUS)
            .map(|hex| {
                let position = layout.hex_to_world_pos(hex);
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(position.x, 0.0, position.y)
                        .with_scale(Vec3::new(1.0 - GRID_MARGIN, 0.1, 1.0 - GRID_MARGIN)),
                    mesh: mesh_handle.clone(),
                    material: empty_tile_material_handle.clone(),
                    ..default()
                });
                let entity = commands
                    .spawn((
                        PbrBundle {
                            transform: Transform::from_xyz(position.x, 0.0, position.y).with_scale(
                                Vec3::new(1.0 - GRID_MARGIN, 1.0 - GRID_MARGIN, 1.0 - GRID_MARGIN),
                            ),
                            mesh: mesh_handle.clone(),
                            material: hidden_material_handle.clone(),
                            ..default()
                        },
                        NotShadowCaster,
                        OnPointer::<Over>::send_event::<TileEvent>(),
                        OnPointer::<Out>::send_event::<TileEvent>(),
                        OnPointer::<Down>::send_event::<TileEvent>(),
                        OnPointer::<Up>::send_event::<TileEvent>(),
                        PickHighlight,
                        Highlight {
                            hovered: Some(HighlightKind::Fixed(selector_material_handle.clone())),
                            pressed: Some(HighlightKind::Fixed(hidden_material_handle.clone())),
                        },
                        TileSelector,
                    ))
                    .with_children(|commands| {
                        commands.spawn((
                            PbrBundle {
                                transform: Transform {
                                    scale: Vec3::new(1.0, 0.01, 1.0),
                                    ..default()
                                },
                                mesh: mesh_handle.clone(),
                                material: hidden_material_handle.clone(),
                                ..default()
                            },
                            RaycastPickTarget::default(),
                            Interaction::None,
                        ));
                    })
                    .id();
                (entity, HexCoords::new(hex, 0))
            })
            .collect();

    commands.insert_resource(GridTracker {
        current_tile_variant: TileVariant::Purple,
        layout,
        tiles,
        tile_materials,
        hidden_material_handle,
        mesh_handle,
        last_hex: Hex::ZERO,
        drag_layer: 0,
    });
}

fn compute_mesh(mesh_info: MeshInfo) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}
