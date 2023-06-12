use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};

#[cfg(debug_assertions)]
use {
    crate::GameState,
    bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    bevy::input::common_conditions::input_toggle_active,
    bevy::window::PrimaryWindow,
    bevy_debug_text_overlay::{screen_print, OverlayPlugin},
    bevy_inspector_egui::quick::WorldInspectorPlugin,
};

pub const ASPECT_RATIO: f32 = 10. / 16.;
pub const WIDTH: f32 = 500.;
pub const HEIGHT: f32 = WIDTH * ASPECT_RATIO;

#[cfg(debug_assertions)]
#[derive(Resource, Default)]
pub struct DebugOptions {
    printed_info_enabled: bool,
}

#[derive(Component)]
pub struct CameraFlag;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "bevy_trunk_template".to_string(),
                            canvas: Some("#bevy".to_owned()),
                            fit_canvas_to_parent: true,
                            present_mode: PresentMode::AutoVsync,
                            resolution: WindowResolution::new(WIDTH, HEIGHT),
                            ..default()
                        }),
                        ..default()
                    })
                    .set(AssetPlugin {
                        watch_for_changes: true,
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
            );

        #[cfg(debug_assertions)]
        {
            app.insert_resource(DebugOptions::default())
                .add_plugin(OverlayPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(
                    WorldInspectorPlugin::default()
                        .run_if(input_toggle_active(false, KeyCode::Key2)),
                )
                .add_system(debug_toggle_system)
                .add_system(debug_system);
        }
    }
}

#[cfg(debug_assertions)]
fn debug_system(
    time: Res<Time>,
    debug_options: Res<DebugOptions>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    app_state: Res<State<GameState>>,
    diagnostics: Res<Diagnostics>,
) {
    let current_time = time.elapsed_seconds();
    let at_interval = |t: f32| current_time % t < time.delta_seconds();
    if debug_options.printed_info_enabled && at_interval(0.25) {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps) = fps.value() {
                screen_print!(sec: 0.3, col: Color::CYAN, "fps: {fps}");
            };
        };
        screen_print!(sec: 0.3, col: Color::CYAN, "game state: {:?}", app_state.0);
        if let Ok(window) = window_query.get_single() {
            if let Some(position) = window.cursor_position() {
                screen_print!(sec: 0.3, col: Color::CYAN, "cursor_position: {}", position);
            };
        };
    }
}

#[cfg(debug_assertions)]
fn debug_toggle_system(input: Res<Input<KeyCode>>, mut debug_options: ResMut<DebugOptions>) {
    if input.just_pressed(KeyCode::Key1) {
        debug_options.printed_info_enabled = !debug_options.printed_info_enabled;
    }
}

pub fn get_world_position(
    raw_position: Vec2,
    window: &Window,
    camera_transform: &GlobalTransform,
) -> Vec3 {
    let adjusted_position = Vec3::new(
        raw_position.x / window.width() * WIDTH - WIDTH / 2.,
        raw_position.y / window.height() * HEIGHT - HEIGHT / 2.,
        0.,
    );

    *camera_transform * adjusted_position
}
