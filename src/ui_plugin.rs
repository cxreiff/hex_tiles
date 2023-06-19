use bevy::prelude::*;

use crate::{loading_plugin::LoadedAssets, GameState};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(ui_control);
    }
}

fn ui_setup(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "use arrow keys to orbit camera.\nclick to add tiles.",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 18.,
                    color: Color::PURPLE,
                },
            ));
        });
}

fn ui_control(mut _commands: Commands) {
    //
}
