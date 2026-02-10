use bevy::ecs::component::Component;

mod plugin;

pub(super) use plugin::PlayerPlugin;

#[derive(Component)]
pub struct Player;
