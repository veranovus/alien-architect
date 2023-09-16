use crate::global;
use crate::world::tile;
use crate::world::tile::{Tile, TileMap};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_map);
        app.add_systems(Startup, load_map);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug)]
pub struct MapPoint {
    pub index: usize,
    // Always between the range -> (0, 99)
    pub order: usize,
    pub world_position: Vec2,
    pub grid_position: IVec2,
}

#[derive(Debug, Resource)]
pub struct Map {
    lrow: usize,
    srow: usize,
    row_count: usize,
    pub size: usize,
    pub origins: Vec<MapPoint>,
}

impl Map {
    fn new(lrow: usize, srow: usize, row_count: usize) -> Self {
        let mut map = Self {
            lrow,
            srow,
            row_count,
            size: 0,
            origins: vec![],
        };

        map.size = (map.lrow * map.row_count) + (map.srow * map.row_count);

        return map;
    }
}

/************************************************************
 * - System Functions
 */

fn setup_map(mut commands: Commands) {
    let mut map = Map::new(5, 4, 6);

    let line: usize = map.lrow + map.srow;

    // Full width of the sprite
    let fw = 30.0;
    // Half width of the sprite
    let hw = fw / 2.0;

    // Full height of the sprite
    let fh = 18.0;
    // Half height of the sprite
    let hh = fh / 2.0;
    // Transfer height: Distance from the center of one tile
    //                  to another tile in the same column.
    let fth = 14.0;
    // Half transfer height
    let hth = fth / 2.0;

    // Total size of the world area
    let area = (
        // Total width of the world tiles
        map.lrow as f32 * fw,
        // Total height for the world tiles
        fh + ((map.row_count - 1) as f32 * fth) + hth,
    );

    let offset = (
        (global::window::VIEWPORT_RESOLUTION.0 as f32 - area.0) / 2.0,
        (global::window::VIEWPORT_RESOLUTION.1 as f32 - area.1) / 2.0,
    );

    for i in 0..map.size {
        let xpos = (i % line) as f32 * hw;
        let ypos = ((i / line) as f32 * fth) + (((i % line) % 2) as f32 * hth);
        let zpos = ((i / line) * 9) + ((i % line) / 2) + (((i % line) % 2) * 4);

        map.origins.push(MapPoint {
            index: i,
            order: zpos,
            world_position: Vec2::new(
                (offset.0 + area.0) - (xpos + hw),
                (offset.1 + area.1) - (ypos + hh),
            ),
            grid_position: IVec2::new(0, 0),
        });
    }

    // Add Map as a resource
    commands.insert_resource(map);
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<Map>) {
    let mut tiles = vec![];

    for origin in &map.origins {
        tiles.push(Tile::new(true, &origin, &asset_server, &mut commands));
    }

    TileMap::new(&tiles, &mut commands);
}

/************************************************************
 * - Notes
 *
 * 6x 5
 * 5x 4
 *
 * Total Tile: 60
 *
 * 08 -- 06 -- 04 -- 02 -- 00
 * -- 07 -- 05 -- 03 -- 01 --
 * 17 -- 15 -- 13 -- 11 -- 09
 * -- 16 -- 14 -- 12 -- 10 --
 * 00 -- 00 -- 00 -- 00 -- 00
 * -- 00 -- 00 -- 00 -- 00 --
 *
 * I / 2 + I % 2 * 4
 *
 * I  => Index
 * HW => Half Width
 * FH => Full Height
 * HH => Half Height
 *
 * Rule -> (I % 9 * HW)
 * Rule -> (I % 9 * FH) + (I % 9) % 2 == 1 => HH,
 *
 */
