use crate::global;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (setup_camera, setup_background));
    }
}

/************************************************************
 * - Constants
 */

const BACKGROUND_IMAGE_PATH: &str = "background.png";

/************************************************************
 * - Constants
 */

#[derive(Debug, Component)]
struct MainCamera;

/************************************************************
 * - System Functions
 */

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 1.0 / global::window::SCALE_FACTOR as f32;

    camera.transform.translation = Vec3::new(
        global::window::VIEWPORT_RESOLUTION.0 as f32 / 2.0,
        global::window::VIEWPORT_RESOLUTION.1 as f32 / 2.0,
        0.0,
    );

    commands.spawn((camera, MainCamera, Name::new("Main Camera")));
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture: asset_server.load(BACKGROUND_IMAGE_PATH),
            ..Default::default()
        },
        Name::new("Background Image"),
    ));
}
