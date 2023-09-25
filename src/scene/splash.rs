use crate::{
    animation::{Animate, AnimationMode},
    global::window,
    render::{RenderLayer, RENDER_LAYER},
    state::transition::{SceneTransitionEvent, TransitionEffect},
    state::AppState,
};
use bevy::{ecs::query::QuerySingleError, prelude::*};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Splash), load_splash_scene)
            .add_systems(OnExit(AppState::Splash), unload_splash_scene)
            .add_systems(
                Update,
                update_splash_animation.run_if(in_state(AppState::Splash)),
            );
    }
}

/************************************************************
 * - Constants
 */

const SPLASH_TEXTURE_ATLAS_PATH: &str = "ui/gbjam_intro.png";

const SPLASH_TEXTURE_ATLAS_TILE: (usize, usize) =
    (window::VIEWPORT_RESOLUTION.0, window::VIEWPORT_RESOLUTION.1);

const SPLASH_TEXTURE_ATLAS_SIZE: (usize, usize) = (8, 1);

const SPLASH_ANIMATION_INTERVAL: f32 = 0.1;

const SPLASH_ANIMATION_DURATION: f32 = 2.0;

/************************************************************
 * - Types
 */

#[derive(Debug, Component)]
struct SplashAnimation {
    timer: Timer,
}

impl SplashAnimation {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

/************************************************************
 * - System Functions
 */

fn update_splash_animation(
    mut query: Query<&mut SplashAnimation>,
    mut event_writer: EventWriter<SceneTransitionEvent>,
    time: Res<Time>,
) {
    let mut splash = match query.get_single_mut() {
        Ok(s) => s,
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple SplashAnimations are present.");
        }
        Err(QuerySingleError::NoEntities(_)) => {
            return;
        }
    };

    splash.timer.tick(time.delta());

    if splash.timer.just_finished() {
        event_writer.send(SceneTransitionEvent::new(
            TransitionEffect::WhiteFade,
            AppState::Game,
        ));
    }
}

fn load_splash_scene(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load(SPLASH_TEXTURE_ATLAS_PATH),
        Vec2::new(
            SPLASH_TEXTURE_ATLAS_TILE.0 as f32,
            SPLASH_TEXTURE_ATLAS_TILE.1 as f32,
        ),
        SPLASH_TEXTURE_ATLAS_SIZE.0,
        SPLASH_TEXTURE_ATLAS_SIZE.1,
        None,
        None,
    );

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_xyz(0.0, 0.0, RENDER_LAYER[RenderLayer::UI as usize] as f32),
            texture_atlas: texture_atlases.add(texture_atlas),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..Default::default()
            },
            ..Default::default()
        },
        Animate::new(
            SPLASH_TEXTURE_ATLAS_SIZE.0,
            SPLASH_ANIMATION_INTERVAL,
            AnimationMode::Loop,
        ),
        SplashAnimation::new(SPLASH_ANIMATION_DURATION),
        Name::new("Splash Animation"),
    ));
}

fn unload_splash_scene(mut commands: Commands, query: Query<Entity, With<SplashAnimation>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
