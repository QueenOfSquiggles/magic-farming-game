use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use serde::{Deserialize, Serialize};

use crate::data::game_asset_path::GameAssetPath;

pub mod drops;
pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins()
        app.add_plugins(RonAssetPlugin::<ItemData>::new(&["item.ron"]));
    }
}

#[derive(Asset, Serialize, Deserialize, Reflect, Hash, Clone, PartialEq, Debug)]
pub struct ItemData {
    pub id: ItemId,
    pub icon: String,
    pub model: Option<String>,
}

#[derive(Component, Clone, PartialEq)]
pub struct Item(pub Handle<ItemData>);

#[derive(Serialize, Deserialize, Reflect, Hash, Clone, PartialEq, Eq, Debug)]
pub struct ItemId(pub String);

impl Item {
    pub fn from_id(id: ItemId, assets: &Res<AssetServer>) -> Self {
        Self(assets.load(GameAssetPath::new_data(format!("::items/{}", id.0))))
    }
}
