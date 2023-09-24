use crate::object::asset::{ObjectAsset, ObjectAssetServer};
use crate::object::{self, Object, ObjectSelectEvent, Selectable};
use crate::render::{RenderLayer, RENDER_LAYER};
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
            .add_systems(Update, (control_ufo, ufo_carry_object))
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

#[derive(Debug)]
struct UFOSelection {
    entity: Entity,
    occupy_index: usize,
}

impl UFOSelection {
    fn new(entity: Entity, occupy_index: usize) -> Self {
        Self {
            entity,
            occupy_index,
        }
    }
}

#[derive(Debug, Event)]
pub struct UFOLiftEvent {
    position: IVec2,
}

impl UFOLiftEvent {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}

#[derive(Debug, Event)]
pub struct UFODropEvent {}

impl UFODropEvent {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Component)]
pub struct UFO {
    pub position: IVec2,
    pub offset: IVec2,
    selected: Option<UFOSelection>,
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
                        RENDER_LAYER[RenderLayer::UFO as usize] as f32,
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
                    selected: None,
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
    mut ufo_query: Query<(&mut UFO, &mut Transform)>,
    mut tile_event_writer: EventWriter<TileStateChangeEvent>,
    mut objc_event_writer: EventWriter<ObjectSelectEvent>,
    mut lift_event_writer: EventWriter<UFOLiftEvent>,
    mut drop_event_writer: EventWriter<UFODropEvent>,
    grid: Res<Grid>,
    keys: Res<Input<KeyCode>>,
) {
    // Get UFO
    let (mut ufo, mut transform) = match ufo_query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    let y_mod = ufo.position.y % 2;

    let mut moved = false;
    let mut position = ufo.position;

    // Vertical
    if keys.just_pressed(KeyCode::D) {
        position.x += 0 + y_mod;
        position.y += -1;

        moved = true;
    }
    if keys.just_pressed(KeyCode::A) {
        position.x += -1 + y_mod;
        position.y += 1;

        moved = true;
    }

    // Horizontal
    if keys.just_pressed(KeyCode::W) {
        position.x += 0 + y_mod;
        position.y += 1;

        moved = true;
    }
    if keys.just_pressed(KeyCode::S) {
        position.x += -1 + y_mod;
        position.y += -1;

        moved = true;
    }

    // Lift & Drop
    if keys.just_pressed(KeyCode::H) {
        if ufo.selected.is_none() {
            objc_event_writer.send(ObjectSelectEvent::new(ufo.position));

            lift_event_writer.send(UFOLiftEvent::new(ufo.position));
        } else {
            drop_event_writer.send(UFODropEvent::new());
        }
    }

    if moved {
        let t = ufo.position;
        let moved = ufo_move(position, &mut ufo, &mut transform, &grid);

        if moved && ufo.selected.is_none() {
            tile_event_writer.send(TileStateChangeEvent::new(t, TileState::Default));
            tile_event_writer.send(TileStateChangeEvent::new(ufo.position, TileState::Selected));

            objc_event_writer.send(ObjectSelectEvent::new(ufo.position));
        }
    }
}

fn ufo_move(target: IVec2, ufo: &mut UFO, transform: &mut Transform, grid: &Grid) -> bool {
    if (target.x < 0 || target.x >= grid.size.0 as i32)
        || (target.y < 0 || target.y >= grid.size.1 as i32)
    {
        return false;
    }

    let index = ((target.y * grid.size.0 as i32) + target.x) as usize;
    if grid.grid[index] == 0 {
        return false;
    }

    ufo.position.x = target.x;
    ufo.position.y = target.y;

    let world_position =
        grid.cell_to_world(UVec2::new(ufo.position.x as u32, ufo.position.y as u32));

    transform.translation.x = world_position.x + ufo.offset.x as f32;
    transform.translation.y = world_position.y + ufo.offset.y as f32;

    return true;
}

fn ufo_carry_object(
    mut obj_query: Query<(Entity, &mut Transform, &Object), With<Selectable>>,
    ufo_query: Query<&UFO>,
    oas: Res<ObjectAssetServer>,
    grid: Res<Grid>,
) {
    // Get UFO
    let ufo = match ufo_query.get_single() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    let selection = match &ufo.selected {
        Some(selection) => selection,
        None => return,
    };

    for (entity, mut transform, obj) in &mut obj_query {
        if entity != selection.entity {
            continue;
        }

        let asset = oas.get(obj.id);
        let target = calculate_object_poition(ufo, selection, asset);

        if (target.x < 0 || target.x >= grid.size.0 as i32)
            || (target.y < 0 || target.y >= grid.size.1 as i32)
        {
            return;
        }

        let world_position = grid.cell_to_world(UVec2::new(target.x as u32, target.y as u32));
        let order = grid.cell_order(UVec2::new(target.x as u32, target.y as u32));

        transform.translation.x = world_position.x + obj.offset.x as f32;
        transform.translation.y = world_position.y + obj.offset.y as f32 + UFO_LIFT_MODIFIER as f32;
        transform.translation.z = (RENDER_LAYER[RenderLayer::Entity as usize] + order) as f32;
    }
}

