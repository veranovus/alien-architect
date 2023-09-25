use crate::{
    render::{RenderLayer, RENDER_LAYER},
    state::AppState,
};
use bevy::{prelude::*, sprite::Anchor};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::End), load_end)
            .add_systems(OnExit(AppState::End), unload_end);
    }
}

/************************************************************
 * - Constants
 */

const END_IMAGE_PATH: &str = "ui/end_screen.png";

/************************************************************
 * - Types
 */

#[derive(Debug, Component)]
struct EndScreen;

impl EndScreen {
    fn new() -> Self {
        Self
    }
}

/************************************************************
 * - Constantss
 */

fn load_end(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, RENDER_LAYER[RenderLayer::UI as usize] as f32),
            texture: asset_server.load(END_IMAGE_PATH),
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            ..Default::default()
        },
        EndScreen::new(),
        Name::new("End Screen"),
    ));
}

fn unload_end(mut commands: Commands, query: Query<Entity, With<EndScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
