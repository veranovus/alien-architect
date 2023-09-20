use crate::object::ObjectID;
use crate::world::grid::{Grid, GridPlugin};
use crate::world::level::LevelPlugin;
use crate::world::tile::{TileMap, TilePlugin};
use bevy::prelude::*;

pub mod grid;
mod level;
mod tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin)
            .add_plugins(TilePlugin)
            .add_plugins(LevelPlugin)
            .add_systems(Startup, setup_world);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
pub struct World {
    size: (u32, u32),
    objects: Vec<Option<(Entity, ObjectID)>>,
}

impl World {
    fn new(grid: &Grid) -> Self {
        Self {
            size: grid.size,
            objects: vec![None; (grid.size.0 * grid.size.1) as usize],
        }
    }
}

/************************************************************
 * - System Functions
 */

fn setup_world(mut commands: Commands, grid: Res<Grid>) {
    commands.insert_resource(World::new(&grid));
}

/************************************************************
 * - Helper Functions
 */

pub fn generate_tiles(grid: &Grid, commands: &mut Commands) {
    let cc = grid.size.0 * grid.size.1;

    let mut tiles = vec![];

    for i in 0..cc {
        let x = i % grid.size.0;
        let y = i / grid.size.0;

        let tile = tile::Tile::new(
            i as usize, //
            UVec2::new(x, y),
            grid,
            commands,
        );

        tiles.push(tile);
    }

    TileMap::new(&tiles, commands);
}
