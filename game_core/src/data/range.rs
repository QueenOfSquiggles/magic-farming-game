use bevy::prelude::*;
use bevy_rand::prelude::{GlobalEntropy, WyRand};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Creates a range
#[derive(Hash, Reflect, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

impl Default for Range {
    fn default() -> Self {
        Self { min: 0, max: 1 }
    }
}

impl Range {
    /// Generates a random u32 as \[min,max\], such that min, max, and all values in between are valid to return
    pub fn get(&self, rng: &mut ResMut<GlobalEntropy<WyRand>>) -> u32 {
        rng.gen_range(self.min..=self.max)
    }
}
