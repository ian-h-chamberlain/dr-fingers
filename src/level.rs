use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::{log, prelude::*};
use heron::prelude::*;

use crate::loading::{MapAssets, TileAssets};
use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Level>()
            .add_asset_loader(LevelLoader)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_level));
    }
}

#[derive(Component, Clone, Copy)]
pub enum Tile {
    Empty,
    Floor(Side),
}

impl Default for Tile {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Side {
    // Height=2 blocks
    TopLeft,
    Top,
    TopRight,
    BotLeft,
    Bot,
    BotRight,

    // Single-layer blocks
    Left,
    Middle,
    Right,

    // Single vent block
    Standalone,
}

impl Side {
    fn index(self) -> usize {
        match self {
            Self::TopLeft => 0,
            Self::Top => 1,
            Self::TopRight => 2,
            Self::BotLeft => 6,
            Self::Bot => 7,
            Self::BotRight => 8,
            Self::Left => 3,
            Self::Middle => 4,
            Self::Right => 5,
            Self::Standalone => 9,
        }
    }
}

const LEVEL_WIDTH: usize = 20;
const LEVEL_HEIGHT: usize = 14;

#[derive(TypeUuid)]
#[uuid = "e89843e3-8db2-4467-ae09-196b0bb31aa9"]
pub struct Level {
    tiles: Vec<Vec<Tile>>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            tiles: vec![vec![Tile::Empty; LEVEL_WIDTH]; LEVEL_HEIGHT],
        }
    }
}

struct LevelLoader;

impl LevelLoader {
    async fn load_level<'a>(
        bytes: &'a [u8],
        load_context: &'a mut LoadContext<'_>,
    ) -> Result<(), anyhow::Error> {
        let mut level = Level::default();
        let input_str = std::str::from_utf8(bytes)?;

        log::info!("Loading level from string");

        for (j, row) in input_str.lines().enumerate() {
            if j >= LEVEL_HEIGHT {
                return Err(anyhow::format_err!("height larger than max {LEVEL_HEIGHT}"));
            }

            for (i, c) in row.chars().enumerate() {
                if i >= LEVEL_WIDTH {
                    return Err(anyhow::format_err!("width larger than max {LEVEL_WIDTH}"));
                }

                level.tiles[j][i] = match c {
                    '[' => Tile::Floor(Side::Left),
                    '=' => Tile::Floor(Side::Middle),
                    ']' => Tile::Floor(Side::Right),
                    '¬' => Tile::Floor(Side::TopRight),
                    '4' => Tile::Floor(Side::TopLeft),
                    '-' => Tile::Floor(Side::Top),
                    'L' => Tile::Floor(Side::BotLeft),
                    '_' => Tile::Floor(Side::Bot),
                    '/' => Tile::Floor(Side::BotRight),
                    '•' => Tile::Floor(Side::Standalone),
                    _ => Tile::Empty,
                };
            }
        }

        load_context.set_default_asset(LoadedAsset::new(level));
        Ok(())
    }
}

impl AssetLoader for LevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(Self::load_level(bytes, load_context))
    }

    fn extensions(&self) -> &[&str] {
        &["lvl"]
    }
}

// TODO: maybe support hot-reloading the map, could be useful for iteration
fn spawn_level(
    mut commands: Commands,
    maps: Res<MapAssets>,
    level_assets: Res<Assets<Level>>,
    tiles: Res<TileAssets>,
    windows: Res<Windows>,
) {
    let window = windows.primary();
    let tile_start = Vec3::new(-window.width() / 2.0, window.height() / 2.0, 10.0);

    let level0 = level_assets.get(&maps.level0).unwrap();

    for (j, row) in level0.tiles.iter().enumerate() {
        for (i, &tile) in row.iter().enumerate() {
            if let Tile::Floor(side) = tile {
                let position = tile_start + Vec3::new(i as f32 * 48.0, j as f32 * -48.0, 0.0);

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(side.index()),
                        texture_atlas: tiles.tiles.clone(),
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    })
                    .insert(tile)
                    .insert(RigidBody::Static)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(24.0, 24.0, 0.0),
                        border_radius: None,
                    });
            }
        }
    }
}
