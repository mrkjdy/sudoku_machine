use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{diagnostic::DiagnosticsStore, prelude::*};

use super::common::theme::Themed;

pub fn fps_plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, fps_setup)
        .add_systems(Update, fps_system);
}

#[derive(Component)]
struct FpsText;

fn fps_setup(mut commands: Commands) {
    let themed_fps_text_bundle = (
        FpsText,
        Text::new(""),
        TextFont::from_font_size(12.0),
        Themed,
        Node {
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            height: Val::Px(20.0),
            ..default()
        },
    );

    commands.spawn(themed_fps_text_bundle);
}

fn fps_system(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FpsText>>,
    mut smoothed_frame_time_ms: Local<f64>,
) {
    // Get the frame time measurement from the diagnostics store
    let current_frame_time_opt =
        diagnostics.get_measurement(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
    if current_frame_time_opt.is_none() {
        return;
    }
    let current_frame_time_ms: f64 = current_frame_time_opt.unwrap().value; // 0.00909...

    // Calculate the smoothing factor based on the frame time
    let smoothing_base: f64 = 0.9;
    let smoothing_factor = smoothing_base.powf(current_frame_time_ms / 60.0 * 1000.0); // 0.9841630

    // 0.00833...
    // Calculate the new smoothed frame time value using the smoothing factor
    *smoothed_frame_time_ms *= smoothing_factor; // 0.00820135
    *smoothed_frame_time_ms += current_frame_time_ms * (1.0 - smoothing_factor); // 0.008345322

    // Calculate the FPS value based on the smoothed frame time
    let fps = 1000.0 / *smoothed_frame_time_ms;

    // Set the new FPS value in the text component
    let mut text = fps_text_query.single_mut();
    text.0 = format!("{:>7.2}", fps);
}
