use bevy::prelude::*;

mod global;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: bevy::window::WindowResolution::new(
                            global::window::RESOLUTION.0 as f32,
                            global::window::RESOLUTION.1 as f32,
                        ),
                        title: format!("{} | {}", global::window::TITLE, global::app::PKG_VERSION),
                        resizable: global::window::RESIZABLE,
                        present_mode: global::window::PRESENT_MODE,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb(
            206.0 / 255.0,
            192.0 / 255.0,
            167.0 / 255.0,
        )))
        .add_systems(PreStartup, (setup_camera, setup_tiles))
        .run();
}

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        // 60.0, 28.0 + 9.0
        transform: Transform::from_xyz(60.0, 28.0 + 9.0, 0.0),
        texture: asset_server.load("temp/tile.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(30.0, 28.0 + 9.0, 0.0),
        texture: asset_server.load("temp/tile.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 28.0 + 9.0, 0.0),
        texture: asset_server.load("temp/tile.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 14.0 + 9.0, 0.0),
        texture: asset_server.load("temp/tile.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0 + 9.0, 0.0),
        texture: asset_server.load("temp/tile.png"),
        ..Default::default()
    });
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 1.0 / global::window::SCALE_FACTOR as f32;

    commands.spawn(camera);
}

/*
6x 5
5x 4

Total Tile: 60

08 -- 06 -- 04 -- 02 -- 00
-- 07 -- 05 -- 03 -- 01 --
17 -- 15 -- 13 -- 11 -- 09
-- 16 -- 14 -- 12 -- 10 --
00 -- 00 -- 00 -- 00 -- 00
-- 00 -- 00 -- 00 -- 00 --
*/

/*
I  => Index
HW => Half Width
FH => Full Height
HH => Half Height

Rule -> (I % 9 * HW)
Rule -> (I % 9 * FH) + (I % 9) % 2 == 1 => HH,
*/
