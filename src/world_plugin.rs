use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_mod_picking::prelude::*;
use hexx::*;

use crate::GameState;

pub static MARGIN: f32 = 0.05;

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
            .add_system(world_update.in_set(OnUpdate(GameState::Playing)))
            .add_event::<HexEvent>()
            .add_system(handle_hex_event.run_if(on_event::<HexEvent>()));
    }
}

fn world_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    #[rustfmt::skip]
    let initial_tiles = vec![
        vec![ 0, 0, 1, 2, 0, 0 ],
        vec![  0, 0, 3, 2, 0   ],
        vec![ 0, 1, 1, 2, 1, 0 ],
        vec![  0, 1, 2, 2, 0   ],
        vec![ 0, 0, 1, 1, 0, 0 ],
    ];

    commands.insert_resource(WorldTracker {
        _tiles: initial_tiles.clone(),
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-5.0, 12.0, 5.0),
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    let layout = HexLayout::default();
    for (i, tile_row) in initial_tiles.iter().enumerate() {
        for (j, tile) in tile_row.iter().enumerate() {
            let mesh = ColumnMeshBuilder::new(&layout, 1.0).build();
            commands.spawn((
                PbrBundle {
                    transform: hex_grid_transform(i, j, tile),
                    mesh: meshes.add(compute_mesh(mesh)),
                    material: materials.add(Color::CYAN.into()),
                    ..default()
                },
                RaycastPickTarget::default(),
                OnPointer::<Over>::send_event::<HexEvent>(),
                OnPointer::<Out>::send_event::<HexEvent>(),
                OnPointer::<Down>::send_event::<HexEvent>(),
                OnPointer::<Up>::send_event::<HexEvent>(),
                OnPointer::<Drag>::send_event::<HexEvent>(),
            ));
        }
    }
}

fn world_update(time: Res<Time>, mut hexagons: Query<&mut Transform, With<Hexagon>>) {
    for mut hexagon in hexagons.iter_mut() {
        hexagon.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds() / 2.))
    }
}

fn hex_grid_transform(i: usize, j: usize, height: &u32) -> Transform {
    let x = (j as f32 + (i as f32 % 2. / 2.)) * (3_f32.sqrt() + MARGIN) - (3_f32.sqrt() * 5. / 2.);
    let y = 0.0;
    let z = i as f32 * (1.5 + MARGIN) - (1.5 * 5. / 2.);
    let scale = Vec3::new(1.0, (*height as f32 / 2.) + 0.01, 1.0);
    Transform::from_xyz(x, y, z)
        .with_scale(scale)
        .with_rotation(Quat::from_rotation_y(PI / 2.))
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
    Drag(ListenedEvent<Drag>),
    Over(ListenedEvent<Over>),
    Out(ListenedEvent<Out>),
    Down(ListenedEvent<Down>),
    Up(ListenedEvent<Up>),
}

impl From<ListenedEvent<Drag>> for HexEvent {
    fn from(event: ListenedEvent<Drag>) -> Self {
        HexEvent::Drag(event)
    }
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

fn handle_hex_event(
    mut events: EventReader<HexEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut transforms: Query<&mut Transform>,
    material_handles: Query<&Handle<StandardMaterial>>,
) {
    for event in events.iter() {
        match event {
            HexEvent::Drag(event) => {
                let mut transform = transforms.get_mut(event.target).unwrap();
                transform.scale.y = (transform.scale.y + event.delta.y / 100.).clamp(0.01, 3.0);
                let material = materials
                    .get_mut(material_handles.get(event.target).unwrap())
                    .unwrap();
                let mut color = material.base_color.as_hsla_f32();
                let to_u8 = 255.0 / 360.0;
                color[0] =
                    ((color[0] * to_u8) as u8).wrapping_add_signed(event.delta.x as i8) as f32 / to_u8;
                material.base_color = Color::hsla(color[0], color[1], color[2], color[3]);
            },
            HexEvent::Over(event) => {
                let material = materials.get_mut(material_handles.get(event.target).unwrap()).unwrap();
                set_material_lightness(material, 0.75);
            },
            HexEvent::Out(event) => {
                let material = materials.get_mut(material_handles.get(event.target).unwrap()).unwrap();
                set_material_lightness(material, 0.5);
            },
            HexEvent::Down(event) => {
                let material = materials.get_mut(material_handles.get(event.target).unwrap()).unwrap();
                set_material_lightness(material, 0.5);
            },
            HexEvent::Up(event) => {
                let material = materials.get_mut(material_handles.get(event.target).unwrap()).unwrap();
                set_material_lightness(material, 0.75);
            },
        }
    }
}

fn set_material_lightness(material: &mut StandardMaterial, lightness: f32) {
    let mut color = material.base_color.as_hsla_f32();
    color[2] = lightness;
    material.base_color = Color::hsla(color[0], color[1], color[2], color[3]);
}
