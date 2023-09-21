use crate::object::asset::ObjectAssetServer;
use crate::object::{find_valid_cells, Object, ObjectID, ObjectSelectEvent, Selectable};
use crate::world::tile::{TileState, TileStateChangeEvent};
use crate::world::{grid::Grid, World};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UFODropEvent>()
            .add_event::<UFOLiftEvent>()
            .add_systems(Update, control_ufo)
            .add_systems(PostUpdate, (handle_ufo_lift_event, handle_ufo_drop_event));
    }
}

/************************************************************
 * - Types
 */

const UFO_TEXUTRE_PATH: &str = "ufo.png";

const UFO_SPRITE_OFFSET: (i32, i32) = (5, 4 + 26);

const UFO_LIFT_MODIFIER: i32 = 8;

/************************************************************
 * - Types
 */

#[derive(Debug, Event)]
pub struct UFODropEvent {
    entity: Entity,
    position: IVec2,
}

impl UFODropEvent {
    pub fn new(entity: Entity, position: IVec2) -> Self {
        Self { entity, position }
    }
}

#[derive(Debug, Event)]
pub struct UFOLiftEvent {
    id: ObjectID,
    entity: Entity,
    position: IVec2,
}

impl UFOLiftEvent {
    pub fn new(id: ObjectID, entity: Entity, position: IVec2) -> Self {
        Self {
            id,
            entity,
            position,
        }
    }
}

#[derive(Debug, Component)]
pub struct UFO {
    pub position: IVec2,
    pub offset: IVec2,
    selected: bool,
    selected_id: ObjectID,
    selected_entity: Entity,
    selected_difference: IVec2,
    selected_valid_cells: Vec<IVec2>,
}

impl UFO {
    pub fn new(
        position: IVec2,
        grid: &Grid,
        asset_server: &AssetServer,
        commands: &mut Commands,
        events: &mut EventWriter<TileStateChangeEvent>,
    ) -> Entity {
        let world_position: Vec2 =
            grid.cell_to_world(UVec2::new(position.x as u32, position.y as u32));

        events.send(TileStateChangeEvent::new(position, TileState::Selected));

        return commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        world_position.x + UFO_SPRITE_OFFSET.0 as f32,
                        world_position.y + UFO_SPRITE_OFFSET.1 as f32,
                        300.0,
                    ),
                    texture: asset_server.load(UFO_TEXUTRE_PATH),
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                UFO {
                    position,
                    offset: IVec2::new(UFO_SPRITE_OFFSET.0, UFO_SPRITE_OFFSET.1),
                    selected: false,
                    selected_id: ObjectID::None,
                    selected_entity: Entity::PLACEHOLDER,
                    selected_difference: IVec2::ZERO,
                    selected_valid_cells: vec![],
                },
                Name::new("UFO"),
            ))
            .id();
    }
}

/************************************************************
 * - System Functions
 */

