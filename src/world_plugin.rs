use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::HashMap;
use bevy_mod_picking::prelude::*;
use derive_more::Constructor;
use hexx::*;

use crate::GameState;

pub static MAP_RADIUS: u32 = 4;
pub static MARGIN: f32 = 0.06;

#[derive(Clone, Constructor)]
struct HexCoords {
    hex: Hex,
    layer: u32,
}

#[derive(Component)]
struct Selector;

#[derive(Resource, Default)]
struct WorldTracker {
    layout: HexLayout,
    tiles: HashMap<Entity, HexCoords>,
    mesh_handle: Handle<Mesh>,
    tile_material_handle: Handle<StandardMaterial>,
    hidden_material_handle: Handle<StandardMaterial>,
    selector_material_handle: Handle<StandardMaterial>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(world_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_event::<HexEvent>()
            .add_system(handle_hex_event.run_if(on_event::<HexEvent>()));
    }
}

fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        orientation: HexOrientation::pointy(),
        ..default()
    };

    let mesh_handle = meshes.add(compute_mesh(ColumnMeshBuilder::new(&layout, 0.5).build()));
    let tile_material_handle = materials.add(Color::rgb(0.66, 0.53, 0.66).into());
    let hidden_material_handle = materials.add(Color::RED.with_a(0.0).into());
    let selector_material_handle = materials.add(Color::rgb(0.66, 0.53, 0.66).with_a(0.3).into());
    let empty_tile_material_handle = materials.add(Color::GRAY.with_a(0.5).into());

    let tiles =
        shapes::hexagon(Hex::ZERO, MAP_RADIUS)
            .map(|hex| {
                let position = layout.hex_to_world_pos(hex);
                commands.spawn(PbrBundle {
                    transform: Transform::from_xyz(position.x, 0.0, position.y)
                        .with_scale(Vec3::new(1.0 - MARGIN, 0.1, 1.0 - MARGIN)),
                    mesh: mesh_handle.clone(),
                    material: empty_tile_material_handle.clone(),
                    ..default()
                });
                let entity = commands
                    .spawn((
                        PbrBundle {
                            transform: Transform::from_xyz(position.x, 0.0, position.y)
                                .with_scale(Vec3::new(1.0 - MARGIN, 1.0 - MARGIN, 1.0 - MARGIN)),
                            mesh: mesh_handle.clone(),
                            material: hidden_material_handle.clone(),
                            ..default()
                        },
                        NotShadowCaster,
                        OnPointer::<Over>::send_event::<HexEvent>(),
                        OnPointer::<Out>::send_event::<HexEvent>(),
                        OnPointer::<Down>::send_event::<HexEvent>(),
                        Selector,
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
                        ));
                    })
                    .id();
                (entity, HexCoords::new(hex, 0))
            })
            .collect();

    commands.insert_resource(WorldTracker {
        layout,
        tiles,
        mesh_handle,
        tile_material_handle,
        hidden_material_handle,
        selector_material_handle,
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

enum HexEvent {
    Over(ListenedEvent<Over>),
    Out(ListenedEvent<Out>),
    Down(ListenedEvent<Down>),
}

impl From<ListenedEvent<Over>> for HexEvent {
    fn from(event: ListenedEvent<Over>) -> Self {
        HexEvent::Over(event)
    }
}
impl From<ListenedEvent<Out>> for HexEvent {
    fn from(event: ListenedEvent<Out>) -> Self {
        HexEvent::Out(event)
    }
}
impl From<ListenedEvent<Down>> for HexEvent {
    fn from(event: ListenedEvent<Down>) -> Self {
        HexEvent::Down(event)
    }
}

fn handle_hex_event(
    mut commands: Commands,
    mut events: EventReader<HexEvent>,
    mut tracker: ResMut<WorldTracker>,
    mut q_transforms: Query<&mut Transform, With<Selector>>,
) {
    for event in events.iter() {
        match event {
            HexEvent::Over(event) => {
                commands
                    .entity(event.listener)
                    .insert(tracker.selector_material_handle.clone());
            }
            HexEvent::Out(event) => {
                commands
                    .entity(event.listener)
                    .insert(tracker.hidden_material_handle.clone());
            }
            HexEvent::Down(event) => {
                let mut transform = q_transforms.get_mut(event.listener).unwrap();
                let hex_coords = tracker.tiles.get(&event.listener).unwrap().clone();
                let position = tracker.layout.hex_to_world_pos(hex_coords.hex);
                let entity = commands
                    .spawn(PbrBundle {
                        transform: Transform::from_xyz(
                            position.x,
                            (hex_coords.layer as f32) * 0.5,
                            position.y,
                        )
                        .with_scale(Vec3::new(
                            1.0 - MARGIN,
                            1.0 - MARGIN,
                            1.0 - MARGIN,
                        )),
                        mesh: tracker.mesh_handle.clone(),
                        material: tracker.tile_material_handle.clone(),
                        ..default()
                    })
                    .id();
                transform.translation.y += 0.5;
                tracker.tiles.insert(entity, hex_coords.clone());

                let hex_coords = tracker.tiles.get_mut(&event.listener).unwrap();
                hex_coords.layer += 1;
            }
        }
    }
}
