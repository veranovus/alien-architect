use crate::render::RenderLayer;
use crate::world::map::MapPoint;
use bevy::prelude::*;

/************************************************************
 * - Constants
 */

const TILE_TEXTURE_PATH: &str = "temp/tile.png";

/************************************************************
 * - Types
 */

#[derive(Debug, Component)]
pub struct TileMap;

impl TileMap {
    pub fn new(tiles: &Vec<Entity>, commands: &mut Commands) -> Entity {
        let tilemap = commands
            .spawn((SpatialBundle::default(), TileMap, Name::new("Tile Map")))
            .id();

        commands.entity(tilemap).push_children(tiles);

        return tilemap;
    }
}

#[derive(Debug, Component)]
pub struct Tile {
    pub position: IVec2,
    pub active: bool,
}

impl Tile {
    pub fn new(
        active: bool,
        origin: &MapPoint,
        asset_server: &AssetServer,
        commands: &mut Commands,
    ) -> Entity {
        return commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        origin.world_position.x,
                        origin.world_position.y,
                        0.0,
                    ),
                    texture: asset_server.load(TILE_TEXTURE_PATH),
                    ..Default::default()
                },
                Tile {
                    position: origin.grid_position,
                    active,
                },
                RenderLayer::Tile(origin.order),
                Name::new(format!("Tile #{}", origin.index)),
            ))
            .id();
    }
}
