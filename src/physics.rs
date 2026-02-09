use avian2d::{
    PhysicsPlugins,
    prelude::{Gravity, PhysicsLayer},
};
use bevy::prelude::*;

/// Represents the pixels-per-meter unit for the physics engine.
const LENGTH_UNIT: f32 = 32.0;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(LENGTH_UNIT));

        #[cfg(debug_assertions)]
        app.add_plugins(avian2d::prelude::PhysicsDebugPlugin);

        app.insert_resource(Gravity::ZERO);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Interactable,
}
