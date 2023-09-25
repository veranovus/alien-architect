use crate::{
    global::window,
    render::{RenderLayer, RENDER_LAYER},
    scene::level::{Level, Score, TurnCounter},
    state::AppState,
};
use bevy::{prelude::*, sprite::Anchor};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_ui_number_texture_atlas)
            .add_systems(OnEnter(AppState::Game), load_game_ui)
            .add_systems(OnExit(AppState::Game), unload_game_ui)
            .add_systems(
                PostUpdate,
                (
                    update_ui_numbers::<UINumberScore, Score>,
                    update_ui_numbers::<UINumberLevel, Level>,
                    update_ui_numbers::<UINumberTurn, TurnCounter>,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

/************************************************************
 * - Traits
 */

pub trait GameUINumberValue {
    fn value(&self) -> usize;
}

/************************************************************
 * - Constants
 */

const UI_IMAGES_PATHS: [(&str, (usize, usize)); 4] = [
    ("ui/game/panel.png", (0, 0)),
    ("ui/game/score.png", (33, 7)),
    ("ui/game/day.png", (28, 7)),
    ("ui/game/turn.png", (32, 7)),
];

const UI_TEXT_IMAGE_OFFSET: usize = 13;

const UI_NUMBER_TEXTURE_ATLAS_PATH: &str = "ui/game/numbers.png";

const UI_NUMBER_TEXTURE_ATLAS_TILE: (usize, usize) = (6, 7);

const UI_NUMBER_TEXTURE_ATLAS_SIZE: (usize, usize) = (10, 1);

const UI_TEXT_NUMBER_OFFSET: usize = 4;

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
struct UINumberTextureAtlas(Handle<TextureAtlas>);

#[derive(Debug, Component)]
struct GameUI;

#[derive(Debug, Component)]
struct UINumberScore;

#[derive(Debug, Component)]
struct UINumberLevel;

#[derive(Debug, Component)]
struct UINumberTurn;

/************************************************************
 * - System Functions
 */

fn setup_ui_number_texture_atlas(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load(UI_NUMBER_TEXTURE_ATLAS_PATH),
        Vec2::new(
            UI_NUMBER_TEXTURE_ATLAS_TILE.0 as f32,
            UI_NUMBER_TEXTURE_ATLAS_TILE.1 as f32,
        ),
        UI_NUMBER_TEXTURE_ATLAS_SIZE.0,
        UI_NUMBER_TEXTURE_ATLAS_SIZE.1,
        None,
        None,
    );

    commands.insert_resource(UINumberTextureAtlas(texture_atlases.add(texture_atlas)));
}

fn update_ui_numbers<T: Component, U: Resource + GameUINumberValue>(
    mut c_query: Query<&mut TextureAtlasSprite>,
    p_query: Query<&Children, With<T>>,
    resource: Res<U>,
) {
    if !resource.is_changed() {
        return;
    }

    for children in &p_query {
        let nums = format!("{:0width$}", resource.value(), width = 3);

        for (i, child) in children.iter().enumerate() {
            let mut sprite = c_query.get_mut(*child).unwrap();

            let num = (nums.as_bytes()[i] as char).to_digit(10).unwrap();

            sprite.index = num as usize;
        }
    }
}

fn load_game_ui(
    mut commands: Commands,
    number_texture_atlas: Res<UINumberTextureAtlas>,
    asset_server: Res<AssetServer>,
) {
    let mut children = vec![];

    let panelw = window::VIEWPORT_RESOLUTION.0;
    let offset = panelw / 3;
    let ui_number_w = (offset - (3 * UI_NUMBER_TEXTURE_ATLAS_TILE.0)) / 2;

    let mut index = 0;

    // Panel
    let ui_desc = UI_IMAGES_PATHS[index];
    children.push(spawn_ui_node(
        Vec2::new(0.0, 0.0),
        index,
        ui_desc.0,
        "Panel",
        &mut commands,
        &asset_server,
    ));
    index += 1;

    // Score Text
    let ui_desc = UI_IMAGES_PATHS[index];
    children.push(spawn_ui_node(
        Vec2::new(
            ((offset * (index - 1)) + ((offset - (ui_desc.1).0) / 2)) as f32,
            UI_TEXT_IMAGE_OFFSET as f32,
        ),
        index,
        ui_desc.0,
        "Score Text",
        &mut commands,
        &asset_server,
    ));

    let number_entity = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                ((offset * (index - 1)) + ui_number_w) as f32,
                UI_TEXT_NUMBER_OFFSET as f32,
                (RENDER_LAYER[RenderLayer::UI as usize] + index as u32) as f32,
            )),
            UINumberScore,
            Name::new("UI Score Numbers"),
        ))
        .id();
    children.push(number_entity);

    let numbers = spawn_ui_number_node(number_texture_atlas.0.clone(), &mut commands);
    commands.entity(number_entity).push_children(&numbers);

    index += 1;

    // Level Text
    let ui_desc = UI_IMAGES_PATHS[index];
    children.push(spawn_ui_node(
        Vec2::new(
            ((offset * (index - 1)) + ((offset - (ui_desc.1).0) / 2)) as f32,
            UI_TEXT_IMAGE_OFFSET as f32,
        ),
        index,
        ui_desc.0,
        "Level Text",
        &mut commands,
        &asset_server,
    ));

    let number_entity = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                ((offset * (index - 1)) + ui_number_w) as f32,
                UI_TEXT_NUMBER_OFFSET as f32,
                (RENDER_LAYER[RenderLayer::UI as usize] + index as u32) as f32,
            )),
            UINumberLevel,
            Name::new("UI Level Numbers"),
        ))
        .id();
    children.push(number_entity);

    let numbers = spawn_ui_number_node(number_texture_atlas.0.clone(), &mut commands);
    commands.entity(number_entity).push_children(&numbers);

    index += 1;

    // Turn Text
    let ui_desc = UI_IMAGES_PATHS[index];
    children.push(spawn_ui_node(
        Vec2::new(
            ((offset * (index - 1)) + ((offset - (ui_desc.1).0) / 2)) as f32,
            UI_TEXT_IMAGE_OFFSET as f32,
        ),
        index,
        ui_desc.0,
        "Turn Text",
        &mut commands,
        &asset_server,
    ));

    let number_entity = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                ((offset * (index - 1)) + ui_number_w) as f32,
                UI_TEXT_NUMBER_OFFSET as f32,
                (RENDER_LAYER[RenderLayer::UI as usize] + index as u32) as f32,
            )),
            UINumberTurn,
            Name::new("UI Turn Numbers"),
        ))
        .id();
    children.push(number_entity);

    let numbers = spawn_ui_number_node(number_texture_atlas.0.clone(), &mut commands);
    commands.entity(number_entity).push_children(&numbers);

    // Game UI
    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            GameUI,
            Name::new("Game UI"),
        ))
        .push_children(&children);
}

fn unload_game_ui(mut commands: Commands, query: Query<Entity, With<GameUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/************************************************************
 * - Helper Functions
 */

fn spawn_ui_number_node(handle: Handle<TextureAtlas>, commands: &mut Commands) -> Vec<Entity> {
    let mut ids = vec![];
    for i in 0..3 {
        ids.push(
            commands
                .spawn((
                    SpriteSheetBundle {
                        transform: Transform::from_xyz(
                            (i * UI_NUMBER_TEXTURE_ATLAS_TILE.0) as f32,
                            0.0,
                            0.0,
                        ),
                        texture_atlas: handle.clone(),
                        sprite: TextureAtlasSprite {
                            index: 0,
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Name::new(format!("UI Number #{}", i)),
                ))
                .id(),
        );
    }
    return ids;
}

fn spawn_ui_node(
    position: Vec2,
    order: usize,
    path: &str,
    name: &str,
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    position.x,
                    position.y,
                    (RENDER_LAYER[RenderLayer::UI as usize] + order as u32) as f32,
                ),
                texture: asset_server.load(path),
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                ..Default::default()
            },
            Name::new(name.to_string()),
        ))
        .id()
}
