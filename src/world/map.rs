use crate::global;
use crate::world::tile;
use crate::world::tile::{Tile, TileMap};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_map);
        app.add_systems(Startup, generate_map);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct MapDesc {
    lrow: usize,
    srow: usize,
    row_count: usize,
    grid: Vec<usize>,
}

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
    pub grid: Vec<usize>,
    pub origins: Vec<MapPoint>,
}

impl Map {
    pub fn new() -> Self {
        return Self {
            lrow: 0,
            srow: 0,
            row_count: 0,
            size: 0,
            grid: vec![],
            origins: vec![],
        };
    }

    pub fn load(&mut self, path: &str) {
        let mdesc: MapDesc = if let Ok(contents) = std::fs::read_to_string(path) {
            ron::from_str(&contents).unwrap()
        } else {
            panic!("Failed to read map-file at `{}`.", path);
        };

        // Setup Map properties from MapDesc
        self.lrow = mdesc.lrow;
        self.srow = mdesc.srow;
        self.row_count = mdesc.row_count;
        self.size = (self.lrow * self.row_count) + (self.srow * self.row_count);

        self.grid = mdesc.grid;

        // Calculate Map origins
        let line: usize = self.lrow + self.srow;

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
            self.lrow as f32 * fw,
            // Total height for the world tiles
            fh + ((self.row_count - 1) as f32 * fth) + hth,
        );

        let offset = (
            (global::window::VIEWPORT_RESOLUTION.0 as f32 - area.0) / 2.0,
            (global::window::VIEWPORT_RESOLUTION.1 as f32 - area.1) / 2.0,
        );

        for i in 0..self.size {
            let xpos = (i % line) as f32 * hw;
            let ypos = ((i / line) as f32 * fth) + (((i % line) % 2) as f32 * hth);
            let zpos = ((i / line) * 9) + ((i % line) / 2) + (((i % line) % 2) * 4);

            self.origins.push(MapPoint {
                index: i,
                order: zpos,
                world_position: Vec2::new(
                    (offset.0 + area.0) - (xpos + hw),
                    (offset.1 + area.1) - (ypos + hh),
                ),
                grid_position: IVec2::new(
                    (i % line) as i32,
                    ((i / line) * 2) as i32 + ((i % line) % 2) as i32,
                ),
            });
        }
    }
}

/************************************************************
 * - System Functions
 */

fn setup_map(mut commands: Commands) {
    let mut map = Map::new();

    map.load("assets/test-map.ron");

    commands.insert_resource(map);
}

pub fn generate_map(mut commands: Commands, mut map: ResMut<Map>, asset_server: Res<AssetServer>) {
    let mut tiles = vec![];

    for (i, origin) in map.origins.iter().enumerate() {
        if map.grid[i] == 0 {
            continue;
        }

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
