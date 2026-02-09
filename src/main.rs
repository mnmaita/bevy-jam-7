use bevy::prelude::*;

use crate::{animation::SpriteAnimationPlugin, audio::AudioPlugin};

mod animation;
mod audio;
mod game_timer;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Jam 7".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins((SpriteAnimationPlugin, AudioPlugin));

    app.run();
}
