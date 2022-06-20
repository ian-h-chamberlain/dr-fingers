use bevy::prelude::*;
use heron::prelude::*;

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
        vec![Empty; 16],
        vec![Empty; 16],
        vec![Empty; 16],
        vec![Empty, Empty, Empty, Empty, Floor(Standalone), ],
        vec![Empty; 16],
        vec![Empty; 16],
        vec![Empty; 16],
        vec![Empty, Floor(TopLeft), Floor(Top), Floor(Top), Floor(Top), Floor(Top), Floor(Top), Floor(Top), Floor(Top), Floor(TopRight), Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        vec![Empty, Floor(BotLeft), Floor(Bot), Floor(Bot), Floor(Bot), Floor(Bot), Floor(Bot), Floor(Bot), Floor(Bot), Floor(BotRight),  Empty, Empty, Empty, Empty, Empty, Empty, Empty],
        vec![Empty; 16],
        vec![Empty, Empty, Floor(Left), Floor(Middle), Floor(Middle), Floor(Middle), Floor(Right)],
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
