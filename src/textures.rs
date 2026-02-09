use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub const ASSET_FOLDER_TEXTURES: &str = "textures";

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TexturesLoadState>();
        app.init_resource::<TextureHandles>();
        app.add_systems(Startup, load_textures);
        app.add_systems(
            Update,
            update_texture_assets_load_state.run_if(not(TexturesLoadState::loaded)),
        );
    }
}

#[derive(Resource, Deref)]
pub struct TexturesLoadState(RecursiveDependencyLoadState);

impl TexturesLoadState {
    pub fn loaded(load_state: Res<Self>) -> bool {
        load_state.is_loaded()
    }
}

impl Default for TexturesLoadState {
    fn default() -> Self {
        Self(RecursiveDependencyLoadState::NotLoaded)
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct TextureHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<Image>>,
);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_TEXTURES)
        }
        #[cfg(target_family = "wasm")]
        {
            let asset_textures_list = [
                // format!("{ASSET_FOLDER_TEXTURES}/example.png"),
            ];
            asset_textures_list
                .iter()
                .map(|path| asset_server.load::<Image>(path))
                .collect::<Vec<Handle<Image>>>()
        }
    };

    commands.insert_resource(TextureHandles(texture_handles));
}

fn update_texture_assets_load_state(
    mut textures_load_state: ResMut<TexturesLoadState>,
    texture_handles: Res<TextureHandles>,
    asset_server: Res<AssetServer>,
) {
    textures_load_state.0 = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.recursive_dependency_load_state(texture_handles.id())
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = texture_handles.iter().all(|handle| {
                asset_server
                    .recursive_dependency_load_state(handle.id())
                    .is_loaded()
            });
            if all_loaded {
                RecursiveDependencyLoadState::Loaded
            } else {
                RecursiveDependencyLoadState::NotLoaded
            }
        }
    };
}
