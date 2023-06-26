mod setup;
mod tile_select;

use bevy::prelude::*;

use crate::GameState;

use self::{setup::setup, tile_select::tile_select_system};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(tile_select_system.in_set(OnUpdate(GameState::Playing)));
    }
}
