use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use data::CropDefinition;
use systems::add_test_crop;
use systems::initialize_crops;
use systems::update_crops;

pub mod components;
pub mod data;
pub mod systems;
pub struct CropsPlugin;

impl Plugin for CropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<CropDefinition>::new(&[".json"]));
        app.add_systems(Startup, add_test_crop);
        if false {
            // dumb little toggle for me
            app.add_systems(Startup, systems::emit_data_file);
        }
        app.add_systems(Update, initialize_crops);
        app.observe(update_crops);
    }
}
