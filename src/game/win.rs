use crate::{
    global::window,
    render::{RenderLayer, RENDER_LAYER},
    scene::level::{Level, Score, TurnCounter},
    state::{
        transition::{SceneTransitionEvent, TransitionEffect},
        AppState,
    },
    world::tile::{TileState, TileStateChangeEvent},
};
use bevy::{ecs::query::QuerySingleError, prelude::*, sprite::Anchor};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerWinEvent>()
            .add_systems(OnExit(AppState::Game), unload_win_animation)
            .add_systems(
                Update,
                update_win_animation.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                PostUpdate,
                handle_player_win_event.run_if(in_state(AppState::Game)),
            );
    }
}

/************************************************************
 * - Constants
 */

const WIN_TEXTURE_PATH: &str = "ui/game/complete_overlay.png";

const WIN_TEXTUER_SIZE: (usize, usize) = (160, 36);

const WIN_ANIMATION_DEFAULT_INTERVAL: f32 = 0.15;

const WIN_ANIMATION_TRANSITION_TIME: f32 = 2.0;

const MAXIMUM_TURN_POINT: i32 = 10;

/************************************************************
 * - Types
 */

#[derive(Debug, Event)]
pub struct PlayerWinEvent {
    path: Vec<IVec2>,
}

impl PlayerWinEvent {
    pub fn new(path: Vec<IVec2>) -> Self {
        Self { path }
    }
}

#[derive(Debug, Component)]
struct WinAnimation {
    path: Vec<IVec2>,
    tile_timer: Timer,
    trns_timer: Timer,
}

impl WinAnimation {
    fn new(path: Vec<IVec2>, interval: f32) -> Self {
        Self {
            tile_timer: Timer::from_seconds(interval, TimerMode::Repeating),
            trns_timer: Timer::from_seconds(
                (interval * path.len() as f32) + WIN_ANIMATION_TRANSITION_TIME,
                TimerMode::Once,
            ),
            path,
        }
    }
}

/************************************************************
 * - System Functions
 */

fn update_win_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut WinAnimation)>,
    mut trns_event_writer: EventWriter<SceneTransitionEvent>,
    mut tile_event_writer: EventWriter<TileStateChangeEvent>,
    mut level: ResMut<Level>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    let (entity, mut wa) = match query.get_single_mut() {
        Ok(wa) => wa,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Encountered multiple WinAnimation's in the scene.");
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    wa.tile_timer.tick(time.delta());

    if wa.tile_timer.just_finished() {
        match wa.path.pop() {
            Some(position) => {
                tile_event_writer.send(TileStateChangeEvent::new(position, TileState::Path));
            }
            None => {
                let id = commands
                    .spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                ((window::VIEWPORT_RESOLUTION.0 - WIN_TEXTUER_SIZE.0) / 2) as f32,
                                ((window::VIEWPORT_RESOLUTION.1 - WIN_TEXTUER_SIZE.1) / 2) as f32,
                                RENDER_LAYER[RenderLayer::UI as usize] as f32,
                            ),
                            texture: asset_server.load(WIN_TEXTURE_PATH),
                            sprite: Sprite {
                                anchor: Anchor::BottomLeft,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Name::new("Win Overlay"),
                    ))
                    .id();

                commands.entity(entity).add_child(id);

                wa.tile_timer.pause();
            }
        };
    }

    wa.trns_timer.tick(time.delta());

    if wa.trns_timer.just_finished() {
        trns_event_writer.send(SceneTransitionEvent::new(
            TransitionEffect::Fade,
            level.next(),
        ));
    }
}

fn handle_player_win_event(
    mut commands: Commands,
    mut event_reader: EventReader<PlayerWinEvent>,
    mut score: ResMut<Score>,
    turn_counter: Res<TurnCounter>,
    query: Query<&WinAnimation>,
) {
    if event_reader.is_empty() {
        return;
    }

    let event = event_reader.iter().next().unwrap();

    if query.is_empty() {
        let bonus = MAXIMUM_TURN_POINT - turn_counter.turn as i32;

        score.current += if bonus <= 0 { 1 } else { bonus } as usize;

        let mut path = event.path.clone();
        path.reverse();

        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            WinAnimation::new(path, WIN_ANIMATION_DEFAULT_INTERVAL),
            Name::new("Win Animation"),
        ));
    }

    event_reader.clear();
}

fn unload_win_animation(mut commands: Commands, query: Query<Entity, With<WinAnimation>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
