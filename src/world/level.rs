use crate::object::ObjectDesc;
use crate::world::tile::TileMap;
use crate::world::{self, grid::Grid};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>()
            .add_systems(PreUpdate, control_load_level)
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

fn control_load_level(mut events: EventWriter<LoadLevelEvent>, keyboard: Res<Input<KeyCode>>) {
    if !keyboard.just_pressed(KeyCode::R) {
        return;
    }

    events.send(LoadLevelEvent::new("scn/test-level.ron"));
}

fn handle_load_level_event(
    mut commands: Commands,
    mut events: EventReader<LoadLevelEvent>,
    tilemap: Query<Entity, With<TileMap>>,
    grid: Res<Grid>,
) {
    if events.is_empty() {
        return;
    }

    let mut path = String::new();

    let mut first = true;
    for e in events.iter() {
        if !first {
            warn!("Encountered unhandled LoadLevelEvent.");
            continue;
        }

        path = e.path.to_string();

        first = false;
    }

    for e in &tilemap {
        commands.entity(e).despawn_recursive();
    }

    world::generate_tiles(&grid, &mut commands);
}
