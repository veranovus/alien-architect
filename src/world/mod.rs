use bevy::prelude::*;

pub mod map;
mod tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(map::MapPlugin);
    }
}
