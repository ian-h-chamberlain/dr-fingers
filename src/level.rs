use bevy::prelude::*;

use crate::loading::TileAssets;
use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>()
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(build_level))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_level));
    }
}

#[derive(Component, Clone, Copy)]
enum Tile {
    Empty,
    Floor(Side),
}

impl Default for Tile {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
enum Side {
    TopLeft,
    Top,
    TopRight,
    BotLeft,
    Bot,
    BotRight,
    // TODO maybe standalone blocks?
    // and probably will want some TopBot kind of tiles
}

impl Side {
    fn index(self) -> usize {
        match self {
            Self::TopLeft => 16,
            Self::Top => 17,
            Self::TopRight => 18,
            Self::BotLeft => 32,
            Self::Bot => 33,
            Self::BotRight => 34,
        }
    }
}

#[derive(Default)]
struct Level {
    tiles: Vec<Vec<Tile>>,
}

#[rustfmt::skip]
fn build_level(mut level: ResMut<Level>) {
    use Side::*;
    use Tile::*;

    // TODO maybe RON or even an ascii map or something a little nicer than this
    level.tiles = vec![
        vec![Empty; 16],
        vec![Empty, Floor(TopLeft), Floor(Top), Floor(TopRight), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        vec![Empty, Floor(BotLeft), Floor(Bot), Floor(BotRight), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        vec![Empty; 16],
    ];
}

fn spawn_level(
    mut commands: Commands,
    level: Res<Level>,
    tiles: Res<TileAssets>,
    windows: Res<Windows>,
) {
    let window = windows.primary();
    let tile_start = Vec3::new(-window.width() / 2.0, window.height() / 2.0, 10.0);

    for (j, row) in level.tiles.iter().enumerate() {
        for (i, tile) in row.iter().enumerate() {
            if let Tile::Floor(side) = tile {
                let position = tile_start + Vec3::new(i as f32 * 48.0, j as f32 * -48.0, 0.0);

                commands.spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(side.index()),
                    texture_atlas: tiles.tiles.clone(),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                });
            }
        }
    }
}