fn control_ufo(
    mut drop_event_writer: EventWriter<UFODropEvent>,
    mut tile_event_writer: EventWriter<TileStateChangeEvent>,
    mut objc_event_writer: EventWriter<ObjectSelectEvent>,
    mut ufo_query: Query<(&mut UFO, &mut Transform)>,
    mut obj_query: Query<
        (Entity, &Object, &mut Transform),
        (With<Object>, With<Selectable>, Without<UFO>),
    >,
    mut grid: Res<Grid>,
    oas: Res<ObjectAssetServer>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut ufo, mut transform) = match ufo_query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    let even = ufo.position.y % 2 == 0;
    let mut lift = false;
    let mut moved = false;
    let mut position = IVec2::new(
        ufo.position.x as i32, //
        ufo.position.y as i32,
    );

    // Vertical
    if keyboard.just_pressed(KeyCode::D) {
        if even {
            position.x += 0;
        } else {
            position.x += 1;
        }
        position.y -= 1;

        moved = true;
    }
    if keyboard.just_pressed(KeyCode::A) {
        if even {
            position.x -= 1;
        } else {
            position.x += 0;
        }
        position.y += 1;

        moved = true;
    }

    // Horizontal
    if keyboard.just_pressed(KeyCode::W) {
        if even {
            position.x += 0;
        } else {
            position.x += 1;
        }
        position.y += 1;

        moved = true;
    }
    if keyboard.just_pressed(KeyCode::S) {
        if even {
            position.x -= 1;
        } else {
            position.x += 0;
        }
        position.y -= 1;

        moved = true;
    }

    if keyboard.just_pressed(KeyCode::H) {
        lift = true;
    }

    if position.x < 0 {
        if even {
            position.x = 1;
        } else {
            position.x = 0;
        }
    } else if position.x >= grid.size.0 as i32 {
        position.x = grid.size.0 as i32 - 1;
    }

    if position.y < 0 {
        position.y = 0;
    } else if position.y >= grid.size.1 as i32 {
        position.y = grid.size.1 as i32 - 1;
    }

    if grid.grid[((position.y * grid.size.0 as i32) + position.x) as usize] == 0 {
        return;
    }

    if !ufo.selected && lift {
        tile_event_writer.send(TileStateChangeEvent::new(ufo.position, TileState::Default));

        objc_event_writer.send(ObjectSelectEvent::new(ufo.position, true));
    }

    if moved {
        if !ufo.selected {
            tile_event_writer.send(TileStateChangeEvent::new(ufo.position, TileState::Default));
            tile_event_writer.send(TileStateChangeEvent::new(position, TileState::Selected));

            objc_event_writer.send(ObjectSelectEvent::new(position, false));
        }

        ufo.position.x = position.x;
        ufo.position.y = position.y;

        let world_position =
            grid.cell_to_world(UVec2::new(ufo.position.x as u32, ufo.position.y as u32));

        transform.translation.x = world_position.x + ufo.offset.x as f32;
        transform.translation.y = world_position.y + ufo.offset.y as f32;
    }

    if ufo.selected {
        let asset = oas.get(ufo.selected_id);

        if lift {
            let mut occupied = vec![];
            for pos in &asset.conf.occupy {
                occupied.push((ufo.position - ufo.selected_difference) + *pos);
            }

            let mut valid = false;
            for position in &occupied {
                if !ufo.selected_valid_cells.contains(position) {
                    valid = false;
                    break;
                }

                valid = true;
            }

            if valid {
                ufo.selected = false;

                drop_event_writer.send(UFODropEvent::new(ufo.selected_entity, occupied[0]))
            }
        }

        let lift_mod = if !lift { UFO_LIFT_MODIFIER as f32 } else { 0.0 };

        let obj_position = UVec2::new(
            (ufo.position.x + ufo.selected_difference.x) as u32,
            (ufo.position.y + ufo.selected_difference.y) as u32,
        );

        for (entity, obj, mut transform) in &mut obj_query {
            if entity != ufo.selected_entity {
                continue;
            }

            let world_position: Vec2 = grid.cell_to_world(obj_position);

            transform.translation.x = world_position.x + obj.offset.x as f32;
            transform.translation.y = world_position.y + obj.offset.y as f32 + lift_mod;
            transform.translation.z = 100.0 + grid.cell_order(obj_position) as f32;
        }
    }
}

fn handle_ufo_lift_event(
    mut event_writer: EventWriter<TileStateChangeEvent>,
    mut event_reader: EventReader<UFOLiftEvent>,
    mut query: Query<&mut UFO>,
    world: Res<World>,
    grid: Res<Grid>,
) {
    if event_reader.is_empty() {
        return;
    }
    if event_reader.len() > 1 {
        panic!("Encountered unhandled UFOLiftEvent.");
    }

    let mut ufo = match query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    for e in event_reader.iter() {
        ufo.selected = true;
        ufo.selected_id = e.id;
        ufo.selected_entity = e.entity;
        ufo.selected_difference = e.position - ufo.position;
        ufo.selected_valid_cells = find_valid_cells(ufo.selected_id, e.position, &world, &grid);
    }

    for position in &ufo.selected_valid_cells {
        event_writer.send(TileStateChangeEvent::new(*position, TileState::Selected));
    }
}

fn handle_ufo_drop_event(
    mut event_writer: EventWriter<TileStateChangeEvent>,
    mut event_reader: EventReader<UFODropEvent>,
    mut ufo_query: Query<&mut UFO>,
    mut obj_query: Query<(Entity, &mut Object), With<Selectable>>,
    mut world: ResMut<World>,
    oas: Res<ObjectAssetServer>,
) {
    if event_reader.is_empty() {
        return;
    }
    if event_reader.len() > 1 {
        panic!("Encountered unhandled UFODropEvent.");
    }

    let mut ufo = match ufo_query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    for pos in &ufo.selected_valid_cells {
        event_writer.send(TileStateChangeEvent::new(*pos, TileState::Default));
    }

    for e in event_reader.iter() {
        for (entity, mut obj) in &mut obj_query {
            if entity != e.entity {
                continue;
            }

            println!("POSITION: {}", e.position);

            let asset = oas.get(obj.id);
            let world_size = world.size.0 as i32;

            for pos in &obj.occupied {
                world.objects[((pos.y * world_size) + pos.x) as usize] = None
            }

            let mut occupied = vec![];
            for offset in &asset.conf.occupy {
                occupied.push(e.position + *offset);
            }

            obj.occupied = occupied;
            for pos in &obj.occupied {
                world.objects[((pos.y * world_size) + pos.x) as usize] = Some((entity, obj.id));
            }

            break;
        }
    }
}
