use bevy::prelude::*;

use crate::animation::SpriteAnimation;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
        app.init_resource::<PlayerTextureAtlasLayout>();
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_layout: Res<PlayerTextureAtlasLayout>,
) {
    const FRAME_SPEED: f32 = 0.05;

    commands.spawn((
        Player,
        Sprite {
            image: asset_server.load("textures/bevyJam-player-running.png"),
            texture_atlas: Some(player_layout.clone().into()),
            ..Default::default()
        },
        SpriteAnimation::new([
            (0, FRAME_SPEED).into(),
            (1, FRAME_SPEED).into(),
            (2, FRAME_SPEED).into(),
            (3, FRAME_SPEED).into(),
            (4, FRAME_SPEED).into(),
            (5, FRAME_SPEED).into(),
            (6, FRAME_SPEED).into(),
            (7, FRAME_SPEED).into(),
            (8, FRAME_SPEED).into(),
            (9, FRAME_SPEED).into(),
            (10, FRAME_SPEED).into(),
            (11, FRAME_SPEED).into(),
            (12, FRAME_SPEED).into(),
            (13, FRAME_SPEED).into(),
            (14, FRAME_SPEED).into(),
            (15, FRAME_SPEED).into(),
        ]),
    ));
}

#[derive(Resource, Deref)]
struct PlayerTextureAtlasLayout(Handle<TextureAtlasLayout>);

impl FromWorld for PlayerTextureAtlasLayout {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let atlas = TextureAtlasLayout::from_grid(uvec2(16, 16), 16, 1, None, None);
        Self(asset_server.add(atlas))
    }
}
