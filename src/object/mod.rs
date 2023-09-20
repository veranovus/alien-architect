use crate::object::asset::ObjectAssetServer;
use crate::render::RenderLayer;
use crate::world::grid::Grid;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use serde::{Deserialize, Serialize};

pub mod asset;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(asset::AssetPlugin);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDesc {
    pub id: ObjectID,
    pub position: IVec2,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component, Serialize, Deserialize)]
pub enum ObjectID {
    // Entity IDs
    King,
    Villager,
    Cow,
    Assassin,
    // Building IDs
    Castle,
    Mountain,
    Field,
    House,
    BigHouse,
    Farm,
    Tower,
    Church,
    Tavern,
}

#[derive(Debug, Component)]
pub struct Object {
    pub id: ObjectID,
    pub name: String,
    pub position: UVec2,
    pub movable: bool,
}

impl Object {
    pub fn new(
        id: ObjectID,
        position: UVec2,
        grid: &Grid,
        oas: &ObjectAssetServer,
        commands: &mut Commands,
    ) -> Entity {
        let asset = oas.get(id);

        let world_position = grid.cell_to_world(position) + grid.cell_center_offset();

        return commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(world_position.x, world_position.y, 0.0),
                    texture: asset.handle.clone(),
                    sprite: Sprite {
                        anchor: Anchor::Custom(asset.origin),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Object {
                    id,
                    name: asset.name,
                    position,
                    movable: id.movable(),
                },
                RenderLayer::Entity(grid.cell_order(position) as usize),
                Name::new(asset.name),
            ))
            .id();
    }
}
