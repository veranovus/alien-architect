use crate::render::RenderLayer;
use crate::world::grid::Grid;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, setup_tile);
    }
}

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
        index: usize,
        position: UVec2,
        grid: &Grid,
        asset_server: &AssetServer,
        commands: &mut Commands,
    ) -> Entity {
        let world_position = grid.cell_to_world(position);

        let tile_path = if position.y % 2 != 0 {
            "tiles/tile_0.png"
        } else {
            "tiles/tile_1.png"
        };

        return commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(world_position.x, world_position.y, 0.0),
                    texture: asset_server.load(tile_path),
                    ..Default::default()
                },
                Tile {
                    position: IVec2::new(position.x as i32, position.y as i32),
                    active: grid.grid[index] != 0,
                },
                RenderLayer::Tile(grid.cell_order(index) as usize),
                Name::new(format!("Tile #{}", index)),
            ))
            .id();
    }
}

/************************************************************
 * - Types
 */

fn setup_tile(mut query: Query<(&Tile, &mut Visibility), Added<Tile>>) {
    for (tile, mut visibility) in &mut query {
        if tile.active {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
