use bevy::prelude::*;

pub mod transition;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(transition::TransitionPlugin);
    }
}

/************************************************************
 * - Types
 */

#[derive(States, PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    Preload,
    Splash,
    Title,
    Credits,
    Game,
    End,
    Transition,
}
