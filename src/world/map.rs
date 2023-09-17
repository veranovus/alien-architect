use crate::global;
use crate::object::{Object, ObjectDesc, ObjectID};
use crate::world::tile::{Tile, TileMap};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadMapEvent>()
            .add_systems(PreStartup, setup_map)
            .add_systems(PostUpdate, handle_load_map_event)
            .add_systems(Update, control_map);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
struct MapDesc {
    lrow: usize,
    srow: usize,
    row_count: usize,
    grid: Vec<usize>,
    objects: Vec<ObjectDesc>,
}

#[derive(Debug, Event)]
pub struct LoadMapEvent {
    path: String,
}

impl LoadMapEvent {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
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
    pub origins: HashMap<IVec2, MapPoint>,
}

impl Map {
    fn new() -> Self {
        return Self {
            lrow: 0,
            srow: 0,
            row_count: 0,
            size: 0,
            grid: vec![],
            origins: HashMap::new(),
        };
    }

    fn generate_origins(&mut self, mdesc: &MapDesc) {
        // Setup Map properties from MapDesc
        self.lrow = mdesc.lrow;
        self.srow = mdesc.srow;
        self.row_count = mdesc.row_count;
        self.size = (self.lrow * self.row_count) + (self.srow * self.row_count);

        // Prepare origins and grid
        self.grid = mdesc.grid.clone();
        self.origins.clear();

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

            let key = IVec2::new(
                (i % line) as i32,
                ((i / line) * 2) as i32 + ((i % line) % 2) as i32,
            );

            self.origins.insert(
                key,
                MapPoint {
                    index: i,
                    order: zpos,
                    world_position: Vec2::new(
                        (offset.0 + area.0) - (xpos + hw),
                        (offset.1 + area.1) - (ypos + hh),
                    ),
                    grid_position: key,
                },
            );
        }
    }
}

/************************************************************
 * - System Functions
 */

fn setup_map(mut commands: Commands) {
    commands.insert_resource(Map::new());
}

fn control_map(mut events: EventWriter<LoadMapEvent>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_released(KeyCode::R) {
        events.send(LoadMapEvent::new("assets/test-level.ron"));
    }
}

pub fn handle_load_map_event(
    mut commands: Commands,
    mut events: EventReader<LoadMapEvent>,
    mut map: ResMut<Map>,
    tilemap: Query<Entity, With<TileMap>>,
    objects: Query<Entity, With<Object>>,
    asset_server: Res<AssetServer>,
) {
    if events.is_empty() {
        return;
    }
    if events.len() > 1 {
        panic!("Encountered multiple unhandled events for `LoadMapEvent`.");
    }

    let mut path = String::new();
    for e in events.iter() {
        path = e.path.clone();
        break;
    }
    events.clear();

    // De-spawn TileMap
    match tilemap.get_single() {
        Ok(e) => commands.entity(e).despawn_recursive(),
        Err(QuerySingleError::MultipleEntities(e)) => panic!("{}", e),
        _ => {}
    }

    // De-spawn Objects
    for e in &objects {
        commands.entity(e).despawn_recursive();
    }

    let mdesc: MapDesc = if let Ok(contents) = std::fs::read_to_string(&path) {
        ron::from_str(&contents).unwrap()
    } else {
        panic!("Failed to read level file at, `{}`.", path);
    };

    map.generate_origins(&mdesc);

    generate_tiles(&map, &asset_server, &mut commands);

    generate_objects(&map, &mdesc, &asset_server, &mut commands);
}

/************************************************************
 * - Helper Functions
 */

fn generate_tiles(map: &Map, asset_server: &Res<AssetServer>, commands: &mut Commands) {
    let mut tiles = vec![];
    for (_, origin) in map.origins.iter() {
        if map.grid[origin.index] == 0 {
            continue;
        }
        tiles.push(Tile::new(true, &origin, &asset_server, commands));
    }

    TileMap::new(&tiles, commands);
}

fn generate_objects(
    map: &Map,
    mdesc: &MapDesc,
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
) {
    for obj in &mdesc.objects {
        let origin = if let Some(origin) = map.origins.get(&obj.position) {
            origin
        } else {
            panic!(
                "Failed to spawn Object::{}, origin doesn't exist for given position `{}`.",
                obj.id.to_string(),
                obj.position
            );
        };

        Object::new(
            obj.id,
            origin.world_position,
            obj.position,
            origin.order,
            asset_server,
            commands,
        );
    }
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
