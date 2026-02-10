use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    game::player::actions::{Interact, Walk},
    input::actions::ui,
};

use super::Player;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<Player>();

        app.add_observer(bind_player_actions);
    }
}

fn bind_player_actions(add: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .insert(Actions::<Player>::spawn((
            // We bind Pause to the Player context because the UiContext is only enabled in menus.
            Spawn((
                Action::<ui::Pause>::new(),
                ActionSettings {
                    require_reset: true,
                    ..default()
                },
                bindings![KeyCode::Escape, GamepadButton::Start],
            )),
            Spawn((
                Action::<Walk>::new(),
                Bindings::spawn((
                    Axial::new(GamepadAxis::LeftStickX, Binding::None).with((
                        DeadZone {
                            lower_threshold: 0.1,
                            ..Default::default()
                        },
                        SmoothNudge::default(),
                    )),
                    Bidirectional::new(KeyCode::ArrowRight, KeyCode::ArrowLeft),
                    Bidirectional::new(KeyCode::KeyD, KeyCode::KeyA),
                )),
            )),
            Spawn((
                Action::<Interact>::new(),
                bindings![GamepadButton::South, KeyCode::KeyE],
            )),
        )));
}
