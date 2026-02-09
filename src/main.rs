use bevy::prelude::*;

mod animation;
mod audio;
mod camera;
mod fonts;
mod game_timer;
mod input;
mod pause;
mod textures;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Jam 7".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins((
        camera::CameraPlugin,
        input::InputPlugin,
        textures::TexturesPlugin,
        audio::AudioPlugin,
        fonts::FontsPlugin,
        ui::UiPlugin,
        animation::SpriteAnimationPlugin,
        pause::PausePlugin,
    ));

    app.run();
}
