use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

/************************************************************
 * - Types
 */

const UFO_TEXUTRE_PATH: &str = "ufo.png";

/************************************************************
 * - Types
 */

#[derive(Debug, Component)]
pub struct UFO {
    position: IVec2,
}

impl UFO {
    pub fn new(
        position: Vec2,
        grid_position: IVec2,
        asset_server: &AssetServer,
        commands: &mut Commands,
    ) -> Entity {
        return commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(position.x, position.y, 300.0),
                    texture: asset_server.load(UFO_TEXUTRE_PATH),
                    ..Default::default()
                },
                UFO {
                    position: grid_position,
                },
                Name::new("UFO"),
            ))
            .id();
    }
}
