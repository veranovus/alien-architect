use crate::render::RenderLayer;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        todo!()
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
#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize)]
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

impl ToString for ObjectID {
    fn to_string(&self) -> String {
        return match self {
            ObjectID::King => "King",
            ObjectID::Villager => "Villager",
            ObjectID::Cow => "Cow",
            ObjectID::Assassin => "Assassin",
            ObjectID::Castle => "Castle",
            ObjectID::Mountain => "Mountain",
            ObjectID::Field => "Field",
            ObjectID::House => "House",
            ObjectID::BigHouse => "BigHouse",
            ObjectID::Farm => "Farm",
            ObjectID::Tower => "Tower",
            ObjectID::Church => "Church",
            ObjectID::Tavern => "Tavern",
        }
        .to_string();
    }
}

impl ObjectID {
    fn get_texture_path(&self) -> &str {
        return match self {
            ObjectID::King => "temp/king.png",
            ObjectID::Villager => "temp/villager.png",
            ObjectID::Cow => "temp/cow.png",
            ObjectID::Assassin => "temp/assassin.png",
            ObjectID::Castle => "temp/castle.png",
            ObjectID::Mountain => "temp/mountain.png",
            ObjectID::Field => "temp/field.png",
            ObjectID::House => "temp/house.png",
            ObjectID::BigHouse => "temp/big-house.png",
            ObjectID::Farm => "temp/farm.png",
            ObjectID::Tower => "temp/tower.png",
            ObjectID::Church => "temp/church.png",
            ObjectID::Tavern => "temp/tavern.png",
        };
    }

    pub fn movable(&self) -> bool {
        return match self {
            ObjectID::Villager => true,
            ObjectID::Cow => true,
            ObjectID::House => true,
            ObjectID::BigHouse => true,
            ObjectID::Farm => true,
            ObjectID::Tower => true,
            ObjectID::Church => true,
            ObjectID::Tavern => true,
            _ => false,
        };
    }
}

#[derive(Debug, Component)]
pub struct Object {
    pub id: ObjectID,
    pub grid_position: IVec2,
    pub movable: bool,
}

impl Object {
    pub fn new(
        id: ObjectID,
        position: Vec2,
        grid_position: IVec2,
        order: usize,
        asset_server: &Res<AssetServer>,
        commands: &mut Commands,
    ) -> Entity {
        return commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(position.x, position.y, 0.0),
                    texture: asset_server.load(id.get_texture_path()),
                    ..Default::default()
                },
                Object {
                    id,
                    grid_position,
                    movable: id.movable(),
                },
                RenderLayer::Entity(order),
                Name::new(id.to_string()),
            ))
            .id();
    }
}
