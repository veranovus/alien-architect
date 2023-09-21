use crate::render::RenderLayer;
use crate::world::grid::Grid;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::collections::HashMap;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_tile_asset_server)
            .add_systems(PostUpdate, update_tile_image);
    }
}

/************************************************************
 * - Constants
 */

const TILE_ASSET_PATH_EVEN: [&str; 3] = [
    "tiles/tile_0.png",
    "tiles/selected_tile_0.png",
    "tiles/pathtile.png",
];

const TILE_ASSET_PATH_ODD: [&str; 3] = [
    "tiles/tile_1.png",
    "tiles/selected_tile_1.png",
    "tiles/pathtile.png",
];

/************************************************************
 * - Types
 */

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum TileType {
    Even = 0,
    Odd = 1,
}

impl From<u32> for TileType {
    fn from(value: u32) -> Self {
        return if value % 2 == 0 {
            Self::Even
        } else {
            Self::Odd
        };
    }
}

#[derive(Debug, Resource)]
struct TileAssetServer {
    assets: HashMap<TileType, [Handle<Image>; 3]>,
}

impl TileAssetServer {
    fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TileState {
    Default,
    Selected,
    Path,
}

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
    pub selected: bool,
    pub state: TileState,
    r#type: TileType,
}

impl Tile {
    pub fn new(index: usize, position: UVec2, grid: &Grid, commands: &mut Commands) -> Entity {
        let world_position = grid.cell_to_world(position);

        return commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(world_position.x, world_position.y, 0.0),
                    ..Default::default()
                },
                Tile {
                    position: IVec2::new(position.x as i32, position.y as i32),
                    active: grid.grid[index] != 0,
                    selected: false,
                    state: TileState::Default,
                    r#type: TileType::from(position.y % 2),
                },
                RenderLayer::Tile(grid.cell_order(position) as usize),
                Name::new(format!("Tile #{}", index)),
            ))
            .id();
    }
}

/************************************************************
 * - System Functions
 */

fn setup_tile_asset_server(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut tas = TileAssetServer::new();

    tas.assets.insert(
        TileType::Even,
        [
            asset_server.load(TILE_ASSET_PATH_EVEN[0]),
            asset_server.load(TILE_ASSET_PATH_EVEN[1]),
            asset_server.load(TILE_ASSET_PATH_EVEN[2]),
        ],
    );

    tas.assets.insert(
        TileType::Odd,
        [
            asset_server.load(TILE_ASSET_PATH_ODD[0]),
            asset_server.load(TILE_ASSET_PATH_ODD[1]),
            asset_server.load(TILE_ASSET_PATH_ODD[2]),
        ],
    );

    commands.insert_resource(tas);
}

fn update_tile_image(
    mut query: Query<(&Tile, &mut Visibility, &mut Handle<Image>), Changed<Tile>>,
    tas: Res<TileAssetServer>,
) {
    for (tile, mut visibility, mut handle) in &mut query {
        if tile.active {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }

        match tile.state {
            TileState::Default => *handle = tas.assets.get(&tile.r#type).unwrap()[0].clone(),
            TileState::Selected => *handle = tas.assets.get(&tile.r#type).unwrap()[1].clone(),
            TileState::Path => *handle = tas.assets.get(&tile.r#type).unwrap()[2].clone(),
        }
    }
}
