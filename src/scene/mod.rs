use crate::state::AppState;
use bevy::prelude::*;

pub mod level;
pub mod splash;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(splash::SplashPlugin)
            .add_plugins(level::LevelPlugin)
            .add_systems(PostStartup, set_initial_scene);
    }
}

/************************************************************
 * - System Functions
 */

fn set_initial_scene(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Splash);
}
