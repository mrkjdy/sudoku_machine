use bevy::{ecs::system::EntityCommands, prelude::*};
use derive_builder::Builder;
use strum_macros::Display;

use crate::plugins::common::theme::button::ThemedButtonBundleBuilder;
use crate::plugins::common::theme::node::ThemedNodeBundleBuilder;
use crate::plugins::common::theme::text::ThemedTextBundleBuilder;

use super::{Spawn, Spawnable};

pub fn dropdown_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            dropdown_button_text_system,
            dropdown_button_icon_system,
            dropdown_list_visibility_system,
            dropdown_list_selection_system,
            dropdown_list_position_system,
        ),
    );
}

#[derive(Default, Component, Clone)]
pub struct DropdownData {
    pub selected: usize,
    pub options: Vec<String>,
}

#[derive(Default, Display)]
enum DropdownIcon {
    #[default]
    #[strum(to_string = "v")]
    Closed,
    #[strum(to_string = "^")]
    Open,
}

impl From<DropdownIcon> for String {
    fn from(value: DropdownIcon) -> Self {
        value.to_string()
    }
}

#[derive(Display)]
enum SelectionIcon {
    #[strum(to_string = "*")]
    Selected,
    #[strum(to_string = "")]
    Unselected,
}

#[derive(Builder)]
#[builder(build_fn(skip), default, public)]
pub struct DropdownWidget {
    dropdown: DropdownData,
    font_size: f32,
    container_style: Style,
    button_style: Style,
    button_text_style: Style,
    list_style: Style,
    // border_color: BorderColor,
    // border_radius: BorderRadius,
    // background_color: BackgroundColor,
}

impl DropdownWidgetBuilder {
    pub fn build(&self) -> DropdownWidget {
        let DropdownWidgetBuilder {
            dropdown,
            font_size,
            container_style,
            button_style,
            button_text_style,
            list_style,
            // border_color,
            // border_radius,
            // background_color,
        } = self;
        DropdownWidget {
            dropdown: dropdown.clone().unwrap_or_default(),
            font_size: font_size.unwrap_or_default(),
            container_style: container_style.clone().unwrap_or_default(),
            button_style: button_style.clone().unwrap_or_default(),
            button_text_style: button_text_style.clone().unwrap_or_default(),
            list_style: list_style.clone().unwrap_or_default(),
            // border_color: border_color.unwrap_or_default(),
            // border_radius: border_radius.unwrap_or_default(),
            // background_color: background_color.unwrap_or_default(),
        }
    }
}

#[derive(Component)]
struct DropdownButton;

#[derive(Component)]
struct DropdownButtonText;

#[derive(Component)]
struct DropdownButtonIcon;

#[derive(Component)]
struct DropdownList;

#[derive(Component)]
struct DropdownListItem(usize);

#[derive(Component)]
struct DropdownListItemText;

#[derive(Component)]
struct DropdownListItemIcon;

impl Spawnable for DropdownWidget {
    fn spawn_with_components<'a, S: Spawn>(
        &self,
        spawner: &'a mut S,
        components: impl Bundle,
    ) -> EntityCommands<'a> {
        let DropdownWidget {
            dropdown,
            container_style,
            button_style,
            font_size,
            button_text_style,
            list_style,
            // border_color,
            // border_radius,
            // background_color,
        } = self;

        let container = NodeBundle {
            style: container_style.clone(),
            ..default()
        };

        let button = ThemedButtonBundleBuilder::default()
            .style(Style {
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..button_style.clone()
            })
            .build();

        let button_text = ThemedTextBundleBuilder::default()
            .value(
                dropdown
                    .options
                    .get(dropdown.selected)
                    .unwrap_or(&"".to_string())
                    .into(),
            )
            .font_size(*font_size)
            .style(button_text_style.clone())
            .build();

        let button_icon = ThemedTextBundleBuilder::default()
            .value(DropdownIcon::Closed.into())
            .font_size(*font_size)
            .build();

        let list = ThemedNodeBundleBuilder::default()
            .style(Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..list_style.clone()
            })
            .z_index(ZIndex::Global(1000))
            .visibility(Visibility::Hidden)
            .build();

        let list_item = ThemedButtonBundleBuilder::default()
            .style(Style {
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..button_style.clone()
            })
            .ui_rect(UiRect::all(Val::ZERO))
            // .border_color(BorderColor(background_color.0))
            .build();

        let mut dropdown_list_item_text_builder = ThemedTextBundleBuilder::default();
        dropdown_list_item_text_builder.font_size(*font_size);

        let mut dropdown_list_item_icon_builder = ThemedTextBundleBuilder::default();
        dropdown_list_item_icon_builder.font_size(*font_size);

        let mut ec = spawner.spawn((container, dropdown.clone()));
        ec.insert(components);
        ec.with_children(|parent| {
            parent
                .spawn((button, DropdownButton))
                .with_children(|parent| {
                    parent.spawn((button_text, DropdownButtonText));
                    parent.spawn((button_icon, DropdownButtonIcon));
                });
            parent.spawn((list, DropdownList)).with_children(|parent| {
                for (i, option) in dropdown.options.iter().enumerate() {
                    parent
                        .spawn((list_item.clone(), DropdownListItem(i)))
                        .with_children(|parent| {
                            let dropdown_list_item_text =
                                dropdown_list_item_text_builder.value(option.into()).build();
                            parent.spawn((dropdown_list_item_text, DropdownListItemText));
                            let icon_text = if i == dropdown.selected {
                                SelectionIcon::Selected
                            } else {
                                SelectionIcon::Unselected
                            }
                            .to_string();
                            let dropdown_list_item_icon =
                                dropdown_list_item_icon_builder.value(icon_text).build();
                            parent.spawn((dropdown_list_item_icon, DropdownListItemIcon));
                        });
                }
            });
        });
        return ec;
    }
}

