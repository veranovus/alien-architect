use crate::object::ObjectDesc;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>()
            .add_systems(PostUpdate, handle_load_level_event);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
struct LevelDesc {
    objects: Vec<ObjectDesc>,
}

#[derive(Debug, Event)]
pub struct LoadLevelEvent {
    path: String,
}

impl LoadLevelEvent {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

/************************************************************
 * - System Functions
 */

fn handle_load_level_event(events: EventReader<LoadLevelEvent>) {}
