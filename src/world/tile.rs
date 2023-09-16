use crate::render::RenderLayer;
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
        let tilemap = commands.spawn((TransformBundle::default(), TileMap)).id();

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
        posw: Vec2,
        posg: IVec2,
        order: usize,
        active: bool,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Entity {
        return commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(posw.x, posw.y, 0.0),
                    texture: asset_server.load(TILE_TEXTURE_PATH),
                    ..Default::default()
                },
                Tile {
                    position: posg,
                    active,
                },
                RenderLayer::Tile(order),
            ))
            .id();
    }
}
