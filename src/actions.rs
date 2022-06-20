use bevy::prelude::*;

use crate::GameState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(set_movement_actions),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_x_movement: Option<f32>,
    pub player_jump: bool,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if GameControl::Jump.just_released(&keyboard_input)
        || GameControl::Jump.pressed(&keyboard_input)
    {
        actions.player_jump = true;
    } else {
        actions.player_jump = false;
    }

    if GameControl::Left.pressed(&keyboard_input)
        || GameControl::Left.pressed(&keyboard_input)
        || GameControl::Right.just_released(&keyboard_input)
        || GameControl::Right.pressed(&keyboard_input)
    {
        let player_movement;
        if GameControl::Right.just_released(&keyboard_input)
            || GameControl::Left.just_released(&keyboard_input)
        {
            if GameControl::Right.pressed(&keyboard_input) {
                player_movement = 1.0;
            } else if GameControl::Left.pressed(&keyboard_input) {
                player_movement = -1.0;
            } else {
                player_movement = 0.0;
            }
        } else if GameControl::Right.just_pressed(&keyboard_input) {
            player_movement = 1.0;
        } else if GameControl::Left.just_pressed(&keyboard_input) {
            player_movement = -1.0;
        } else {
            player_movement = actions.player_x_movement.unwrap_or(0.0);
        }

        if player_movement != 0.0 {
            actions.player_x_movement = Some(player_movement);
        }
    } else {
        actions.player_x_movement = None;
    }
}

enum GameControl {
    Jump,
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Jump => keyboard_input.just_released(KeyCode::Space),
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Jump => keyboard_input.pressed(KeyCode::Space),
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Jump => keyboard_input.just_pressed(KeyCode::Space),
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
