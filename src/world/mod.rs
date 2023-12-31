use crate::object::asset::ObjectAssetServer;
use crate::object::{Object, ObjectDesc, ObjectID};
use crate::world::grid::{Grid, GridPlugin};
use crate::world::tile::{TileMap, TilePlugin};
use bevy::prelude::*;

pub mod grid;
pub mod tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GridPlugin)
            .add_plugins(TilePlugin)
            .add_systems(Startup, setup_world);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
pub struct World {
    pub size: (u32, u32),
    pub objects: Vec<Option<(Entity, ObjectID)>>,
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

pub fn generate_objects(
    objects: &Vec<ObjectDesc>,
    grid: &Grid,
    oas: &ObjectAssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    world: &mut World,
    commands: &mut Commands,
) {
    world.objects.fill(None);

    let (mut king, mut castle) = (false, false);

    for od in objects {
        if !king {
            if let ObjectID::King = od.id {
                king = true;
            }
        } else {
            if let ObjectID::King = od.id {
                panic!("There can't be more than one King in a level.");
            }
        }

        if !castle {
            if let ObjectID::Castle = od.id {
                castle = true;
            }
        } else {
            if let ObjectID::Castle = od.id {
                panic!("There can't be more than one Castle in a level.");
            }
        }

        Object::new(
            od.id,
            od.position,
            texture_atlases,
            world,
            commands,
            grid,
            oas,
        );
    }

    if !king | !castle {
        panic!("Encountered incomplete level, either King or Castle is missing.");
    }
}
