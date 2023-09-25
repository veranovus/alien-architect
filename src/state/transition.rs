use crate::{
    animation::{Animate, AnimationMode},
    global,
    render::{RenderLayer, RENDER_LAYER},
    state::AppState,
};
use bevy::{ecs::query::QuerySingleError, prelude::*, sprite::Anchor};

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_event::<SceneTransitionEvent>()
            .add_systems(PreStartup, setup_transition_asset_server)
            .add_systems(PreUpdate, update_scene_transition)
            .add_systems(PostUpdate, handle_scene_transition_event);
    }
}

/************************************************************
 * - Constants
 */

const TRANSITION_TEXTURE_ATLAS_DESCS: [(&str, (usize, usize)); 3] = [
    ("ui/transitions/screen_fade.png", (21, 1)),
    ("ui/transitions/white_screen_fade.png", (21, 1)),
    ("ui/transitions/screen_wipe.png", (14, 1)),
];

const DEFAULT_TRANSITION_INTERVAL: f32 = 0.15;

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
struct TransitionAssetServer {
    assets: Vec<Handle<TextureAtlas>>,
}

impl TransitionAssetServer {
    fn new() -> Self {
        Self { assets: vec![] }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransitionEffect {
    Fade = 0,
    WhiteFade,
    Wipe,
}

#[derive(Debug, Event)]
pub struct SceneTransitionEvent {
    effect: TransitionEffect,
    next: AppState,
}

impl SceneTransitionEvent {
    pub fn new(effect: TransitionEffect, next: AppState) -> Self {
        Self { effect, next }
    }
}

#[derive(Debug, Component)]
pub struct SceneTransition {
    half: Timer,
    next: AppState,
}

impl SceneTransition {
    fn new(duration: f32, next: AppState) -> Self {
        Self {
            half: Timer::from_seconds(duration, TimerMode::Once),
            next,
        }
    }
}

/************************************************************
 * - System Functions
 */

fn setup_transition_asset_server(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let mut tas = TransitionAssetServer::new();

    for (path, atlas) in TRANSITION_TEXTURE_ATLAS_DESCS {
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load(path),
            Vec2::new(
                global::window::VIEWPORT_RESOLUTION.0 as f32,
                global::window::VIEWPORT_RESOLUTION.1 as f32,
            ),
            atlas.0,
            atlas.1,
            None,
            None,
        );

        tas.assets.push(texture_atlases.add(texture_atlas));
    }

    commands.insert_resource(tas);
}

fn handle_scene_transition_event(
    mut commands: Commands,
    mut event_reader: EventReader<SceneTransitionEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    query: Query<&SceneTransition>,
    tas: Res<TransitionAssetServer>,
) {
    if event_reader.is_empty() {
        return;
    }

    if !query.is_empty() {
        return;
    }

    let event = event_reader.iter().next().unwrap();

    app_state.set(AppState::Transition);

    spawn_new_scene_transition(&mut commands, &tas, event);

    event_reader.clear();
}

fn update_scene_transition(
    mut query: Query<&mut SceneTransition>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    // Get SceneTransition
    let mut st = match query.get_single_mut() {
        Ok(st) => st,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple SceneTransitions are deteced.")
        }
        Err(QuerySingleError::NoEntities(_)) => return,
    };

    st.half.tick(time.delta());

    if st.half.just_finished() {
        app_state.set(st.next);
    }
}

/************************************************************
 * - Helper Functions
 */

fn spawn_new_scene_transition(
    commands: &mut Commands,
    tas: &TransitionAssetServer,
    event: &SceneTransitionEvent,
) -> Entity {
    let (_, texture_atlas_size) = TRANSITION_TEXTURE_ATLAS_DESCS[event.effect as usize];

    let frame_count = texture_atlas_size.0 * texture_atlas_size.1;
    let transition_duration = (frame_count as f32 / 2.0) * DEFAULT_TRANSITION_INTERVAL;

    return commands
        .spawn((
            SpriteSheetBundle {
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    RENDER_LAYER[RenderLayer::Overlay as usize] as f32,
                ),
                texture_atlas: tas.assets[event.effect as usize].clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                ..Default::default()
            },
            Animate::new(
                frame_count,
                DEFAULT_TRANSITION_INTERVAL,
                AnimationMode::Delete,
            ),
            SceneTransition::new(transition_duration, event.next),
            Name::new("Scene Transition Effect"),
        ))
        .id();
}
