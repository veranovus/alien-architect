use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};
use serde::{Deserialize, Serialize};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {}
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
pub enum AudioMode {
    Once,
    Loop,
    Despawn,
}

#[derive(Debug, Component)]
pub struct AudioPlayer;

/************************************************************
 * - Service Functions
 */

pub fn spawn_audio_player(commands: &mut Commands) -> Entity {
    return commands.spawn_empty().id();
}

/************************************************************
 * - System Functions
 */
