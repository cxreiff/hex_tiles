use bevy::prelude::*;

use crate::grid_plugin::{GridTracker, TileVariant};

pub fn tile_select_system(
    mut interactions: Query<
        (&Interaction, &TileVariant, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut world_tracker: ResMut<GridTracker>,
) {
    for (interaction, tile_variant, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Clicked => {
                world_tracker.current_tile_variant = tile_variant.clone();
            }
            Interaction::Hovered => {
                if let Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                } = Color::from(tile_variant.clone()).as_hsla()
                {
                    background_color.0 = Color::Hsla {
                        hue,
                        saturation,
                        lightness: lightness + 0.05,
                        alpha,
                    };
                };
            }
            Interaction::None => {
                if let Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                } = Color::from(tile_variant.clone()).as_hsla()
                {
                    background_color.0 = Color::Hsla {
                        hue,
                        saturation,
                        lightness: lightness - 0.05,
                        alpha,
                    };
                };
            }
        }
    }
}
