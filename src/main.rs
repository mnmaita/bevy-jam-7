use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Jam 7".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.run();
}
