use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

use crate::actions::Actions;
use crate::loading::AudioAssets;
use crate::GameState;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(control_flying_sound),
            );
    }
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.3);
    // TODO: don't use this sound for movement, lol
    audio.play_looped(audio_assets.flying.clone());
    audio.pause();
}

fn control_flying_sound(actions: Res<Actions>, audio: Res<Audio>) {
    if actions.player_x_movement.is_some() {
        audio.resume();
    } else {
        audio.pause()
    }
}
