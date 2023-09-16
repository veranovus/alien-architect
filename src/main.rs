use bevy::prelude::*;

/*
- List for ordering of the entity sprites:
    - Tile layer -> (0, 99)
    - Entity Layer -> (100, 199)
*/

mod global;
mod render;
mod world;

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
        .add_plugins(render::RenderPlugin)
        .add_plugins(world::WorldPlugin)
        .add_systems(PreStartup, setup_camera)
        .add_systems(Update, camera_movement)
        .run();
}

fn setup_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lrow: usize = 5;
    let lrow_cnt: usize = 6;

    let srow: usize = 4;
    let srow_cnt: usize = 6;

    let row: usize = lrow + srow;

    let total_tile = (lrow * lrow_cnt) + (srow * srow_cnt);

    // Full width of the sprite
    let fw = 30.0;
    // Half width of the sprite
    let hw = fw / 2.0;

    // Full height of the sprite
    let fh = 18.0;
    // Half height of the sprite
    let hh = fh / 2.0;
    // Transfer height: Distance from the center of one tile
    //                  to another tile in the same column.
    let fth = 14.0;
    // Half transfer height
    let hth = fth / 2.0;

    // Total size of the world area
    let area = (
        // Total width of the world tiles
        lrow as f32 * fw,
        // Total height for the world tiles
        fh + ((srow_cnt - 1) as f32 * fth) + hth,
    );

    let offset = (
        (global::window::VIEWPORT_RESOLUTION.0 as f32 - area.0) / 2.0,
        (global::window::VIEWPORT_RESOLUTION.1 as f32 - area.1) / 2.0,
    );

    let handle = asset_server.load("temp/tile.png");

    for i in 0..total_tile {
        let xpos = (i % row) as f32 * hw;
        let ypos = ((i / row) as f32 * fth) + (((i % row) % 2) as f32 * hth);
        let zpos = ((i / row) * 9) + ((i % row) / 2) + (((i % row) % 2) * 4);

        commands.spawn(SpriteBundle {
            transform: Transform::from_xyz(
                offset.0 + area.0 - xpos - hw,
                offset.1 + area.1 - ypos - hh,
                zpos as f32,
            ),
            texture: handle.clone(),
            ..Default::default()
        });
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 1.0 / global::window::SCALE_FACTOR as f32;

    camera.transform.translation = Vec3::new(
        global::window::VIEWPORT_RESOLUTION.0 as f32 / 2.0,
        global::window::VIEWPORT_RESOLUTION.1 as f32 / 2.0,
        0.0,
    );

    commands.spawn(camera);
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 15.0 * time.delta_seconds();

    for mut transform in &mut query {
        let mut moved = false;

        if keyboard.pressed(KeyCode::A) {
            transform.translation.x -= speed;
            moved = true;
        }
        if keyboard.pressed(KeyCode::D) {
            transform.translation.x += speed;
            moved = true;
        }

        if keyboard.pressed(KeyCode::S) {
            transform.translation.y -= speed;
            moved = true;
        }
        if keyboard.pressed(KeyCode::W) {
            transform.translation.y += speed;
            moved = true;
        }

        if moved {
            println!("TRANSFORM : {}", transform.translation);
        }
    }
}
