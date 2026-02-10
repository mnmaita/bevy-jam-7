use bevy::ecs::component::Component;

pub mod actions;
mod input;
mod plugin;

pub(super) use input::*;
pub(super) use plugin::*;

#[derive(Component)]
pub struct Player;
