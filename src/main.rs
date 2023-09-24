use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation;
mod camera;
mod global;
mod object;
mod player;
mod render;
mod scene;
mod state;
mod ui;
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
                        mode: global::window::MODE,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Grave)),
        )
        .add_plugins(camera::CameraPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(scene::ScenePlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(object::ObjectPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}