fn dropdown_button_text_system(
    container_query: Query<(&DropdownData, &Children), Changed<DropdownData>>,
    button_query: Query<&Children, With<DropdownButton>>,
    mut button_text_query: Query<&mut Text, With<DropdownButtonText>>,
) {
    for (dropdown, container_children) in container_query.iter() {
        // Get the button and its children
        let button_id = container_children[0];
        let button_children = button_query.get(button_id).unwrap();
        // Set the button text
        let button_text_id = button_children[0];
        let mut button_text = button_text_query.get_mut(button_text_id).unwrap();
        button_text.sections[0].value = dropdown.options[dropdown.selected].clone();
    }
}

fn dropdown_button_icon_system(
    list_query: Query<(&Parent, &Visibility), (Changed<Visibility>, With<DropdownList>)>,
    container_query: Query<&Children, With<DropdownData>>,
    button_query: Query<&Children, With<DropdownButton>>,
    mut button_icon_query: Query<&mut Text, With<DropdownButtonIcon>>,
) {
    for (list_parent, list_visibility) in list_query.iter() {
        // Get the container and its children
        let container_id = list_parent.get();
        let container_children = container_query.get(container_id).unwrap();
        // Get the button and its children
        let button_id = container_children[0];
        let button_children = button_query.get(button_id).unwrap();
        // Set the corresponding icon
        let button_icon_id = button_children[1];
        let mut button_icon = button_icon_query.get_mut(button_icon_id).unwrap();
        button_icon.sections[0].value = match *list_visibility {
            Visibility::Visible => DropdownIcon::Open,
            _ => DropdownIcon::Closed,
        }
        .to_string();
    }
}

fn dropdown_list_visibility_system(
    buttons: Res<ButtonInput<MouseButton>>,
    button_query: Query<(&Interaction, &Parent), With<DropdownButton>>,
    container_query: Query<&Children, With<DropdownData>>,
    mut list_query: Query<&mut Visibility, With<DropdownList>>,
) {
    if buttons.get_just_pressed().len() <= 0 {
        return;
    }
    for (&button_interaction, button_parent) in button_query.iter() {
        // Get the list
        let container_id = button_parent.get();
        let container_children = container_query.get(container_id).unwrap();
        let list_id = container_children[1];
        let mut list_visibility = list_query.get_mut(list_id).unwrap();
        // Set the list visibility
        *list_visibility = if button_interaction == Interaction::Pressed
            && *list_visibility == Visibility::Hidden
        {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn dropdown_list_selection_system(
    interacted_list_item_query: Query<
        (&Interaction, &Parent, &DropdownListItem, &Children),
        Changed<Interaction>,
    >,
    list_query: Query<(&Parent, &Children), With<DropdownList>>,
    mut container_query: Query<&mut DropdownData>,
    previous_list_item_query: Query<&Children, With<DropdownListItem>>,
    mut list_item_icon_query: Query<&mut Text, With<DropdownListItemIcon>>,
) {
    for (interaction, parent, list_item, interacted_list_item_children) in
        interacted_list_item_query.iter()
    {
        if *interaction != Interaction::Pressed {
            continue;
        }
        // Find the dropdown container for this pressed option
        let list_id = parent.get();
        let (dropdown_list_parent, list_items) = list_query.get(list_id).unwrap();
        let dropdown_list_parent_id = dropdown_list_parent.get();
        let mut dropdown_container = container_query.get_mut(dropdown_list_parent_id).unwrap();
        // Remove the selected icon from the previous option
        let previous_list_item_id = list_items[dropdown_container.selected];
        let previous_list_item_children =
            previous_list_item_query.get(previous_list_item_id).unwrap();
        let previous_list_item_icon_id = previous_list_item_children[1];
        let mut previous_list_item_icon = list_item_icon_query
            .get_mut(previous_list_item_icon_id)
            .unwrap();
        previous_list_item_icon.sections[0].value = SelectionIcon::Unselected.to_string();
        // Change the selected option in the container
        dropdown_container.selected = list_item.0;
        // Add the selected icon to the newly selected option
        let pressed_list_item_icon_id = interacted_list_item_children[1];
        let mut pressed_list_item_icon = list_item_icon_query
            .get_mut(pressed_list_item_icon_id)
            .unwrap();
        pressed_list_item_icon.sections[0].value = SelectionIcon::Selected.to_string();
    }
}

fn dropdown_list_position_system(
    button_query: Query<(&Parent, &Node), (Changed<Node>, With<DropdownButton>)>,
    container_query: Query<&Children, With<DropdownData>>,
    mut list_query: Query<&mut Style, With<DropdownList>>,
) {
    for (button_parent, button_node) in button_query.iter() {
        let container_id = button_parent.get();
        let container_children = container_query.get(container_id).unwrap();
        let mut list_style = list_query.get_mut(container_children[1]).unwrap();
        let button_size = button_node.size();
        list_style.top = Val::Px(button_size.y);
    }
}

// TODO - System to change focus back to dropdown button after clicking a list option?
