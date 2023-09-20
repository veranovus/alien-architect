use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod global;
mod object;
mod player;
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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .add_plugins(render::RenderPlugin)
        .add_plugins(world::WorldPlugin)
        //.add_plugins(object::ObjectPlugin)
        .add_systems(PreStartup, setup_camera)
        .add_systems(Update, control_camera)
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

fn control_camera(
    mut query: Query<&mut Transform, With<Camera>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 20.0 * time.delta_seconds();
    let mut camera = query.get_single_mut().unwrap();

    if keyboard.pressed(KeyCode::D) {
        camera.translation.x += speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation.x -= speed;
    }

    if keyboard.pressed(KeyCode::W) {
        camera.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation.y -= speed;
    }
}
