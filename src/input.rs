use bevy::{
    ecs::system::SystemParam,
    input_focus::{
        InputDispatchPlugin, InputFocusVisible, directional_navigation::DirectionalNavigationPlugin,
    },
    prelude::*,
    window::PrimaryWindow,
};
use bevy_enhanced_input::{context::ExternallyMocked, prelude::*};

use crate::camera::MainCamera;

use actions::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputDispatchPlugin,
            DirectionalNavigationPlugin,
            bevy_enhanced_input::EnhancedInputPlugin,
        ));

        app.add_input_context::<ui::UiContext>();
        app.add_input_context::<GamepadContext>();

        app.insert_resource(InputFocusVisible(true));

        app.add_systems(Startup, bind_ui_actions);
    }
}

#[derive(Component, Default)]
pub struct GamepadContext;

pub mod actions {
    use super::{Component, InputAction, Vec2};

    #[derive(InputAction)]
    #[action_output(Vec2)]
    pub struct GamepadMovement;

    pub mod ui {
        use super::*;

        #[derive(Component, Default)]
        pub struct UiContext;

        #[derive(InputAction)]
        #[action_output(Vec2)]
        pub struct Navigate;

        #[derive(InputAction)]
        #[action_output(bool)]
        pub struct Select;

        #[derive(InputAction)]
        #[action_output(bool)]
        pub struct Back;

        #[derive(InputAction)]
        #[action_output(bool)]
        pub struct Pause;
    }
}

#[derive(SystemParam)]
pub struct Cursor<'w, 's> {
    window: Single<'w, 's, &'static Window, With<PrimaryWindow>>,
    main_camera: Single<'w, 's, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
}

impl Cursor<'_, '_> {
    pub fn world_position(&self) -> Option<Vec2> {
        self.window.cursor_position().and_then(|pos| {
            let (camera, transform) = *self.main_camera;
            camera.viewport_to_world_2d(transform, pos).ok()
        })
    }
}

fn bind_ui_actions(mut commands: Commands) {
    use ui::*;

    commands.spawn((
        UiContext,
        ContextPriority::<UiContext>::new(0),
        Actions::<UiContext>::spawn((
            Spawn((
                Action::<Back>::new(),
                ActionSettings {
                    require_reset: true,
                    ..default()
                },
                bindings![GamepadButton::East, KeyCode::Escape],
            )),
            Spawn((
                Action::<Navigate>::new(),
                Bindings::spawn((
                    Cardinal::new(
                        GamepadButton::DPadUp,
                        GamepadButton::DPadLeft,
                        GamepadButton::DPadDown,
                        GamepadButton::DPadRight,
                    ),
                    Cardinal::new(KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD),
                    Cardinal::new(
                        KeyCode::ArrowUp,
                        KeyCode::ArrowLeft,
                        KeyCode::ArrowDown,
                        KeyCode::ArrowRight,
                    ),
                )),
            )),
            Spawn((Action::<Select>::new(), bindings![GamepadButton::South])),
            Spawn((
                Action::<Pause>::new(),
                ActionSettings {
                    require_reset: true,
                    ..default()
                },
                bindings![KeyCode::Escape, GamepadButton::Start],
            )),
        )),
    ));
}

pub fn enable_context<C: Component>(mut commands: Commands, ctx: Single<Entity, With<C>>) {
    commands.entity(*ctx).insert(ContextActivity::<C>::ACTIVE);
}

pub fn disable_context<C: Component>(mut commands: Commands, ctx: Single<Entity, With<C>>) {
    commands.entity(*ctx).insert(ContextActivity::<C>::INACTIVE);
}

pub fn enable_action<A: InputAction + Send>(
    mut commands: Commands,
    actions: Query<Entity, With<Action<A>>>,
) {
    for entity in actions {
        commands.entity(entity).remove::<ExternallyMocked>();
    }
}

pub fn disable_action<A: InputAction + Send>(
    mut commands: Commands,
    actions: Query<Entity, With<Action<A>>>,
) {
    for entity in actions {
        commands.entity(entity).insert(ExternallyMocked);
    }
}

pub fn action_just_pressed<A: InputAction>(events: Query<&ActionEvents, With<Action<A>>>) -> bool {
    events.iter().any(|e| e.contains(ActionEvents::START))
}

pub fn context_inactive<C: Component>(context_activity: Single<&ContextActivity<C>>) -> bool {
    !***context_activity
}
