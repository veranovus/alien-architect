use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct TileMap;

#[derive(Debug, Component)]
pub struct Tile {
    pub active: bool,
}

impl Tile {
    // pub fn new() -> Entity {}
}
