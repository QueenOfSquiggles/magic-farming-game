use super::ItemId;
use crate::data::range::Range;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Hash, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemDrop {
    pub item: ItemId,
    pub amount: Range,
}
