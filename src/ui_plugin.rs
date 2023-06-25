use crate::{
    loading_plugin::LoadedAssets,
    world_plugin::{TileVariant, WorldTracker},
    GameState,
};
use bevy::{prelude::*, ui::FocusPolicy};
use strum::IntoEnumIterator;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(palette_system.in_set(OnUpdate(GameState::Playing)));
    }
}

fn ui_setup(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(
                    Val::Percent(2.0),
                    Val::Percent(2.0),
                    Val::Percent(1.0),
                    Val::Percent(2.0),
                ),
                gap: Size::all(Val::Percent(2.0)),
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                gap: Size::all(Val::Px(5.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 48.,
                                    color: Color::PURPLE,
                                },
                            ));
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(6.0)),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "arrow keys to orbit camera.\nleft lick to add tiles.",
                                            TextStyle {
                                                font: assets.font.clone(),
                                                font_size: 18.,
                                                color: Color::PURPLE,
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                padding: UiRect::all(Val::Percent(5.0)),
                                                ..default()
                                            },
                                        ),
                                    );
                                });
                        });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        flex_direction: FlexDirection::RowReverse,
                        gap: Size::all(Val::Percent(1.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    TileVariant::iter().for_each(|palette_tile| {
                        parent.spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(6.0), Val::Percent(95.0)),
                                    ..default()
                                },
                                background_color: Color::from(palette_tile.clone()).into(),
                                ..default()
                            },
                            palette_tile,
                        ));
                    })
                });
        });
}

fn palette_system(
    mut interactions: Query<
        (&Interaction, &TileVariant, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut world_tracker: ResMut<WorldTracker>,
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
