use crate::object::asset::ObjectAssetServer;
use crate::render::RenderLayer;
use crate::world::grid::Grid;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    pub position: UVec2,
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

impl fmt::Display for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Component)]
pub struct Selectable {
    pub selected: bool,
}

impl Selectable {
    pub fn new() -> Self {
        Self { selected: false }
    }
}

#[derive(Debug, Component)]
pub struct Animated;

impl Animated {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Component)]
pub struct Object {
    pub id: ObjectID,
    pub name: String,
    pub occupied: Vec<UVec2>,
    offset: UVec2,
}

impl Object {
    pub fn new(
        id: ObjectID,
        position: UVec2,
        grid: &Grid,
        ocs: &ObjectAssetServer,
        commands: &mut Commands,
    ) -> Entity {
        let asset = ocs.get(id);

        let world_position = grid.cell_to_world(position);

        let mut occupied = vec![];
        for offset in &asset.conf.occupied {
            occupied.push(UVec2::new(position.x + offset.x, position.y + offset.y));
        }

        let entity = commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        world_position.x + asset.conf.offset.x as f32,
                        world_position.y + asset.conf.offset.y as f32,
                        0.0,
                    ),
                    texture: asset.assets[0].clone(),
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Object {
                    id,
                    occupied,
                    name: asset.conf.name.clone(),
                    offset: asset.conf.offset,
                },
                RenderLayer::Entity(grid.cell_order(position) as usize),
                Name::new(asset.conf.name.clone()),
            ))
            .id();

        if asset.conf.selectable {
            commands.entity(entity).insert(Selectable::new());
        }

        if asset.conf.animated {
            commands.entity(entity).insert(Animated::new());
        }

        return entity;
    }
}

/************************************************************
 * - Notes
 *
 * UFO -> Sends select event, with UFO's position.
 * Check every selectable object's occupied areas.
 * IF T -> Cache relative position difference, set selected.
 *      -> UFO's in selected mode, send change pos every time it moves.
 * IF N -> Send non-selectable event as response to UFO.
 */
