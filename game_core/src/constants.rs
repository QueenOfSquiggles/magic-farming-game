use app_dirs2::{AppDataType, AppInfo};
use bevy::log::{error, info};
use lazy_static::lazy_static;
use std::path::PathBuf;

pub const APP_INFO: AppInfo = AppInfo {
    name: "MagicalFarmingGame",
    author: "QueenOfSquiggles",
};

lazy_static! {
    pub static ref CONFIG_DIR: Option<PathBuf> = {
        match app_dirs2::app_root(AppDataType::UserConfig, &APP_INFO) {
            Ok(path) => {
                info!("Located user config folder: {}", path.display());
                Some(path)
            }
            Err(err) => {
                error!("Failed to locate user config folder: {}", err);
                None
            }
        }
    };
}
