use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::game::player::PlayerPlugin;

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayerPlugin)
    }
}
