use bevy::prelude::*;

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
