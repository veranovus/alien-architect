use crate::object::ObjectSelectEvent;
use crate::world::grid::Grid;
use crate::world::tile::{TileState, TileStateChangeEvent};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_ufo);
    }
}

/************************************************************
 * - Types
 */

const UFO_TEXUTRE_PATH: &str = "ufo.png";

const UFO_SPRITE_OFFSET: (i32, i32) = (5, 4 + 23);

/************************************************************
 * - Types
 */

#[derive(Debug, Component)]
pub struct UFO {
    position: IVec2,
    selected: bool,
    offset: IVec2,
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
                    selected: false,
                    offset: IVec2::new(UFO_SPRITE_OFFSET.0, UFO_SPRITE_OFFSET.1),
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
    mut query: Query<(&mut UFO, &mut Transform)>,
    mut tile_event_writer: EventWriter<TileStateChangeEvent>,
    mut objc_event_writer: EventWriter<ObjectSelectEvent>,
    grid: Res<Grid>,
    keyboard: Res<Input<KeyCode>>,
) {
    let (mut ufo, mut transform) = match query.get_single_mut() {
        Ok(tuple) => tuple,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    let even = ufo.position.y % 2 == 0;
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

    if !moved {
        return;
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

    if !ufo.selected {
        tile_event_writer.send(TileStateChangeEvent::new(ufo.position, TileState::Default));
        tile_event_writer.send(TileStateChangeEvent::new(position, TileState::Selected));

        objc_event_writer.send(ObjectSelectEvent::new(position));
    }

    ufo.position.x = position.x;
    ufo.position.y = position.y;

    let pos = grid.cell_to_world(UVec2::new(ufo.position.x as u32, ufo.position.y as u32));

    transform.translation.x = pos.x + ufo.offset.x as f32;
    transform.translation.y = pos.y + ufo.offset.y as f32;
}
