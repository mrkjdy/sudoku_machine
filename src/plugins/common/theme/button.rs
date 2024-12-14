use bevy::prelude::*;
use derive_builder::Builder;

use super::ThemeComponent;

#[derive(Bundle, Clone)]
pub struct ThemedButtonBundle {
    ui_rect: ThemeComponent<UiRect>,
    border_color: ThemeComponent<BorderColor>,
    border_radius: ThemeComponent<BorderRadius>,
    background_color: ThemeComponent<BackgroundColor>,
    button_bundle: ButtonBundle,
}

#[derive(Builder)]
#[builder(name = "ThemedButtonBundleBuilder", build_fn(skip), default, public)]
struct _ThemedButtonBundleBuilderBase {
    style: Style,
    ui_rect: UiRect,
    border_color: BorderColor,
    border_radius: BorderRadius,
    background_color: BackgroundColor,
}

impl ThemedButtonBundleBuilder {
    pub fn build(&self) -> ThemedButtonBundle {
        let ThemedButtonBundleBuilder {
            style,
            ui_rect,
            border_color,
            border_radius,
            background_color,
        } = self;
        ThemedButtonBundle {
            ui_rect: ui_rect.clone().into(),
            border_color: border_color.clone().into(),
            background_color: background_color.clone().into(),
            border_radius: border_radius.clone().into(),
            button_bundle: ButtonBundle {
                style: style.clone().unwrap_or_default(),
                ..default()
            },
        }
    }
}
