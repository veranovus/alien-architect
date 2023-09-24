use crate::animation::Animate;
use crate::object::asset::ObjectAssetServer;
use crate::render::{RenderLayer, RENDER_LAYER};
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

#[derive(Debug, Event)]
pub struct ObjectSelectEvent {
    position: IVec2,
}

impl ObjectSelectEvent {
    pub fn new(position: IVec2) -> Self {
        Self { position }
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
        texture_atlases: &mut Assets<TextureAtlas>,
        world: &mut World,
        commands: &mut Commands,
        grid: &Grid,
        oas: &ObjectAssetServer,
    ) -> Entity {
        let asset = oas.get(id);

        // Calculate Object's occupied tiles
        let y_mod = position.y % 2;
        let world_position = grid.cell_to_world(position);

        let mut occupied = vec![];
        for (i, offset) in asset.conf.occupy.iter().enumerate() {
            occupied.push(IVec2::new(
                position.x as i32
                    + (if i == 0 {
                        offset.x
                    } else {
                        offset.x + y_mod as i32
                    }),
                position.y as i32 + offset.y,
            ));
        }

        // Create the Object
        let entity = commands
            .spawn((
                Object {
                    id,
                    occupied: occupied.clone(),
                    name: asset.conf.name.clone(),
                    offset: asset.conf.offset,
                },
                Name::new(asset.conf.name.clone()),
            ))
            .id();

        // Give object Selectable property
        if asset.conf.selectable {
            commands.entity(entity).insert(Selectable::new());
        }

        // Give object Animated property
        match &asset.conf.animated {
            Some(desc) => {
                let texture_atlas = TextureAtlas::from_grid(
                    asset.assets[0].clone(),
                    Vec2::new(desc.image_size.0 as f32, desc.image_size.1 as f32),
                    desc.atlas_size.0,
                    desc.atlas_size.1,
                    None,
                    None,
                );

                commands.entity(entity).insert((
                    SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            world_position.x + asset.conf.offset.x as f32,
                            world_position.y + asset.conf.offset.y as f32,
                            (RENDER_LAYER[RenderLayer::Entity as usize] + grid.cell_order(position))
                                as f32,
                        ),
                        texture_atlas: texture_atlases.add(texture_atlas),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Animate::new(desc.atlas_size.0 * desc.atlas_size.1, desc.interval, false),
                ));
            }
            None => {
                commands.entity(entity).insert(SpriteBundle {
                    transform: Transform::from_xyz(
                        world_position.x + asset.conf.offset.x as f32,
                        world_position.y + asset.conf.offset.y as f32,
                        (RENDER_LAYER[RenderLayer::Entity as usize] + grid.cell_order(position))
                            as f32,
                    ),
                    texture: asset.assets[0].clone(),
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }

        // Add object to world
        for cell in occupied {
            let index = ((cell.y * grid.size.0 as i32) + cell.x) as usize;

            world.objects[index] = Some((entity, id));
        }

        return entity;
    }
}

/************************************************************
 * - System Functions
 */

fn handle_select_object_event(
    mut query: Query<(&Object, &mut Selectable)>,
    mut event_reader: EventReader<ObjectSelectEvent>,
) {
    for e in event_reader.iter() {
        let mut selected = false;

        for (obj, mut selectable) in &mut query {
            if obj.occupied.contains(&e.position) && !selected {
                selectable.selected = true;

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
        let asset = oas.get(obj.id);

        if selectable.selected && asset.assets.len() > 1 {
            *handle = asset.assets[1].clone();
        } else {
            *handle = asset.assets[0].clone();
        }
    }
}

/************************************************************
 * - Helper Functions
 */

pub fn find_valid_cells(
    entity: Entity,
    id: ObjectID,
    position: IVec2,
    oas: &ObjectAssetServer,
    world: &World,
    grid: &Grid,
) -> Vec<IVec2> {
    return match id {
        ObjectID::Villager => {
            let mut valid = vec![];

            for i in 0..(grid.size.0 * grid.size.1) {
                let position = IVec2::new((i % grid.size.0) as i32, (i / grid.size.0) as i32);

                // Validate current position
                let (valid_position, index) = validate_position(position, grid);

                if !valid_position {
                    continue;
                }

                // If target position is empty or a farm push it as a valid position
                match world.objects[index] {
                    Some((_, target_id)) => match target_id {
                        ObjectID::House | ObjectID::BigHouse => valid.push(position),
                        _ => {}
                    },
                    None => valid.push(position),
                }
            }

            valid
        }
        ObjectID::Cow => {
            let mut valid = vec![];

            for i in 0..(grid.size.0 * grid.size.1) {
                let position = IVec2::new((i % grid.size.0) as i32, (i / grid.size.0) as i32);

                // Validate current position
                let (valid_position, index) = validate_position(position, grid);

                if !valid_position {
                    continue;
                }

                // If target position is empty or a farm push it as a valid position
                match world.objects[index] {
                    Some((_, target_id)) => {
                        if let ObjectID::Farm = target_id {
                            valid.push(position);
                        }
                    }
                    None => valid.push(position),
                }
            }

            valid
        }
        ObjectID::House => valid_tiles_for_adjacted_rule(position, world, grid),
        ObjectID::BigHouse => valid_tiles_for_adjacted_rule(position, world, grid),
        ObjectID::Farm => valid_tiles_for_n_number_of_neighbour_rule(
            entity,
            ObjectID::Farm,
            ObjectID::Field,
            2,
            oas,
            world,
            grid,
        ),
        ObjectID::Tower => {
            let mut valid = vec![];
            let mut castle = IVec2::ZERO;

            for i in 0..(grid.size.0 * grid.size.1) {
                let position = IVec2::new((i % grid.size.0) as i32, (i / grid.size.0) as i32);

                // Validate current position
                let (valid, index) = validate_position(position, grid);

                if !valid {
                    continue;
                }

                if let Some((_, id)) = world.objects[index] {
                    if let ObjectID::Castle = id {
                        castle = position;
                        break;
                    }
                }
            }

            for i in 0..grid.size.1 as i32 {
                // Select the horizontal tiles
                {
                    let x = (castle.x - (castle.y / 2)) + (i / 2);

                    let position = IVec2::new(x, i);

                    // Validate current position
                    let (mut valid_position, index) = validate_position(position, grid);

                    if valid_position {
                        match world.objects[index] {
                            Some((target_entity, _)) => {
                                if target_entity != entity {
                                    valid_position = false;
                                }
                            }
                            None => {}
                        }
                    }

                    if valid_position {
                        valid.push(position);
                    }
                }

                // Select the vertical tiles
                {
                    let ymod = castle.y % 2;

                    let mut x: i32 = castle.x - ((grid.size.1 as i32 - 1) - castle.y) / 2;
                    let mut y: i32 = (castle.x * 2) + castle.y + ymod;

                    if x < 0 {
                        x = 0;
                    }
                    if x >= grid.size.0 as i32 {
                        x = grid.size.0 as i32 - 1;
                    }

                    if y >= grid.size.1 as i32 {
                        y = grid.size.1 as i32 - 1;
                    }

                    let position = IVec2::new(x + (i as i32 / 2), y - i as i32);

                    // Validate current position
                    let (valid_position, index) = validate_position(position, grid);

                    if !valid_position {
                        continue;
                    }

                    match world.objects[index] {
                        Some((target_entity, _)) => {
                            if target_entity != entity {
                                continue;
                            }
                        }
                        None => {}
                    }

                    valid.push(position);
                }
            }

            valid
        }
        ObjectID::Church => {
            let asset = oas.get(ObjectID::Church);

            let mut valid = vec![];

            for i in 0..(grid.size.0 * grid.size.1) {
                let position = IVec2::new((i % grid.size.0) as i32, (i / grid.size.0) as i32);
                let y_mod = position.y % 2;

                let mut found = true;

                for (i, offset) in asset.conf.occupy.iter().enumerate() {
                    // Calculate position for every cell that Object will occupy
                    let current = IVec2::new(
                        position.x + (if i == 0 { offset.x } else { offset.x + y_mod }),
                        position.y + offset.y,
                    );

                    // Validate current position
                    let (valid, index) = validate_position(current, &grid);

                    if !valid {
                        found = false;
                        break;
                    }

                    match world.objects[index] {
                        Some((target_entity, _)) => {
                            if target_entity == entity {
                                continue;
                            }

                            found = false;
                            break;
                        }
                        None => {
                            continue;
                        }
                    }
                }

                // If a valid position is found push every tile that Object may occupy
                if found {
                    for (i, cell) in asset.conf.occupy.iter().enumerate() {
                        let position = IVec2::new(
                            position.x + (if i == 0 { cell.x } else { cell.x + y_mod }),
                            position.y + cell.y,
                        );

                        if !valid.contains(&position) {
                            valid.push(position);
                        }
                    }
                }
            }

            valid
        }
        ObjectID::Tavern => valid_tiles_for_n_number_of_neighbour_rule(
            entity,
            ObjectID::Tavern,
            ObjectID::House,
            4,
            oas,
            world,
            grid,
        ),
        _ => panic!("Can't find valid cells for immobile {}.", id.to_string()),
    };
}

fn get_adjected(position: IVec2) -> [(i32, i32); 9] {
    let y_mod = position.y % 2;
    return [
        (0, 0),
        (0, 2),
        (0, -2),
        (1, 0),
        (-1, 0),
        (0 + y_mod, 1),
        (0 + y_mod, -1),
        (-1 + y_mod, 1),
        (-1 + y_mod, -1),
    ];
}

fn validate_position(position: IVec2, grid: &Grid) -> (bool, usize) {
    let index = ((position.y * grid.size.0 as i32) + position.x) as usize;

    if (position.x < 0 || position.x >= grid.size.0 as i32)
        || (position.y < 0 || position.y >= grid.size.1 as i32)
    {
        return (false, index);
    }

    if grid.grid[index] == 0 {
        return (false, index);
    }

    return (true, index);
}

fn count_neighbour_id(
    entity: Entity,
    id: ObjectID,
    position: IVec2,
    offsets: &Vec<IVec2>,
    world: &World,
    grid: &Grid,
) -> usize {
    let y_mod = position.y % 2;

    let mut count = 0;
    // Store already counted positions, to not count them multiple times for different occupied spaces
    let mut counted = vec![];

    for (i, offset) in offsets.iter().enumerate() {
        // Recalculate position for every occupied cell
        let current = IVec2::new(
            position.x + (if i == 0 { offset.x } else { offset.x + y_mod }),
            position.y + offset.y,
        );

        // Validate the current position
        let (valid, index) = validate_position(current, grid);

        if !valid {
            return 0;
        }

        match world.objects[index] {
            Some((target_entity, _)) => {
                if target_entity != entity {
                    return 0;
                }
            }
            None => {}
        }

        // Check the adjacted cells for current cell
        let adjected = get_adjected(current);
        for (x, y) in adjected {
            let target = IVec2::new(position.x + x, position.y + y);

            // Validate the current position
            let (valid, index) = validate_position(target, grid);

            if !valid {
                continue;
            }

            // Check if this position is already counted
            if counted.contains(&target) {
                continue;
            }

            // Check if target is a desired Object, if so calculate points
            match world.objects[index] {
                Some((_, target_id)) => {
                    // House and BigHouse are treated as the same building, just with different points
                    if let ObjectID::House = id {
                        if target_id == id {
                            count += 1;
                        } else if target_id == ObjectID::BigHouse {
                            count += 2;
                        }

                        counted.push(target);
                    }
                    // Otherwise calculate points the normal way
                    else {
                        if target_id == id {
                            count += 1;
                        }

                        counted.push(target);
                    }
                }
                None => continue,
            }
        }
    }

    return count;
}

/************************************************************
 * Object Rules
 */

fn valid_tiles_for_n_number_of_neighbour_rule(
    entity: Entity,
    self_id: ObjectID,
    target_id: ObjectID,
    required: usize,
    oas: &ObjectAssetServer,
    world: &World,
    grid: &Grid,
) -> Vec<IVec2> {
    let asset = oas.get(self_id);

    let mut valid = vec![];

    for i in 0..(grid.size.0 * grid.size.1) {
        let position = IVec2::new((i % grid.size.0) as i32, (i / grid.size.0) as i32);

        // Count the neighbours for current position
        let count =
            count_neighbour_id(entity, target_id, position, &asset.conf.occupy, world, grid);

        if count >= required {
            // Push every tile Objet can occupy as a valid one
            let y_mod = position.y % 2;
            for (i, cell) in asset.conf.occupy.iter().enumerate() {
                valid.push(IVec2::new(
                    position.x + (if i == 0 { cell.x } else { cell.x + y_mod }),
                    position.y + cell.y,
                ));
            }
        }
    }

    valid
}

fn valid_tiles_for_adjacted_rule(position: IVec2, world: &World, grid: &Grid) -> Vec<IVec2> {
    let adjected = get_adjected(position);

    let mut valid = vec![];
    for (x, y) in adjected {
        let target = IVec2::new(position.x + x, position.y + y);

        if (target.x < 0 || target.x >= grid.size.0 as i32)
            || (target.y < 0 || target.y >= grid.size.1 as i32)
        {
            continue;
        }

        let index = ((target.y * world.size.0 as i32) + target.x) as usize;

        if grid.grid[index] == 0 {
            continue;
        }
        if !(world.objects[index].is_none() || (x == 0 && y == 0)) {
            continue;
        }

        valid.push(target);
    }

    valid
}
