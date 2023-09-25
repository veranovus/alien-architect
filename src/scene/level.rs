use crate::{
    object::asset::ObjectAssetServer,
    object::{Object, ObjectDesc},
    player::UFO,
    state::AppState,
    ui::game_ui::GameUINumberValue,
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
        app.add_systems(PreStartup, setup_resources)
            .add_systems(OnEnter(AppState::Game), load_level)
            .add_systems(OnExit(AppState::Game), unload_level);
    }
}

/************************************************************
 * - Constants
 */

const LEVEL_PATHS: [&str; 11] = [
    "assets/scn/level_0.ron",
    "assets/scn/level_1.ron",
    "assets/scn/level_2.ron",
    "assets/scn/level_3.ron",
    "assets/scn/level_4.ron",
    "assets/scn/level_5.ron",
    "assets/scn/level_6.ron",
    "assets/scn/level_7.ron",
    "assets/scn/level_8.ron",
    "assets/scn/level_9.ron",
    "assets/scn/test-level.ron",
];

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
struct LevelDesc {
    objects: Vec<ObjectDesc>,
}

#[derive(Debug, Resource)]
pub struct TurnCounter {
    pub turn: usize,
}

impl TurnCounter {
    fn new() -> Self {
        Self { turn: 0 }
    }
}

impl GameUINumberValue for TurnCounter {
    fn value(&self) -> usize {
        return self.turn;
    }
}

#[derive(Debug, Resource)]
pub struct Score {
    pub previous: usize,
    pub current: usize,
}

impl Score {
    fn new() -> Self {
        Self {
            previous: 0,
            current: 0,
        }
    }
}

impl GameUINumberValue for Score {
    fn value(&self) -> usize {
        return self.current;
    }
}

#[allow(dead_code)]
#[derive(Debug, Resource)]
pub struct Level {
    pub current: usize,
    pub maximum: usize,
}

impl Level {
    fn new(current: usize) -> Self {
        let maximum = LEVEL_PATHS.len();
        if current >= maximum {
            panic!("Supplied invalid Level number at startup.");
        }

        Self { current, maximum }
    }

    pub fn next(&mut self) -> AppState {
        self.current += 1;
        if self.current >= self.maximum {
            self.current = self.maximum - 1;

            return AppState::Splash;
        }

        return AppState::Game;
    }
}

impl GameUINumberValue for Level {
    fn value(&self) -> usize {
        return self.current;
    }
}

/************************************************************
 * - System Functions
 */

fn setup_resources(mut commands: Commands) {
    commands.insert_resource(Level::new(0));
    commands.insert_resource(Score::new());
    commands.insert_resource(TurnCounter::new());
}

fn load_level(
    mut commands: Commands,
    mut event_writer: EventWriter<TileStateChangeEvent>,
    mut world: ResMut<world::World>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut turn_counter: ResMut<TurnCounter>,
    mut score: ResMut<Score>,
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

    // Reset TurnCounter
    turn_counter.turn = 0;

    // Set Score
    score.previous = score.current;

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
