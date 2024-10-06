use bevy::{asset::Asset, reflect::TypePath};

use crate::data::range::Range;

use super::components::CropStatus;

#[derive(Asset, Debug, Clone, serde::Serialize, serde::Deserialize, TypePath)]
pub struct CropDefinition {
    pub id: String,
    pub stages: Vec<CropStage>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize, TypePath)]
pub struct CropStage {
    pub model: String,
    pub duration: Range,
    pub begin_status: Option<CropStatus>,
}
