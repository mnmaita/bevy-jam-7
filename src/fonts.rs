use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub const ASSET_FOLDER_FONTS: &str = "fonts";

pub struct FontsPlugin;

impl Plugin for FontsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontsLoadState>();
        app.init_resource::<FontHandles>();
        app.add_systems(Startup, load_fonts);
        app.add_systems(
            Update,
            update_font_assets_load_state.run_if(not(FontsLoadState::loaded)),
        );
    }
}

#[derive(Resource, Deref)]
pub struct FontsLoadState(RecursiveDependencyLoadState);

impl FontsLoadState {
    pub fn loaded(load_state: Res<Self>) -> bool {
        load_state.is_loaded()
    }
}

impl Default for FontsLoadState {
    fn default() -> Self {
        Self(RecursiveDependencyLoadState::NotLoaded)
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct FontHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<Font>>,
);

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_FONTS)
        }
        #[cfg(target_family = "wasm")]
        {
            let asset_font_list = [
                // format!("{ASSET_FOLDER_FONTS}/example.ttf"),
            ];
            asset_font_list
                .iter()
                .map(|path| asset_server.load::<Font>(path))
                .collect::<Vec<Handle<Font>>>()
        }
    };

    commands.insert_resource(FontHandles(font_handles));
}

fn update_font_assets_load_state(
    mut fonts_load_state: ResMut<FontsLoadState>,
    font_handles: Res<FontHandles>,
    asset_server: Res<AssetServer>,
) {
    fonts_load_state.0 = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.recursive_dependency_load_state(font_handles.id())
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = font_handles.iter().all(|handle| {
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
