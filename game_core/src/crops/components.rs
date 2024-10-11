use bevy::prelude::*;

use crate::items::drops::ItemDrop;

use super::data::CropStage;

#[derive(Debug, Bundle, Default)]
pub struct CropBundle {
    pub data: CropData,
}

impl CropBundle {
    /// Creates a new crop bundle with the given stages, automatically initializing the supplemental components as they ought to be
    pub fn new(stages: impl IntoIterator<Item = CropStage>) -> Result<Self, ()> {
        let data = CropData::new(stages);
        if data.stages.is_empty() {
            return Err(());
        }

        Ok(Self { data })
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct CropData {
    pub id: String,
    pub stages: Vec<CropStage>,
    pub index: usize,
}

impl CropData {
    pub fn new(stages: impl IntoIterator<Item = CropStage>) -> Self {
        Self {
            id: "".into(),
            stages: stages.into_iter().collect::<Vec<_>>(),
            index: 0,
        }
    }
}

#[derive(Component, Debug)]
pub struct CropTimer(pub u32);

#[derive(Component, Debug)]
pub struct CropFruit(pub Vec<ItemDrop>);

impl Default for CropTimer {
    fn default() -> Self {
        Self(2)
    }
}

#[derive(
    Component, Debug, Hash, PartialEq, Default, Clone, serde::Deserialize, serde::Serialize, Reflect,
)]
pub enum CropStatus {
    #[default]
    Growing,
    Fruiting {
        model: String,
        drops: Vec<ItemDrop>,
    },
    Seeding {
        model: String,
        drops: Vec<ItemDrop>,
    },
    Dead,
}
