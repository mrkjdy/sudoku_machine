use bevy::{ecs::system::EntityCommands, prelude::*};
use derive_builder::Builder;
use strum_macros::Display;

use crate::plugins::common::theme::{node::ListItemButton, Themed};

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
#[require(Node)]
pub struct DropdownContainer {
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
    dropdown: DropdownContainer,
    text_font: TextFont,
    container_node: Node,
    button_node: Node,
    button_text_node: Node,
    list_node: Node,
    // border_color: BorderColor,
    // border_radius: BorderRadius,
    // background_color: BackgroundColor,
}

impl DropdownWidgetBuilder {
    pub fn build(&self) -> DropdownWidget {
        let DropdownWidgetBuilder {
            dropdown,
            text_font,
            container_node,
            button_node,
            button_text_node,
            list_node,
            // border_color,
            // border_radius,
            // background_color,
        } = self;
        DropdownWidget {
            dropdown: dropdown.clone().unwrap_or_default(),
            text_font: text_font.clone().unwrap_or_default(),
            container_node: container_node.clone().unwrap_or_default(),
            button_node: button_node.clone().unwrap_or_default(),
            button_text_node: button_text_node.clone().unwrap_or_default(),
            list_node: list_node.clone().unwrap_or_default(),
            // border_color: border_color.unwrap_or_default(),
            // border_radius: border_radius.unwrap_or_default(),
            // background_color: background_color.unwrap_or_default(),
        }
    }
}

#[derive(Component)]
#[require(Themed, Button)]
struct DropdownButton;

#[derive(Component)]
#[require(Themed, Text)]
struct DropdownButtonText;

#[derive(Component)]
#[require(Themed, Text)]
struct DropdownButtonIcon;

#[derive(Component)]
#[require(Themed, Node)]
struct DropdownList;

#[derive(Component)]
#[require(ListItemButton)]
struct DropdownListItem(usize);

#[derive(Component)]
#[require(Themed, Text)]
struct DropdownListItemText;

#[derive(Component)]
#[require(Themed, Text)]
struct DropdownListItemIcon;

impl Spawnable for DropdownWidget {
    fn spawn_with_components<'a, S: Spawn>(
        &self,
        spawner: &'a mut S,
        components: impl Bundle,
    ) -> EntityCommands<'a> {
        let DropdownWidget {
            dropdown,
            container_node,
            button_node,
            text_font,
            button_text_node,
            list_node,
            // border_color,
            // border_radius,
            // background_color,
        } = self;

        let container_bundle = (dropdown.clone(), container_node.clone());

        let button_bundle = (
            Node {
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                ..button_node.clone()
            },
            DropdownButton,
        );

        let initial_text = dropdown
            .options
            .get(dropdown.selected)
            .unwrap_or(&"".to_string())
            .clone();

        let button_text_bundle = (
            Text::new(initial_text),
            text_font.clone(),
            button_text_node.clone(),
            DropdownButtonText,
        );

        let button_icon_bundle = (
            Text::new(DropdownIcon::Closed.to_string()),
            text_font.clone(),
            DropdownButtonIcon,
        );

        let list_container_bundle = (
            Node {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..list_node.clone()
            },
            BackgroundColor(Color::default()),
            Visibility::Hidden,
            DropdownList,
            GlobalZIndex(100),
        );

        let list_item_bundles = dropdown
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let list_item_bundle = (
                    Node {
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        ..button_node.clone()
                    },
                    DropdownListItem(i),
                );
                let list_item_text_bundle = (
                    Text::new(option.clone()),
                    text_font.clone(),
                    DropdownListItemText,
                );
                let list_item_icon_text = match i == dropdown.selected {
                    true => SelectionIcon::Selected,
                    false => SelectionIcon::Unselected,
                }
                .to_string();
                let list_item_icon_bundle = (
                    Text::new(list_item_icon_text),
                    text_font.clone(),
                    DropdownListItemIcon,
                );
                (
                    list_item_bundle,
                    list_item_text_bundle,
                    list_item_icon_bundle,
                )
            })
            .collect::<Vec<_>>();

        let mut ec = spawner.spawn((container_bundle, components));
        ec.with_children(|parent| {
            ChildBuild::spawn(parent, button_bundle).with_children(|parent| {
                ChildBuild::spawn(parent, button_text_bundle);
                ChildBuild::spawn(parent, button_icon_bundle);
            });
            ChildBuild::spawn(parent, list_container_bundle).with_children(|parent| {
                for (list_item_bundle, list_item_text_bundle, list_item_icon_bundle) in
                    list_item_bundles
                {
                    ChildBuild::spawn(parent, list_item_bundle).with_children(|parent| {
                        ChildBuild::spawn(parent, list_item_text_bundle);
                        ChildBuild::spawn(parent, list_item_icon_bundle);
                    });
                }
            });
        });
        return ec;
    }
}

fn dropdown_button_text_system(
    container_query: Query<(&DropdownContainer, &Children), Changed<DropdownContainer>>,
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
        button_text.0 = dropdown.options[dropdown.selected].clone();
    }
}

fn dropdown_button_icon_system(
    list_query: Query<(&Parent, &Visibility), (Changed<Visibility>, With<DropdownList>)>,
    container_query: Query<&Children, With<DropdownContainer>>,
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
        button_icon.0 = match *list_visibility {
            Visibility::Visible => DropdownIcon::Open,
            _ => DropdownIcon::Closed,
        }
        .to_string();
    }
}

fn dropdown_list_visibility_system(
    buttons: Res<ButtonInput<MouseButton>>,
    button_query: Query<(&Interaction, &Parent), With<DropdownButton>>,
    container_query: Query<&Children, With<DropdownContainer>>,
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
    mut container_query: Query<&mut DropdownContainer>,
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
        previous_list_item_icon.0 = SelectionIcon::Unselected.to_string();
        // Change the selected option in the container
        dropdown_container.selected = list_item.0;
        // Add the selected icon to the newly selected option
        let pressed_list_item_icon_id = interacted_list_item_children[1];
        let mut pressed_list_item_icon = list_item_icon_query
            .get_mut(pressed_list_item_icon_id)
            .unwrap();
        pressed_list_item_icon.0 = SelectionIcon::Selected.to_string();
    }
}

fn dropdown_list_position_system(
    button_query: Query<
        (&Parent, &Node),
        (Changed<Node>, With<DropdownButton>, Without<DropdownList>),
    >,
    container_query: Query<&Children, With<DropdownContainer>>,
    mut list_query: Query<&mut Node, With<DropdownList>>,
) {
    for (button_parent, button_node) in button_query.iter() {
        let container_id = button_parent.get();
        let container_children = container_query.get(container_id).unwrap();
        let mut list_style = list_query.get_mut(container_children[1]).unwrap();
        list_style.top = button_node.height;
    }
}

// TODO - System to change focus back to dropdown button after clicking a list option?
