use bevy::{
    input_focus::{InputFocus, InputFocusVisible},
    math::CompassOctant,
    picking::hover::Hovered,
    prelude::*,
    ui::auto_directional_navigation::AutoDirectionalNavigator,
    ui_widgets::Activate,
};
use bevy_enhanced_input::prelude::Start;

use crate::input::actions::ui::{Navigate, Select};

pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            highlight_focused_element
                .run_if(resource_changed::<InputFocus>.or(resource_changed::<InputFocusVisible>)),
        );

        app.add_observer(navigate);
        app.add_observer(select);
        app.add_observer(on_add_first_navigable_node);
    }
}

#[derive(Component)]
pub struct FirstNavigableNode;

fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut BorderColor), With<Hovered>>,
) {
    todo!("Update highlighting style of focused element");
    const FOCUSED_BORDER_COLOR: Color = Color::BLACK;

    for (entity, mut border_color) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            *border_color = BorderColor::all(FOCUSED_BORDER_COLOR);
        } else {
            *border_color = BorderColor::DEFAULT;
        }
    }
}

fn navigate(navigate: On<Start<Navigate>>, mut navigator: AutoDirectionalNavigator) {
    if let Some(direction) = Dir2::new(navigate.value).ok().map(CompassOctant::from) {
        let _ = navigator.navigate(direction);
    }
}

fn select(_: On<Start<Select>>, input_focus: Res<InputFocus>, mut commands: Commands) {
    if let Some(entity) = input_focus.0 {
        commands.trigger(Activate { entity });
    }
}

fn on_add_first_navigable_node(
    add: On<Add, FirstNavigableNode>,
    mut input_focus: ResMut<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
) {
    if input_focus_visible.0 {
        input_focus.set(add.entity);
    }
}
