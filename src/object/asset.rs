use crate::object::ObjectID;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_object_assets);
    }
}

/************************************************************
 * - Constants
 */

const OBJECT_CONFIG_PATH: &str = "assets/object-conf.ron";

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
pub struct ObjectAssetServer {
    assets: HashMap<ObjectID, ObjectAsset>,
}

impl ObjectAssetServer {
    fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn get(&self, id: ObjectID) -> &ObjectAsset {
        return match self.assets.get(&id) {
            None => panic!("Failed to get asset for ObjectID::{}.", id.to_string()),
            Some(asset) => asset,
        };
    }
}

/************************************************************
 * - System Functions
 */

fn load_object_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut oas = ObjectAssetServer::new();

    let aconf: AssetConf = if let Ok(contents) = std::fs::read_to_string(ASSET_CONFIG_PATH) {
        ron::from_str(&contents).unwrap()
    } else {
        panic!(
            "Failed to read file to load AssetConf, `{}`.",
            ASSET_CONFIG_PATH
        );
    };

    for desc in &aconf.descs {
        oas.assets.insert(
            desc.id,
            ObjectAsset::new(asset_server.load(&desc.path), desc.origin),
        );
    }

    commands.insert_resource(oas);
}
