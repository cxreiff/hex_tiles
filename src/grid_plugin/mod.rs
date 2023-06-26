mod setup;
mod tile_events;
mod tile_variant;

use bevy::prelude::*;

use crate::GameState;

use self::setup::setup;
pub use self::setup::{GridTracker, HexCoords, TileSelector};
use self::tile_events::{handle_spawn_tile, handle_update_parent, TileEvent};
pub use self::tile_variant::TileVariant;

pub static GRID_RADIUS: u32 = 4;
pub static GRID_MARGIN: f32 = 0.05;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileEvent>()
            .add_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (handle_update_parent, handle_spawn_tile)
                    .chain()
                    .in_set(OnUpdate(GameState::Playing))
                    .distributive_run_if(on_event::<TileEvent>()),
            );
    }
}
