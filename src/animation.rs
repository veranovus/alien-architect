use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, animate);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectAnimationDesc {
    pub image_size: (usize, usize),
    pub atlas_size: (usize, usize),
    pub interval: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum AnimationMode {
    Default,
    Loop,
    Delete,
}

#[derive(Debug, Component)]
pub struct Animate {
    pub timer: Timer,
    pub current_frame: usize,
    pub frame_count: usize,
    mode: AnimationMode,
}

impl Animate {
    pub fn new(frame_count: usize, interval: f32, mode: AnimationMode) -> Self {
        Self {
            timer: Timer::from_seconds(interval, TimerMode::Repeating),
            current_frame: 0,
            frame_count,
            mode,
        }
    }
}

/************************************************************
 * - System Functions
 */

fn animate(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Animate, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (entity, mut animate, mut sprite) in &mut query {
        animate.timer.tick(time.delta());

        if animate.timer.just_finished() {
            animate.current_frame += 1;

            if animate.current_frame >= animate.frame_count {
                match animate.mode {
                    AnimationMode::Default => {
                        animate.current_frame = animate.frame_count - 1;
                    }
                    AnimationMode::Loop => {
                        animate.current_frame = 0;
                    }
                    AnimationMode::Delete => {
                        commands.entity(entity).despawn_recursive();
                        continue;
                    }
                }
            }

            *sprite = TextureAtlasSprite {
                index: animate.current_frame,
                anchor: sprite.anchor.clone(),
                ..Default::default()
            };
        }
    }
}
