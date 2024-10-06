use std::{
    fs::{self, File},
    path::PathBuf,
};

use bevy::prelude::*;
use lazy_static::lazy_static;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameSettings {
    something: Option<u32>,
    another: String,
    and_more: Vec<f32>,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            something: Some(42),
            another: "What a lovely day it is?".into(),
            and_more: vec![0.1, 0.3, 0.4, 0.5, 100.0],
        }
    }
}

pub struct GameSettingsPlugin;

lazy_static! {
    static ref SETTINGS_FILE: Option<PathBuf> = constants::CONFIG_DIR
        .as_ref()
        .and_then(|path| Some(path.join("settings.ron")));
}

impl Plugin for GameSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(get_settings().unwrap_or_default());
    }
}

fn get_settings() -> Option<GameSettings> {
    let Some(path) = SETTINGS_FILE.as_ref() else {
        return None;
    };
    let Ok(file) = File::open(path) else {
        make_default_settings();
        return None;
    };
    let Ok(settings) = ron::de::from_reader(file) else {
        make_default_settings();
        return None;
    };

    Some(settings)
}

fn make_default_settings() {
    let Some(path) = SETTINGS_FILE.as_ref() else {
        return;
    };
    if !fs::exists(path).unwrap_or(false) {
        match path.parent() {
            Some(parent) => {
                let _ = fs::create_dir_all(parent);
            }
            None => {
                error!(
                    "failed to create directory structure for settings file path: {:?}",
                    SETTINGS_FILE.as_ref().and_then(|p| Some(p.display()))
                );
            }
        }
    }

    let settings = GameSettings::default();
    let Ok(file) = File::create(path) else {
        return;
    };
    if let Err(err) = ron::ser::to_writer_pretty(file, &settings, PrettyConfig::default()) {
        error!("Failed to write out setttings : {}", err);
    }
}
