use crate::object::asset::ObjectAssetServer;
use crate::object::{find_valid_cells, Object, ObjectID, ObjectSelectEvent, Selectable};
use crate::world::tile::{TileState, TileStateChangeEvent};
use crate::world::{grid::Grid, World};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::tracing::Event;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UFODropEvent>()
            .add_event::<UFOLiftEvent>()
            .add_systems(Update, (control_ufo, ufo_carry_object))
            .add_systems(PostUpdate, handle_ufo_lift_event);
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
pub struct UFODropEvent {
    position: IVec2,
}

impl UFODropEvent {
    pub fn new(position: IVec2) -> Self {
        Self { position }
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
/*
fn _control_ufo(
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

        //objc_event_writer.send(ObjectSelectEvent::new(ufo.position, true));
    }

    if moved {
        if !ufo.selected {
            tile_event_writer.send(TileStateChangeEvent::new(ufo.position, TileState::Default));
            tile_event_writer.send(TileStateChangeEvent::new(position, TileState::Selected));

            //objc_event_writer.send(ObjectSelectEvent::new(position, false));
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

                //drop_event_writer.send(UFODropEvent::new(ufo.selected_entity, occupied[0]))
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

fn _handle_ufo_lift_event(
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
            drop_event_writer.send(UFODropEvent::new(ufo.position));
        }
    }

    if moved {
        let t = ufo.position;
        let moved = ufo_move(position, &mut ufo, &mut transform, &grid);

        if moved {
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
        let target = ufo.position - asset.conf.occupy[selection.occupy_index];

        let world_position = grid.cell_to_world(UVec2::new(target.x as u32, target.y as u32));
        let order = grid.cell_order(UVec2::new(target.x as u32, target.y as u32));

        transform.translation.x = world_position.x + obj.offset.x as f32;
        transform.translation.y = world_position.y + obj.offset.y as f32 + UFO_LIFT_MODIFIER as f32;
        transform.translation.z = 100.0 + order as f32;
    }
}

fn handle_ufo_lift_event(
    mut ufo_query: Query<&mut UFO>,
    mut obj_query: Query<(Entity, &Object), With<Selectable>>,
    mut event_reader: EventReader<UFOLiftEvent>,
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
            if !*occupy == event.position {
                continue;
            }

            occupy_index = i as i32;
            break;
        }

        if occupy_index == -1 {
            continue;
        }

        ufo.selected = Some(UFOSelection::new(entity, occupy_index as usize));
        break;
    }
}

fn hande_ufo_drop_event() {}
