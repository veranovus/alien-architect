use crate::audio::AudioMode;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {}
}

/************************************************************
 * - Types
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDesc {
    path: String,
    size: UVec2,
}

#[derive(Debug)]
pub struct TextureAsset {
    handle: Handle<Image>,
    desc: TextureDesc,
}

impl TextureAsset {
    fn new(desc: TextureDesc, asset_server: &AssetServer) -> TextureAsset {
        Self {
            handle: asset_server.load(&desc.path),
            desc,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureAtlasDesc {
    path: String,
    tile: UVec2,
    size: UVec2,
}

#[derive(Debug)]
pub struct TextureAtlasAsset {
    handle: Handle<TextureAtlas>,
    desc: TextureAtlasDesc,
}

impl TextureAtlasAsset {
    fn new(
        desc: TextureAtlasDesc,
        texture_atlases: &mut Assets<TextureAtlas>,
        asset_server: &AssetServer,
    ) -> Self {
        let texture_atlas = TextureAtlas::from_grid(
            asset_server.load(&desc.path),
            Vec2::new(desc.tile.y as f32, desc.tile.y as f32),
            desc.size.x as usize,
            desc.size.y as usize,
            None,
            None,
        );

        Self {
            handle: texture_atlases.add(texture_atlas),
            desc,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioDesc {
    path: String,
    mode: AudioMode,
    volume: f32,
}

#[derive(Debug)]
pub struct AudioAsset {
    handle: Handle<AudioSource>,
    desc: AudioDesc,
}

impl AudioAsset {
    fn new(desc: AudioDesc, asset_server: &AssetServer) -> Self {
        Self {
            handle: asset_server.load(&desc.path),
            desc,
        }
    }
}

#[derive(Debug, Resource)]
pub struct GameAssetServer {
}

impl GameAssetServer {
    fn get(path: &str) -> {
        
    }
}

/************************************************************
 * - System Functions
 */

fn load_assets() {}
