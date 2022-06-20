use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<SpriteAssets>()
            .with_collection::<TileAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(path = "sprites/dogken.png")]
    #[asset(texture_atlas(tile_size_x = 46.0, tile_size_y = 34.0, columns = 1, rows = 10))]
    pub dogken: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct TileAssets {
    #[asset(path = "textures/prison_tiles.png")]
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 6, rows = 2))]
    pub tiles: Handle<TextureAtlas>,
}
