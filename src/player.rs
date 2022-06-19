use crate::actions::Actions;
use crate::loading::SpriteAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

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
                .with_system(animate_sprite),
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
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Player)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
