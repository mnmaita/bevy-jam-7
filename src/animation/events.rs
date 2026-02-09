use bevy::prelude::{Component, Entity, EntityEvent};

#[derive(EntityEvent)]
pub struct SpriteAnimationFrameChangeEvent {
    pub entity: Entity,
    pub index: usize,
}

#[derive(EntityEvent)]
pub struct SpriteAnimationEndEvent {
    pub entity: Entity,
}

#[derive(Component, Default)]
pub struct SpriteAnimationFrameChangeEventsEnabled;
