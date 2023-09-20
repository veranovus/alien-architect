use crate::world::grid::{Grid, GridPlugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub mod grid;
pub mod map;
mod tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(map::MapPlugin);
        app.add_plugins(GridPlugin)
            .add_systems(Startup, generate_tiles);
    }
}

fn generate_tiles(mut commands: Commands, grid: Res<Grid>, asset_server: Res<AssetServer>) {
    let cc = grid.size.0 * grid.size.1;

    for i in 0..cc {
        let x = i % grid.size.0;
        let y = i / grid.size.0;

        let v = grid.grid_to_world(UVec2::new(x, y));
        let order = cc - (i % grid.size.0 + (i / grid.size.0 * grid.size.0));

        println!("(WP, ZP) : ({}, {})", v, order);

        commands.spawn(SpriteBundle {
            /*
            sprite: Sprite {
                anchor: Anchor::Custom(Vec2::new(
                    0.0,
                    grid.cell_offset.1 as f32 / (grid.size.1 + grid.cell_offset.1) as f32,
                )),
                ..Default::default()
            },
             */
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform::from_xyz(v.x, v.y, order as f32),
            texture: asset_server.load("tiles/tile.png"),
            ..Default::default()
        });
    }
}
