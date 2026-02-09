use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_pause_event);
    }
}

#[derive(Event)]
pub enum PauseEvent {
    Pause,
    Resume,
}

fn on_pause_event(pause: On<PauseEvent>, mut time: ResMut<Time<Virtual>>) {
    match pause.event() {
        PauseEvent::Pause => {
            time.pause();
        }
        PauseEvent::Resume => {
            time.unpause();
        }
    }
}
