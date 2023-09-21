use crate::object::asset::ObjectAssetServer;
use crate::object::{Object, ObjectDesc};
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

    events.send(LoadLevelEvent::new("assets/scn/test-level.ron"));
}

fn handle_load_level_event(
    mut commands: Commands,
    mut events: EventReader<LoadLevelEvent>,
    tilemap: Query<Entity, With<TileMap>>,
    objects: Query<Entity, With<Object>>,
    oas: Res<ObjectAssetServer>,
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

    let level_desc: LevelDesc = if let Ok(contents) = std::fs::read_to_string(&path) {
        ron::from_str(&contents).unwrap()
    } else {
        panic!("Failed to load LevelDesc from, `{}`.", path);
    };

    // De-spawn TileMap
    for e in &tilemap {
        commands.entity(e).despawn_recursive();
    }

    // De-spawn Objects
    for e in &objects {
        commands.entity(e).despawn_recursive();
    }

    world::generate_tiles(&grid, &mut commands);

    for obj_desc in &level_desc.objects {
        Object::new(obj_desc.id, obj_desc.position, &grid, &oas, &mut commands);
    }
}
