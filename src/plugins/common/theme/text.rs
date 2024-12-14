use bevy::{prelude::*, text::BreakLineOn};
use derive_builder::Builder;

use super::ThemeComponent;

#[derive(Default, Clone, Copy)]
pub enum FontWeight {
    #[default]
    Regular,
    Bold,
}

#[derive(Default, Clone, Copy)]
pub(super) struct FontColor(pub(super) Color);

#[derive(Bundle)]
pub struct ThemedTextBundle {
    font_weight: ThemeComponent<FontWeight>,
    font_color: ThemeComponent<FontColor>,
    text_bundle: TextBundle,
}

#[derive(Builder)]
#[builder(name = "ThemedTextBundleBuilder", build_fn(skip), default, public)]
struct _ThemedTextBundleBuilderBase {
    value: String,
    font_weight: FontWeight,
    font_size: f32,
    font_color: FontColor,
    background_color: BackgroundColor,
    line_break_behavior: BreakLineOn,
    style: Style,
    justify_text: JustifyText,
}

impl ThemedTextBundleBuilder {
    pub fn build(&self) -> ThemedTextBundle {
        let ThemedTextBundleBuilder {
            value,
            font_weight,
            font_size,
            font_color,
            background_color,
            line_break_behavior,
            style,
            justify_text,
        } = self;
        ThemedTextBundle {
            font_weight: font_weight.clone().into(),
            font_color: font_color.clone().into(),
            text_bundle: TextBundle {
                style: style.clone().unwrap_or_default(),
                text: Text {
                    sections: vec![TextSection {
                        value: value.clone().unwrap_or_default(),
                        style: TextStyle {
                            font_size: font_size.unwrap_or_default(),
                            ..default()
                        },
                    }],
                    justify: justify_text.unwrap_or_default(),
                    linebreak_behavior: line_break_behavior.unwrap_or_default(),
                },
                background_color: background_color.unwrap_or_default(),
                ..default()
            },
        }
    }
}
