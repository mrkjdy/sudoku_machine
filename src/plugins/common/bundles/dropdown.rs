use bevy::{ecs::spawn::SpawnIter, prelude::*};
use strum_macros::Display;

use crate::plugins::common::theme::{node::ListItemButton, Themed};

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

impl From<bool> for SelectionIcon {
    fn from(b: bool) -> Self {
        if b {
            SelectionIcon::Selected
        } else {
            SelectionIcon::Unselected
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

struct DropdownButtonBundleOptions {
    text: String,
    text_font: TextFont,
    button_node: Node,
    button_text_node: Node,
}

fn dropdown_button_bundle(options: DropdownButtonBundleOptions) -> impl Bundle {
    let DropdownButtonBundleOptions {
        text,
        text_font,
        button_node,
        button_text_node,
    } = options;

    let dropdown_button_text_bundle = (
        DropdownButtonText,
        Text::new(text),
        text_font.clone(),
        button_text_node,
    );

    let dropdown_button_icon_bundle = (
        DropdownButtonIcon,
        Text::new(DropdownIcon::Closed.to_string()),
        text_font,
    );

    (
        DropdownButton,
        Node {
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..button_node
        },
        children![dropdown_button_text_bundle, dropdown_button_icon_bundle],
    )
}

struct DropdownListItemBundleOptions {
    index: usize,
    button_node: Node,
    text: String,
    text_font: TextFont,
    selected: bool,
}

fn dropdown_list_item_bundle(options: DropdownListItemBundleOptions) -> impl Bundle {
    let DropdownListItemBundleOptions {
        index,
        button_node,
        text,
        text_font,
        selected,
    } = options;

    let dropdown_list_item_text = (DropdownListItemText, Text::new(text), text_font.clone());

    let dropdown_list_item_icon = (
        DropdownListItemIcon,
        Text::new(SelectionIcon::from(selected).to_string()),
        text_font,
    );

    (
        DropdownListItem(index),
        Node {
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..button_node
        },
        children![dropdown_list_item_text, dropdown_list_item_icon],
    )
}

struct DropdownListBundleOptions {
    options: Vec<String>,
    selected: usize,
    list_node: Node,
    button_node: Node,
    text_font: TextFont,
}

fn dropdown_list_bundle(options: DropdownListBundleOptions) -> impl Bundle {
    let DropdownListBundleOptions {
        options,
        selected,
        list_node,
        button_node,
        text_font,
    } = options;

    let option_bundles = options.into_iter().enumerate().map(move |(index, text)| {
        dropdown_list_item_bundle(DropdownListItemBundleOptions {
            index,
            button_node: button_node.clone(),
            text,
            text_font: text_font.clone(),
            selected: index == selected,
        })
    });

    (
        DropdownList,
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            ..list_node
        },
        BackgroundColor(Color::default()),
        Visibility::Hidden,
        GlobalZIndex(100),
        Children::spawn(SpawnIter(option_bundles)),
    )
}

#[derive(Default)]
pub struct DropdownBundleOptions {
    pub options: Vec<String>,
    pub selected: usize,
    pub container_node: Node,
    pub button_node: Node,
    pub text_font: TextFont,
    pub button_text_node: Node,
    pub list_node: Node,
}

pub fn dropdown_bundle(options: DropdownBundleOptions) -> impl Bundle {
    let DropdownBundleOptions {
        options,
        selected,
        container_node,
        button_node,
        text_font,
        button_text_node,
        list_node,
    } = options;

    let initial_text = options
        .get(selected)
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string();

    let dropdown_button_bundle = dropdown_button_bundle(DropdownButtonBundleOptions {
        text: initial_text,
        text_font: text_font.clone(),
        button_node: button_node.clone(),
        button_text_node,
    });

    let dropdown_list_bundle = dropdown_list_bundle(DropdownListBundleOptions {
        options: options.clone(),
        selected,
        list_node,
        button_node,
        text_font,
    });

    (
        DropdownContainer { selected, options },
        container_node,
        children![dropdown_button_bundle, dropdown_list_bundle],
    )
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

#[allow(clippy::type_complexity)]
fn dropdown_button_icon_system(
    list_query: Query<(&ChildOf, &Visibility), (Changed<Visibility>, With<DropdownList>)>,
    container_query: Query<&Children, With<DropdownContainer>>,
    button_query: Query<&Children, With<DropdownButton>>,
    mut button_icon_query: Query<&mut Text, With<DropdownButtonIcon>>,
) {
    for (list_childof, list_visibility) in list_query.iter() {
        // Get the container and its children
        let container_id = list_childof.parent();
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
    button_query: Query<(&Interaction, &ChildOf), With<DropdownButton>>,
    container_query: Query<&Children, With<DropdownContainer>>,
    mut list_query: Query<&mut Visibility, With<DropdownList>>,
) {
    if buttons.get_just_pressed().len() == 0 {
        return;
    }
    for (&button_interaction, button_childof) in button_query.iter() {
        // Get the list
        let container_id = button_childof.parent();
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
        (&Interaction, &ChildOf, &DropdownListItem, &Children),
        Changed<Interaction>,
    >,
    list_query: Query<(&ChildOf, &Children), With<DropdownList>>,
    mut container_query: Query<&mut DropdownContainer>,
    previous_list_item_query: Query<&Children, With<DropdownListItem>>,
    mut list_item_icon_query: Query<&mut Text, With<DropdownListItemIcon>>,
) {
    for (interaction, childof, list_item, interacted_list_item_children) in
        interacted_list_item_query.iter()
    {
        if *interaction != Interaction::Pressed {
            continue;
        }
        // Find the dropdown container for this pressed option
        let list_id = childof.parent();
        let (dropdown_list_childof, list_items) = list_query.get(list_id).unwrap();
        let dropdown_list_parent_id = dropdown_list_childof.parent();
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

#[allow(clippy::type_complexity)]
fn dropdown_list_position_system(
    button_query: Query<
        (&ChildOf, &Node),
        (Changed<Node>, With<DropdownButton>, Without<DropdownList>),
    >,
    container_query: Query<&Children, With<DropdownContainer>>,
    mut list_query: Query<&mut Node, With<DropdownList>>,
) {
    for (button_childof, button_node) in button_query.iter() {
        let container_id = button_childof.parent();
        let container_children = container_query.get(container_id).unwrap();
        let mut list_style = list_query.get_mut(container_children[1]).unwrap();
        list_style.top = button_node.height;
    }
}

// TODO - System to change focus back to dropdown button after clicking a list option?
