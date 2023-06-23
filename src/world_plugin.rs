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
pub static MARGIN: f32 = 0.05;

#[derive(Clone, Constructor, Default)]
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
    last_hex: Hex,
    drag_layer: u32,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(world_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_event::<HexEvent>()
            .add_systems(
                (handle_hex_event_update_parent, handle_hex_event_spawn_tile)
                    .chain()
                    .in_set(OnUpdate(GameState::Playing))
                    .distributive_run_if(on_event::<HexEvent>()),
            );
    }
}

fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let mesh_handle = meshes.add(compute_mesh(ColumnMeshBuilder::new(&layout, 0.5).build()));
    let tile_material_handle = materials.add(Color::rgb(0.66, 0.53, 0.66).into());
    let empty_tile_material_handle = materials.add(Color::GRAY.with_a(0.5).into());
    let hidden_material_handle = materials.add(Color::RED.with_a(0.0).into());
    let selector_material_handle = materials.add(Color::rgb(0.66, 0.53, 0.66).with_a(0.3).into());

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
                        OnPointer::<Up>::send_event::<HexEvent>(),
                        PickHighlight,
                        Highlight {
                            hovered: Some(HighlightKind::Fixed(selector_material_handle.clone())),
                            pressed: Some(HighlightKind::Fixed(hidden_material_handle.clone())),
                        },
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
                            Interaction::None,
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

enum HexEvent {
    Over(ListenedEvent<Over>),
    Out(ListenedEvent<Out>),
    Down(ListenedEvent<Down>),
    Up(ListenedEvent<Up>),
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
impl From<ListenedEvent<Up>> for HexEvent {
    fn from(event: ListenedEvent<Up>) -> Self {
        HexEvent::Up(event)
    }
}

fn handle_hex_event_update_parent(
    mut commands: Commands,
    mut events: EventReader<HexEvent>,
    mouse: Res<Input<MouseButton>>,
    q_interaction: Query<&Interaction>,
) {
    fn update_parent_interaction<T: IsPointerEvent>(
        commands: &mut Commands,
        q_interaction: &Query<&Interaction>,
        event: &ListenedEvent<T>,
    ) {
        if let Ok(interaction) = q_interaction.get(event.target) {
            commands.entity(event.listener).insert(*interaction);
        }
    }

    for event in events.iter() {
        match event {
            HexEvent::Over(event) => {
                if !mouse.pressed(MouseButton::Left) {
                    update_parent_interaction(&mut commands, &q_interaction, event);
                }
            }
            HexEvent::Out(event) => {
                if !mouse.pressed(MouseButton::Left) {
                    update_parent_interaction(&mut commands, &q_interaction, event);
                }
            }
            HexEvent::Down(event) => {
                update_parent_interaction(&mut commands, &q_interaction, event);
            }
            HexEvent::Up(event) => {
                update_parent_interaction(&mut commands, &q_interaction, event);
            }
        }
    }
}

fn handle_hex_event_spawn_tile(
    mut commands: Commands,
    mut events: EventReader<HexEvent>,
    mut tracker: ResMut<WorldTracker>,
    mouse: Res<Input<MouseButton>>,
    mut q_transforms: Query<&mut Transform, With<Selector>>,
) {
    fn spawn_tile<T: IsPointerEvent>(
        commands: &mut Commands,
        tracker: &mut ResMut<WorldTracker>,
        q_transforms: &mut Query<&mut Transform, With<Selector>>,
        event: &ListenedEvent<T>,
    ) {
        commands
            .entity(event.listener)
            .insert(tracker.hidden_material_handle.clone());
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
                .with_scale(Vec3::new(1.0 - MARGIN, 1.0 - MARGIN, 1.0 - MARGIN)),
                mesh: tracker.mesh_handle.clone(),
                material: tracker.tile_material_handle.clone(),
                ..default()
            })
            .id();
        transform.translation.y += 0.5;
        tracker.last_hex = hex_coords.hex;
        tracker.tiles.insert(entity, hex_coords);

        let hex_coords = tracker.tiles.get_mut(&event.listener).unwrap();
        hex_coords.layer += 1;
    }

    for event in events.iter() {
        match event {
            HexEvent::Down(event) => {
                if let Some(HexCoords { layer, .. }) = tracker.tiles.get(&event.listener) {
                    tracker.drag_layer = *layer;
                    spawn_tile(&mut commands, &mut tracker, &mut q_transforms, event);
                }
            }
            HexEvent::Over(event) => {
                if let Some(HexCoords { hex, layer }) = tracker.tiles.get(&event.listener) {
                    if mouse.pressed(MouseButton::Left)
                        && *hex != tracker.last_hex
                        && *layer <= tracker.drag_layer
                    {
                        spawn_tile(&mut commands, &mut tracker, &mut q_transforms, event);
                    }
                }
            }
            _ => {}
        }
    }
}
