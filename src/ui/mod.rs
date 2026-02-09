use bevy::prelude::*;

pub mod navigation;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((navigation::UiNavigationPlugin,));
    }
}
