use super::ufo::{control_ufo, UFO, UFO_TEXTURE_ATLAS_TILE};
use crate::render::{RenderLayer, RENDER_LAYER};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct WarningPlugin;

impl Plugin for WarningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWarningEvent>()
            .add_systems(
                Update,
                (
                    update_warning_indicator_lifetimes,
                    update_warning_indicator_positions,
                )
                    .after(control_ufo),
            )
            .add_systems(PostUpdate, handle_spawn_warning_event);
    }
}

/************************************************************
 * - Constants
 */

const WARNING_IMAGE_PATH: &str = "exclamation_mark.png";

const WARNING_IMAGE_SIZE: (usize, usize) = (6, 10);

const WARNING_LIFESPAN: f32 = 0.6;

/************************************************************
 * - Types
 */

#[derive(Debug, Event)]
pub struct SpawnWarningEvent;

impl SpawnWarningEvent {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Component)]
pub struct Warning {
    timer: Timer,
}

impl Warning {
    pub fn new(
        commands: &mut Commands,
        asset_server: &AssetServer,
        ufo_transform: &Transform,
    ) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        ufo_transform.translation.x,
                        ufo_transform.translation.y,
                        RENDER_LAYER[RenderLayer::UI as usize] as f32,
                    ),
                    texture: asset_server.load(WARNING_IMAGE_PATH),
                    sprite: Sprite {
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Warning {
                    timer: Timer::from_seconds(WARNING_LIFESPAN, TimerMode::Once),
                },
                Name::new("Warning Indicator"),
            ))
            .id()
    }
}

/************************************************************
 * - System Functions
 */

fn update_warning_indicator_positions(
    mut ind_query: Query<&mut Transform, (With<Warning>, Without<UFO>)>,
    ufo_query: Query<&Transform, With<UFO>>,
) {
    let ufo = match ufo_query.get_single() {
        Ok(ufo) => ufo,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple UFOs are present in the scene.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    for mut indicator in &mut ind_query {
        indicator.translation.x =
            ufo.translation.x + ((UFO_TEXTURE_ATLAS_TILE.0 - WARNING_IMAGE_SIZE.0) / 2) as f32;
        indicator.translation.y =
            ufo.translation.y + (UFO_TEXTURE_ATLAS_TILE.1 + (WARNING_IMAGE_SIZE.1 / 2)) as f32;
    }
}

fn update_warning_indicator_lifetimes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Warning)>,
    time: Res<Time>,
) {
    for (entity, mut warning) in &mut query {
        warning.timer.tick(time.delta());

        if warning.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_spawn_warning_event(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnWarningEvent>,
    ind_query: Query<Entity, (With<Warning>, Without<UFO>)>,
    ufo_query: Query<&Transform, With<UFO>>,
    asset_server: Res<AssetServer>,
) {
    for _event in &mut event_reader {
        // Get UFO
        let ufo = match ufo_query.get_single() {
            Ok(ufo) => ufo,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("Multiple UFOs are present in the scene.")
            }
            Err(QuerySingleError::NoEntities(_)) => return,
        };

        if !ind_query.is_empty() {
            break;
        }

        Warning::new(&mut commands, &asset_server, ufo);
        break;
    }
    event_reader.clear();
}
