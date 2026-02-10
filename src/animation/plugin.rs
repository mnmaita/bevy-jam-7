use core::time::Duration;

use bevy::prelude::*;

use super::{
    SpriteAnimation, SpriteAnimationDirection, SpriteAnimationEndEvent,
    SpriteAnimationFrameChangeEvent, SpriteAnimationFrameChangeEventsEnabled,
    SpriteAnimationStopped, SpriteAnimationTimer,
};

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);

        app.add_observer(on_insert_sprite_animation);
    }
}

fn animate_sprite(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut SpriteAnimationTimer,
            &mut Sprite,
            &mut SpriteAnimation,
            Has<SpriteAnimationFrameChangeEventsEnabled>,
        ),
        Without<SpriteAnimationStopped>,
    >,
    time: Res<Time>,
) {
    for (entity, mut timer, mut sprite, mut sprite_animation, events_enabled) in &mut query {
        if sprite_animation.is_last_frame() {
            if sprite_animation.despawn_on_finish() {
                commands.entity(entity).try_despawn();
                continue;
            }

            if sprite_animation.is_ping_pong() {
                sprite_animation.direction = sprite_animation.direction.reverse();
            }

            sprite_animation.reset();
            continue;
        }

        if !timer.tick(time.delta()).just_finished() {
            continue;
        }

        sprite.flip_x = sprite_animation.flip_x;

        if let Some(ref mut texture_atlas) = sprite.texture_atlas {
            let next_frame = match sprite_animation.direction {
                SpriteAnimationDirection::Forward => sprite_animation.next(),
                SpriteAnimationDirection::Backward => sprite_animation.next_back(),
            };

            if let Some((.., frame)) = next_frame {
                texture_atlas.index = frame.index();
                timer.set_duration(Duration::from_secs_f32(frame.duration_secs()));
                timer.reset();

                if events_enabled {
                    commands.trigger(SpriteAnimationFrameChangeEvent {
                        entity,
                        index: texture_atlas.index,
                    });

                    if sprite_animation.is_last_frame() {
                        commands.trigger(SpriteAnimationEndEvent { entity });
                    }
                }
            }
        }
    }
}

fn on_insert_sprite_animation(
    insert: On<Insert, SpriteAnimation>,
    mut query: Query<(&mut Sprite, &mut SpriteAnimationTimer, &mut SpriteAnimation)>,
) {
    if let Ok((mut sprite, mut timer, mut sprite_animation)) = query.get_mut(insert.entity) {
        sprite.flip_x = sprite_animation.flip_x;

        if let Some(ref mut texture_atlas) = sprite.texture_atlas {
            let next_frame = match sprite_animation.direction {
                SpriteAnimationDirection::Forward => sprite_animation.next(),
                SpriteAnimationDirection::Backward => sprite_animation.next_back(),
            };

            // We assume the frame vec has at least one element and the `next` methods will always yield an item.
            if let Some((.., frame)) = next_frame {
                texture_atlas.index = frame.index();
                timer.set_duration(Duration::from_secs_f32(frame.duration_secs()));
                timer.reset();
            }
        }
    }
}
