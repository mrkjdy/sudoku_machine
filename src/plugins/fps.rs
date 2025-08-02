use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use super::common::theme::text::{ThemedFontWeight, ThemedTextColor};

pub fn fps_plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
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
        ThemedFontWeight::Regular,
        ThemedTextColor,
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
    time: Res<Time>,
    mut fps_text_query: Query<&mut Text, With<FpsText>>,
    mut frame_count: Local<u32>,
    mut elapsed: Local<f32>,
    mut last_fps: Local<f32>,
) {
    *frame_count += 1;
    *elapsed += time.delta_secs();

    if *elapsed >= 1.0 {
        *last_fps = *frame_count as f32 / *elapsed;
        *frame_count = 0;
        *elapsed = 0.0;
    }

    let mut text = fps_text_query
        .single_mut()
        .expect("Expected exactly one FpsText entity, but found none or multiple.");
    text.0 = format!("{:>7.2}", *last_fps);
}
