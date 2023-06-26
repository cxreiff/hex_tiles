use bevy::utils::HashMap;

use bevy::prelude::*;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Clone, PartialEq, Eq, Hash, Default, EnumIter, Component)]
pub enum TileVariant {
    Cyan,
    #[default]
    Purple,
    Orange,
}

impl TileVariant {
    pub fn initialize_materials(
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> HashMap<TileVariant, Handle<StandardMaterial>> {
        Self::iter()
            .map(|tile_variant| {
                (
                    tile_variant.clone(),
                    materials.add(StandardMaterial::from(tile_variant)),
                )
            })
            .collect()
    }
}

impl From<TileVariant> for Color {
    fn from(value: TileVariant) -> Self {
        match value {
            TileVariant::Cyan => Color::TEAL,
            TileVariant::Purple => Color::PURPLE,
            TileVariant::Orange => Color::ORANGE,
        }
    }
}

impl From<TileVariant> for StandardMaterial {
    fn from(value: TileVariant) -> Self {
        StandardMaterial::from(Color::from(value))
    }
}
