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

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectConf {
    pub id: ObjectID,
    pub name: String,
    pub selectable: bool,
    pub animated: bool,
    pub occupied: Vec<UVec2>,
    pub offset: Vec2,
    assets: Vec<String>,
}

#[derive(Debug)]
pub struct ObjectAsset {
    pub conf: ObjectConf,
    pub assets: Vec<Handle<Image>>,
}

impl ObjectAsset {
    fn new(conf: ObjectConf, asset_server: &AssetServer) -> Self {
        let mut assets = vec![];
        for path in &conf.assets {
            assets.push(asset_server.load(path));
        }

        Self { conf, assets }
    }
}

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
            None => panic!("Failed to get object configuration for {}.", id.to_string()),
            Some(asset) => asset,
        };
    }
}

/************************************************************
 * - System Functions
 */

fn load_object_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut ocs = ObjectAssetServer::new();

    let confs: Vec<ObjectConf> = if let Ok(contents) = std::fs::read_to_string(OBJECT_CONFIG_PATH) {
        ron::from_str(&contents).unwrap()
    } else {
        panic!(
            "Failed to read file to load AssetConf, `{}`.",
            OBJECT_CONFIG_PATH
        );
    };

    for c in confs {
        ocs.assets.insert(c.id, ObjectAsset::new(c, &asset_server));
    }

    commands.insert_resource(ocs);
}
