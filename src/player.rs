use std::f32::consts;

use bevy::prelude::*;
use heron::prelude::*;

use crate::actions::Actions;
use crate::level;
use crate::loading::SpriteAssets;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCollider;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player)
                .with_system(spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                .with_system(animate_player),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: sprites.dogken.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Dynamic)
        .insert(Collisions::default())
        // .insert(PhysicMaterial {
        //     restitution: 0.2,
        //     density: 1.0,
        //     // high friction since we are not actually trying to simulate sliding.
        //     friction: 1.0,
        // })
        .insert(RotationConstraints::lock())
        .insert(Velocity::default())
        .with_children(|commands| {
            commands
                .spawn()
                .insert(PlayerCollider)
                // TODO: maybe trapezoidal convex hull instead?
                .insert(CollisionShape::Capsule {
                    half_segment: 14.0,
                    radius: 16.0,
                })
                .insert_bundle(TransformBundle::from_transform(
                    Transform::from_translation(Vec3::Y * 2.5)
                        .with_rotation(Quat::from_rotation_z(consts::PI / 2.0))
                        .with_scale(Vec3::splat(10.0)),
                ));
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_player(
    time: Res<Time>,
    actions: Res<Actions>,
    atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
) {
    for (mut timer, mut sprite, atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if let Some(movement) = actions.player_x_movement {
                let atlas = atlases
                    .get(atlas_handle)
                    .expect("atlas for texture not found");

                if movement > 0.0 {
                    sprite.index = (sprite.index + 1) % atlas.textures.len();
                } else if movement < 0.0 {
                    sprite.index = sprite
                        .index
                        .checked_sub(1)
                        .unwrap_or(atlas.textures.len() - 1);
                }
            }
        }
    }
}

const MOVE_ACCEL: f32 = 30.0;
const MAX_SPEED_X: f32 = 175.0;
const JUMP_VELOCITY: f32 = 200.0;

fn move_player(
    actions: Res<Actions>,
    mut player_query: Query<(&mut Velocity, &Collisions), With<Player>>,
    tiles_query: Query<&level::Tile>,
) {
    for (mut player_vel, collisions) in player_query.iter_mut() {
        let mut on_floor = false;

        for collided_entity in collisions.entities() {
            use level::{Side::*, Tile};

            // kinda hacky, but only allow movement/jumping from "top" tiles
            if let Ok(Tile::Floor(Top | TopLeft | TopRight)) = tiles_query.get(collided_entity) {
                if actions.player_jump {
                    player_vel.linear.y = JUMP_VELOCITY;
                }

                on_floor = true;
                break;
            }
        }

        // TODO: movement should be less powerful in the air, to counteract friction?
        if let Some(movement) = actions.player_x_movement {
            player_vel.linear.x += movement * MOVE_ACCEL * if on_floor { 1.0 } else { 0.5 };
        }

        // damp + clamp
        player_vel.linear.x -= 2.0 * player_vel.linear.x.signum();
        player_vel.linear.x = player_vel.linear.x.clamp(-MAX_SPEED_X, MAX_SPEED_X);
    }
}
