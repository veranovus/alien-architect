use bevy::prelude::*;

pub mod game_ui;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(game_ui::GameUIPlugin);
    }
}