fn handle_ufo_lift_event(
    mut ufo_query: Query<&mut UFO>,
    mut obj_query: Query<(Entity, &Object), With<Selectable>>,
    mut event_writer: EventWriter<TileStateChangeEvent>,
    mut event_reader: EventReader<UFOLiftEvent>,
    oas: Res<ObjectAssetServer>,
    world: Res<World>,
    grid: Res<Grid>,
) {
    // Validate ER
    if event_reader.len() > 1 {
        panic!("Encountered unhandled UFOLiftEvent.");
    }
    let event = match event_reader.iter().next() {
        Some(event) => event,
        None => return,
    };

    // Get UFO
    let mut ufo = match ufo_query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    for (entity, obj) in &mut obj_query {
        let mut occupy_index = -1;
        for (i, occupy) in obj.occupied.iter().enumerate() {
            if *occupy != event.position {
                continue;
            }

            occupy_index = i as i32;
            break;
        }

        if occupy_index == -1 {
            continue;
        }

        // Clear the TileState for tile UFO is hovering
        event_writer.send(TileStateChangeEvent::new(ufo.position, TileState::Default));

        // Set TileState to Selected for every valid position
        let valid = object::find_valid_cells(entity, obj.id, event.position, &oas, &world, &grid);
        for cell in valid {
            event_writer.send(TileStateChangeEvent::new(cell, TileState::Selected));
        }

        ufo.selected = Some(UFOSelection::new(entity, occupy_index as usize));
        break;
    }
}

fn handle_ufo_drop_event(
    mut ufo_query: Query<&mut UFO>,
    mut obj_query: Query<(Entity, &mut Object, &mut Transform), With<Selectable>>,
    mut event_writer: EventWriter<TileStateChangeEvent>,
    mut event_reader: EventReader<UFODropEvent>,
    mut world: ResMut<World>,
    oas: Res<ObjectAssetServer>,
    grid: Res<Grid>,
) {
    // Validate ER
    if event_reader.len() > 1 {
        panic!("Encountered unhandled UFOLiftEvent.");
    }
    if event_reader.is_empty() {
        return;
    }

    event_reader.clear();

    // Get UFO
    let mut ufo = match ufo_query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    let selection = match &ufo.selected {
        Some(selection) => selection,
        None => panic!("UFO haven't selected any object but, UFODropEvent is called."),
    };

    for (entity, mut obj, mut transform) in &mut obj_query {
        if entity != selection.entity {
            continue;
        }

        let asset = oas.get(obj.id);
        let valid = object::find_valid_cells(entity, obj.id, obj.occupied[0], &oas, &world, &grid);

        // Calculate Object's position
        let target = calculate_object_poition(&ufo, selection, asset);
        let y_mod = target.y % 2;

        // Calculate Object's possible new occupied territory
        let mut occupied = vec![];
        for (i, offset) in asset.conf.occupy.iter().enumerate() {
            occupied.push(IVec2::new(
                target.x as i32
                    + (if i == 0 {
                        offset.x
                    } else {
                        offset.x + y_mod as i32
                    }),
                target.y as i32 + offset.y,
            ));
        }

        // Validate Object's new position
        for cell in &occupied {
            if !valid.contains(cell) {
                return;
            }
        }

        // Clear TileState of currently Selected tiles, except the tile UFO is hovering.
        for cell in valid {
            if cell == ufo.position {
                continue;
            }

            event_writer.send(TileStateChangeEvent::new(cell, TileState::Default));
        }

        // Set the World data to None for Object's previous position.
        for cell in &obj.occupied {
            let index = ((cell.y * grid.size.0 as i32) + cell.x) as usize;

            world.objects[index] = None;
        }

        // Set Object's new position
        obj.occupied = occupied;

        // Set the World data to Some(obj) for the new position
        for cell in &obj.occupied {
            let index = ((cell.y * grid.size.0 as i32) + cell.x) as usize;

            world.objects[index] = Some((entity, obj.id));
        }

        // Set Object's transform to new position
        let world_position = grid.cell_to_world(UVec2::new(target.x as u32, target.y as u32));
        let order = grid.cell_order(UVec2::new(target.x as u32, target.y as u32));

        transform.translation.x = world_position.x + obj.offset.x as f32;
        transform.translation.y = world_position.y + obj.offset.y as f32;
        transform.translation.z = (RENDER_LAYER[RenderLayer::Entity as usize] + order) as f32;

        // Set UFO's selected to None
        ufo.selected = None;

        // Break is mandatory for borrow checker
        break;
    }
}

/************************************************************
 * - Helper Functions
 */

fn calculate_object_poition(ufo: &UFO, selection: &UFOSelection, asset: &ObjectAsset) -> IVec2 {
    let y_mod = (ufo.position.y + 1) % 2;

    let diff = if selection.occupy_index == 0 {
        asset.conf.occupy[selection.occupy_index]
    } else {
        asset.conf.occupy[selection.occupy_index] + IVec2::new(y_mod, 0)
    };

    return ufo.position - diff;
}
