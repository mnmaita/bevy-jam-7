use bevy::prelude::*;
use bevy_kira_audio::SpatialAudioReceiver;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);

        app.add_systems(
            FixedPostUpdate,
            (
                follow_target,
                constrain_camera_to_world_bounds
                    .after(follow_target)
                    .run_if(resource_exists::<WorldBounds>),
            ),
        );
    }
}

#[derive(Component, Default)]
#[require(Camera2d, Msaa::Off)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct MainCameraTarget;

/// Represents the minimum and maximum corner points of the world, in pixels.
#[derive(Resource, Debug)]
pub struct WorldBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl WorldBounds {
    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }
}

impl Default for WorldBounds {
    fn default() -> Self {
        Self {
            min: Vec2::splat(f32::INFINITY),
            max: Vec2::splat(f32::NEG_INFINITY),
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((MainCamera, SpatialAudioReceiver));
}

fn follow_target(
    main_camera: Single<&mut Transform, With<MainCamera>>,
    target: Single<&Transform, (Without<MainCamera>, With<MainCameraTarget>)>,
    time: Res<Time>,
) {
    const DELTA_TIME_RATE: f32 = 4.0;
    let mut camera_transform = main_camera.into_inner();
    let delta_time = time.delta_secs() * DELTA_TIME_RATE;
    let target_transform = target.into_inner();
    let target_pos = target_transform.translation.xy();
    let direction = target_pos.extend(camera_transform.translation.z);
    camera_transform.translation = camera_transform.translation.lerp(direction, delta_time);
}

fn constrain_camera_to_world_bounds(
    main_camera: Single<(&mut Transform, &Camera, &Projection), With<MainCamera>>,
    world_bounds: Res<WorldBounds>,
) {
    let (mut transform, camera, projection) = main_camera.into_inner();

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let half_viewport_size = viewport_size / 2.0;
        let scale = match projection {
            Projection::Orthographic(orthographic) => orthographic.scale,
            _ => 1.0,
        };
        let boundary = Rect::from_corners(
            world_bounds.min() + half_viewport_size * scale,
            world_bounds.max() - half_viewport_size * scale,
        );

        if !boundary.is_empty() && !boundary.contains(transform.translation.xy()) {
            let clamped_pos = transform.translation.xy().clamp(boundary.min, boundary.max);
            transform.translation = clamped_pos.extend(transform.translation.z);
        }
    }
}
