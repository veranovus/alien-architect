use crate::{
    object::asset::ObjectAssetServer,
    object::{Object, ObjectDesc},
    player::UFO,
    state::AppState,
    world::{
        self,
        grid::Grid,
        tile::{TileMap, TileStateChangeEvent},
    },
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_level)
            .add_systems(OnEnter(AppState::Game), load_level)
            .add_systems(OnExit(AppState::Game), unload_level);
    }
}

/************************************************************
 * - Constants
 */

const LEVEL_PATHS: [&str; 1] = ["assets/scn/test-level.ron"];

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
struct LevelDesc {
    objects: Vec<ObjectDesc>,
}

#[allow(dead_code)]
#[derive(Debug, Resource)]
pub struct Level {
    current: usize,
    maximum: usize,
}

impl Level {
    fn new(current: usize) -> Self {
        Self {
            current,
            maximum: LEVEL_PATHS.len(),
        }
    }
}

/************************************************************
 * - System Functions
 */

fn setup_level(mut commands: Commands) {
    commands.insert_resource(Level::new(0));
}

fn load_level(
    mut commands: Commands,
    mut event_writer: EventWriter<TileStateChangeEvent>,
    mut world: ResMut<world::World>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    oas: Res<ObjectAssetServer>,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    grid: Res<Grid>,
) {
    let level_desc: LevelDesc =
        if let Ok(contents) = std::fs::read_to_string(LEVEL_PATHS[level.current]) {
            ron::from_str(&contents).unwrap()
        } else {
            panic!(
                "Failed to load LevelDesc from, `{}`.",
                LEVEL_PATHS[level.current]
            );
        };

    world::generate_tiles(&grid, &mut commands);

    world::generate_objects(
        &level_desc.objects,
        &grid,
        &oas,
        &mut texture_atlases,
        &mut world,
        &mut commands,
    );

    UFO::new(
        IVec2::new(1, 6),
        &grid,
        &asset_server,
        &mut commands,
        &mut texture_atlases,
        &mut event_writer,
    );
}

fn unload_level(
    mut commands: Commands,
    tilemap: Query<Entity, With<TileMap>>,
    objects: Query<Entity, With<Object>>,
    ufo: Query<Entity, With<UFO>>,
) {
    // De-spawn TileMap
    for e in &tilemap {
        commands.entity(e).despawn_recursive();
    }

    // De-spawn Objects
    for e in &objects {
        commands.entity(e).despawn_recursive();
    }

    // De-spawn UFO
    for e in &ufo {
        commands.entity(e).despawn_recursive();
    }
}
