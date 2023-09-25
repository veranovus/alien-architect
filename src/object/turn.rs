use super::{asset::ObjectAssetServer, validate_position, Object, ObjectID};
use crate::{
    object::get_adjected,
    player::win::PlayerWinEvent,
    render::{RenderLayer, RENDER_LAYER},
    state::AppState,
    world::{grid::Grid, World},
};
use bevy::prelude::*;
use rand::prelude::*;

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ObjectsActTurnsEvent>().add_systems(
            PostUpdate,
            handle_objects_act_turns_event.run_if(in_state(AppState::Game)),
        );
    }
}

/************************************************************
 * - Constants
 */

const OBJECT_RANDOM_MOVE_CHANCE: f32 = 0.6;
const OBJECT_RANDOM_MOVE_MAXIMUM_ITER: usize = 250;

/************************************************************
 * - Types
 */

#[derive(Debug, Event)]
pub struct ObjectsActTurnsEvent;

impl ObjectsActTurnsEvent {
    pub fn new() -> Self {
        Self
    }
}

/************************************************************
 * - System Functions
 */

#[allow(unused_variables, unused_mut)]
fn handle_objects_act_turns_event(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Object, &mut Transform)>,
    mut event_writer: EventWriter<PlayerWinEvent>,
    mut event_reader: EventReader<ObjectsActTurnsEvent>,
    mut world: ResMut<World>,
    oas: Res<ObjectAssetServer>,
    grid: Res<Grid>,
) {
    if event_reader.is_empty() {
        return;
    }
    event_reader.clear();

    let mut delete_queue = vec![];

    let mut sorted = vec![];
    for tuple in &mut query {
        sorted.push(match tuple.1.id {
            ObjectID::Villager => (0, tuple),
            ObjectID::Cow => (1, tuple),
            ObjectID::Assassin => (2, tuple),
            ObjectID::King => (3, tuple),
            _ => continue,
        });
    }
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, (entity, mut object, mut transform)) in sorted {
        match object.id {
            ObjectID::King => {
                let path = find_path(&object, &world, &grid);
                match path {
                    Some((path, _)) => {
                        event_writer.send(PlayerWinEvent::new(path));
                    }
                    None => {}
                };
            }
            ObjectID::Villager => {
                move_to_random_adjected_tile(
                    entity,
                    &mut object,
                    &mut transform,
                    &mut world,
                    &grid,
                );
            }
            ObjectID::Cow => {
                move_to_random_adjected_tile(
                    entity,
                    &mut object,
                    &mut transform,
                    &mut world,
                    &grid,
                );
            }
            ObjectID::Assassin => {
                let mut targets = get_entities_to_kill(&mut world, &object, &grid);

                delete_queue.append(&mut targets);
            }
            _ => {}
        }
    }

    for entity in delete_queue {
        commands.entity(entity).despawn_recursive();
    }
}

/************************************************************
 * - Helper Functions
 */

fn get_diagonal_adjected(position: IVec2) -> [(i32, i32); 4] {
    let ymod = position.y % 2;
    return [
        (0 + ymod, 1),
        (0 + ymod, -1),
        (-1 + ymod, 1),
        (-1 + ymod, -1),
    ];
}

fn find_path(object: &Object, world: &World, grid: &Grid) -> Option<(Vec<IVec2>, i32)> {
    let mut target = IVec2::ZERO;
    for i in 0..(grid.size.0 * grid.size.1) {
        match world.objects[i as usize] {
            Some((_, id)) => match id {
                ObjectID::Castle => {
                    target = IVec2::new((i % grid.size.0) as i32, (i / grid.size.0) as i32);
                    break;
                }
                _ => continue,
            },
            None => continue,
        }
    }

    return pathfinding::prelude::dijkstra(
        &object.occupied[0],
        |&current| {
            let adjected = get_diagonal_adjected(current);
            let mut vec = vec![];

            for offset in adjected {
                let pos = IVec2::new(current.x + offset.0, current.y + offset.1);

                // Validate position
                let (valid, index) = validate_position(pos, grid);

                if !valid {
                    continue;
                }

                match world.objects[index] {
                    Some((_, id)) => match id {
                        ObjectID::Castle | ObjectID::King => {}
                        _ => {
                            continue;
                        }
                    },
                    None => {}
                }

                vec.push(pos);
            }

            vec.into_iter().map(|pos| (pos, 1))
        },
        |&pos| pos == target,
    );
}

fn get_entities_to_kill(world: &mut World, object: &Object, grid: &Grid) -> Vec<Entity> {
    let mut targets = vec![];
    let adjected = get_adjected(object.occupied[0]);

    for offset in adjected {
        let position = IVec2::new(
            object.occupied[0].x + offset.0,
            object.occupied[0].y + offset.1,
        );

        // Validate postion
        let (valid, index) = validate_position(position, grid);

        if !valid {
            continue;
        }

        match world.objects[index] {
            Some((entity, id)) => match id {
                ObjectID::King | ObjectID::Villager | ObjectID::Cow => {
                    // Push id to targets
                    targets.push(entity);

                    // Delete the Object from the World
                    world.objects[index] = None;
                }
                _ => {}
            },
            None => continue,
        }
    }

    return targets;
}

fn move_to_random_adjected_tile(
    entity: Entity,
    object: &mut Object,
    transform: &mut Transform,
    world: &mut World,
    grid: &Grid,
) {
    // Pick a random spot to move
    let mut rng = rand::thread_rng();

    let adjected = get_diagonal_adjected(object.occupied[0]);

    let prob: f32 = rng.gen();

    if prob < (1.0 - OBJECT_RANDOM_MOVE_CHANCE) {
        return;
    }

    let mut iter = 0;
    loop {
        if iter >= OBJECT_RANDOM_MOVE_MAXIMUM_ITER {
            break;
        }

        let offset = adjected[rng.gen_range(0..adjected.len())];
        let target = IVec2::new(
            object.occupied[0].x + offset.0,
            object.occupied[0].y + offset.1,
        );

        // Validate position
        let (valid, index) = validate_position(target, grid);

        if !valid {
            iter += 1;
            continue;
        }

        if !world.objects[index].is_none() {
            iter += 1;
            continue;
        }

        // Move the object
        world.objects[index] = Some((entity, object.id));

        let index = ((object.occupied[0].y * grid.size.0 as i32) + object.occupied[0].x) as usize;
        world.objects[index] = None;

        object.occupied = vec![target];

        let world_postition = grid.cell_to_world(UVec2::new(target.x as u32, target.y as u32));
        let order = grid.cell_order(UVec2::new(target.x as u32, target.y as u32));

        transform.translation.x = world_postition.x + object.offset.x as f32;
        transform.translation.y = world_postition.y + object.offset.y as f32;
        transform.translation.z = (RENDER_LAYER[RenderLayer::Entity as usize] + order) as f32;

        break;
    }
}
