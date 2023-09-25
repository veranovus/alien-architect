use bevy::prelude::*;

pub mod ufo;
mod warn;
pub mod win;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(warn::WarningPlugin)
            .add_plugins(win::WinPlugin)
            .add_plugins(ufo::UFOPlugin);
    }
}

/************************************************************
 * - Types
 */

#[derive(States, PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerControlled,
    ObjectControlled,
    Paused,
}
