use std::{iter::Enumerate, vec::IntoIter};

use bevy::{
    ecs::{
        component::{Mutable, StorageType},
        lifecycle::{ComponentHook, HookContext},
    },
    prelude::*,
};

use crate::game_timer::GameTimer;

pub struct SpriteAnimation {
    current_frame: Option<(usize, SpriteAnimationFrame)>,
    despawn_on_finish: bool,
    pub direction: SpriteAnimationDirection,
    pub flip_x: bool,
    frames_iter: Enumerate<IntoIter<SpriteAnimationFrame>>,
    frames: Vec<SpriteAnimationFrame>,
    ping_pong: bool,
}

impl Component for SpriteAnimation {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut world, HookContext { entity, .. }| {
            if let Some(sprite_animation) = world.get::<Self>(entity) {
                let first_frame = match sprite_animation.direction {
                    SpriteAnimationDirection::Forward => {
                        sprite_animation.frames.first().map(|frame| (0, *frame))
                    }
                    SpriteAnimationDirection::Backward => sprite_animation
                        .frames
                        .last()
                        .map(|frame| (sprite_animation.frames.len() - 1, *frame)),
                };

                if let Some((.., frame)) = first_frame {
                    let mut commands = world.commands();
                    let timer = SpriteAnimationTimer::from_seconds(frame.duration_secs);
                    commands.entity(entity).insert(timer);
                }
            }
        })
    }

    fn on_remove() -> Option<ComponentHook> {
        Some(|mut world, HookContext { entity, .. }| {
            let mut commands = world.commands();
            commands.entity(entity).remove::<SpriteAnimationTimer>();
        })
    }
}

impl SpriteAnimation {
    pub fn new(frames: impl Into<Vec<SpriteAnimationFrame>>) -> Self {
        let frames = frames.into();
        let frames_iter = frames.clone().into_iter().enumerate();

        Self {
            current_frame: None,
            despawn_on_finish: false,
            direction: SpriteAnimationDirection::default(),
            flip_x: false,
            frames_iter,
            frames,
            ping_pong: false,
        }
    }

    pub fn despawn_on_finish(&self) -> bool {
        self.despawn_on_finish
    }

    pub fn is_ping_pong(&self) -> bool {
        self.ping_pong
    }

    /// Advances the iterator and returns the next frame's order and atlas index.
    pub fn next(&mut self) -> Option<(usize, SpriteAnimationFrame)> {
        self.current_frame = self.frames_iter.next();
        self.current_frame
    }

    /// Advances the iterator from the back and returns the next frame's order and atlas index.
    pub fn next_back(&mut self) -> Option<(usize, SpriteAnimationFrame)> {
        self.current_frame = self.frames_iter.next_back();
        self.current_frame
    }

    /// Resets this [`SpriteAnimation`] frames iterator, "rewinding" the animation.
    pub fn reset(&mut self) {
        self.frames_iter = self.frames.clone().into_iter().enumerate();
        self.current_frame = None;
    }

    pub fn is_last_frame(&mut self) -> bool {
        match self.direction {
            SpriteAnimationDirection::Forward => self
                .current_frame
                .is_some_and(|(i, ..)| i == self.frames.len() - 1),
            SpriteAnimationDirection::Backward => self.current_frame.is_some_and(|(i, ..)| i == 0),
        }
    }

    pub fn with_despawn_on_finish(mut self) -> Self {
        self.despawn_on_finish = true;
        self
    }

    pub fn with_direction(mut self, direction: SpriteAnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_flip_x(mut self) -> Self {
        self.flip_x = true;
        self
    }

    pub fn with_ping_pong(mut self) -> Self {
        self.ping_pong = true;
        self
    }
}

#[derive(Component, Default)]
pub struct SpriteAnimationStopped;

#[derive(Default, Clone, Copy)]
pub enum SpriteAnimationDirection {
    #[default]
    Forward,
    Backward,
}

impl SpriteAnimationDirection {
    pub fn reverse(&self) -> Self {
        match self {
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct SpriteAnimationFrame {
    index: usize,
    duration_secs: f32,
}

impl SpriteAnimationFrame {
    pub fn new(index: usize, duration_secs: f32) -> Self {
        Self {
            duration_secs,
            index,
        }
    }

    pub fn duration_secs(&self) -> f32 {
        self.duration_secs
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl From<(usize, f32)> for SpriteAnimationFrame {
    fn from(value: (usize, f32)) -> Self {
        Self {
            duration_secs: value.1,
            index: value.0,
        }
    }
}

pub type SpriteAnimationTimer = GameTimer<SpriteAnimation>;
