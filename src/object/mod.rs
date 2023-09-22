use crate::object::asset::ObjectAssetServer;
use crate::player::UFOLiftEvent;
use crate::render::RenderLayer;
use crate::world::{grid::Grid, World};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod asset;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ObjectSelectEvent>()
            .add_plugins(asset::AssetPlugin)
            .add_systems(
                PostUpdate,
                (handle_select_object_event, update_object_image),
            );
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
    None,
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

#[derive(Debug, Event)]
pub struct ObjectSelectEvent {
    position: IVec2,
    callback: bool,
}

impl ObjectSelectEvent {
    pub fn new(position: IVec2, callback: bool) -> Self {
        Self { position, callback }
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
    pub occupied: Vec<IVec2>,
    pub offset: IVec2,
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

        let world_position = grid.cell_to_world(position);

        let mut occupied = vec![];
        for offset in &asset.conf.occupy {
            occupied.push(IVec2::new(
                position.x as i32 + offset.x,
                position.y as i32 + offset.y,
            ));
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
 * - System Functions
 */

fn handle_select_object_event(
    mut event_writer: EventWriter<UFOLiftEvent>,
    mut event_reader: EventReader<ObjectSelectEvent>,
    mut query: Query<(Entity, &Object, &mut Selectable)>,
) {
    for e in event_reader.iter() {
        let mut selected = false;

        for (entity, obj, mut selectable) in &mut query {
            if obj.occupied.contains(&e.position) && !selected {
                selectable.selected = true;

                if e.callback {
                    event_writer.send(UFOLiftEvent::new(obj.id, entity, obj.occupied[0]));
                }

                selected = true;
                continue;
            }

            selectable.selected = false;
        }
    }
}

fn update_object_image(
    mut query: Query<(&Object, &Selectable, &mut Handle<Image>), Changed<Selectable>>,
    oas: Res<ObjectAssetServer>,
) {
    for (obj, selectable, mut handle) in &mut query {
        if selectable.selected {
            *handle = oas.get(obj.id).assets[1].clone();
        } else {
            *handle = oas.get(obj.id).assets[0].clone();
        }
    }
}

/************************************************************
 * - Helper Functions
 */

pub fn find_valid_cells(id: ObjectID, position: IVec2, world: &World, grid: &Grid) -> Vec<IVec2> {
    return match id {
        ObjectID::Villager => {
            vec![]
        }
        ObjectID::Cow => {
            vec![]
        }
        ObjectID::House => {
            let even = position.y % 2;

            let adjecteds: Vec<(i32, i32)> = vec![
                (0, 0),
                (0, 2),
                (0, -2),
                (1, 0),
                (-1, 0),
                (0 + even, 1),
                (0 + even, -1),
                (-1 + even, 1),
                (-1 + even, -1),
            ];

            let mut valid = vec![];
            for (x, y) in adjecteds {
                let new = IVec2::new(position.x + x, position.y + y);
                println!("TARGET: {}", new);

                if (new.x < 0 || new.x >= grid.size.0 as i32)
                    || (new.y < 0 || new.y >= grid.size.1 as i32)
                {
                    continue;
                }
                if grid.grid[((new.y * grid.size.0 as i32) + new.x) as usize] == 0 {
                    continue;
                }

                valid.push(new);
            }

            valid
        }
        ObjectID::BigHouse => {
            vec![]
        }
        ObjectID::Farm => {
            vec![]
        }
        ObjectID::Tower => {
            vec![]
        }
        ObjectID::Church => {
            vec![]
        }
        ObjectID::Tavern => {
            vec![]
        }
        _ => panic!("Can't find valid cells for immobile {}.", id.to_string()),
    };
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
