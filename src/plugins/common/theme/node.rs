use bevy::prelude::*;
use derive_builder::Builder;

use super::ThemeComponent;

// TODO - Merge this and button somehow?

#[derive(Bundle, Clone)]
pub struct ThemedNodeBundle {
    ui_rect: ThemeComponent<UiRect>,
    border_color: ThemeComponent<BorderColor>,
    border_radius: ThemeComponent<BorderRadius>,
    background_color: ThemeComponent<BackgroundColor>,
    node_bundle: NodeBundle,
}

#[derive(Builder)]
#[builder(name = "ThemedNodeBundleBuilder", build_fn(skip), default, public)]
struct _ThemedNodeBundleBuilderBase {
    style: Style,
    ui_rect: UiRect,
    border_color: BorderColor,
    border_radius: BorderRadius,
    background_color: ThemeComponent<BackgroundColor>,
    z_index: ZIndex,
    visibility: Visibility,
}

impl ThemedNodeBundleBuilder {
    pub fn build(&self) -> ThemedNodeBundle {
        let ThemedNodeBundleBuilder {
            style,
            ui_rect,
            border_color,
            border_radius,
            background_color,
            z_index,
            visibility,
        } = self;
        ThemedNodeBundle {
            ui_rect: ui_rect.clone().into(),
            border_color: border_color.clone().into(),
            background_color: background_color.clone().unwrap_or_default(),
            border_radius: border_radius.clone().into(),
            node_bundle: NodeBundle {
                style: style.clone().unwrap_or_default(),
                z_index: z_index.unwrap_or_default(),
                visibility: visibility.unwrap_or_default(),
                ..default()
            },
        }
    }
}
