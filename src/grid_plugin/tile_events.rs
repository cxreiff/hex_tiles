use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::grid_plugin::{HexCoords, TileSelector, GRID_MARGIN};

use super::GridTracker;

pub enum TileEvent {
    Over(ListenedEvent<Over>),
    Out(ListenedEvent<Out>),
    Down(ListenedEvent<Down>),
    Up(ListenedEvent<Up>),
}

impl From<ListenedEvent<Over>> for TileEvent {
    fn from(event: ListenedEvent<Over>) -> Self {
        TileEvent::Over(event)
    }
}
impl From<ListenedEvent<Out>> for TileEvent {
    fn from(event: ListenedEvent<Out>) -> Self {
        TileEvent::Out(event)
    }
}
impl From<ListenedEvent<Down>> for TileEvent {
    fn from(event: ListenedEvent<Down>) -> Self {
        TileEvent::Down(event)
    }
}
impl From<ListenedEvent<Up>> for TileEvent {
    fn from(event: ListenedEvent<Up>) -> Self {
        TileEvent::Up(event)
    }
}

pub fn handle_update_parent(
    mut commands: Commands,
    mut events: EventReader<TileEvent>,
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
            TileEvent::Over(event) => {
                if !mouse.pressed(MouseButton::Left) {
                    update_parent_interaction(&mut commands, &q_interaction, event);
                }
            }
            TileEvent::Out(event) => {
                if !mouse.pressed(MouseButton::Left) {
                    update_parent_interaction(&mut commands, &q_interaction, event);
                }
            }
            TileEvent::Down(event) => {
                update_parent_interaction(&mut commands, &q_interaction, event);
            }
            TileEvent::Up(event) => {
                update_parent_interaction(&mut commands, &q_interaction, event);
            }
        }
    }
}

pub fn handle_spawn_tile(
    mut commands: Commands,
    mut events: EventReader<TileEvent>,
    mut tracker: ResMut<GridTracker>,
    mouse: Res<Input<MouseButton>>,
    mut q_transforms: Query<&mut Transform, With<TileSelector>>,
) {
    fn spawn_tile<T: IsPointerEvent>(
        commands: &mut Commands,
        tracker: &mut ResMut<GridTracker>,
        q_transforms: &mut Query<&mut Transform, With<TileSelector>>,
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
                .with_scale(Vec3::new(
                    1.0 - GRID_MARGIN,
                    1.0 - GRID_MARGIN,
                    1.0 - GRID_MARGIN,
                )),
                mesh: tracker.mesh_handle.clone(),
                material: tracker
                    .tile_materials
                    .get(&tracker.current_tile_variant)
                    .unwrap()
                    .clone(),
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
            TileEvent::Down(event) => {
                if let Some(HexCoords { layer, .. }) = tracker.tiles.get(&event.listener) {
                    tracker.drag_layer = *layer;
                    spawn_tile(&mut commands, &mut tracker, &mut q_transforms, event);
                }
            }
            TileEvent::Over(event) => {
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
