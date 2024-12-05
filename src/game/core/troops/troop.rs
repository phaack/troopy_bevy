use std::default;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Default)]
pub enum TroopType {
    #[default]
    Knight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TroopState {
    #[default]
    JustSpawned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TroopLevelData {
    pub max_health: u16,
}

// Each troop will only contain general information that all troops share
// TroopBehavior like Movement etc. will be handled by different systems

#[derive(Component)]
pub struct Troop {
    pub troop_type: TroopType,
    pub troop_state: TroopState,
    pub health: u16,
    pub current_level: u8,
    pub level_data: Vec<TroopLevelData>,
}

impl Troop {
    // temporary: provide default
    pub fn default(troop_type: TroopType) -> Self {
        Troop {
            troop_type,
            troop_state: TroopState::default(),
            health: 1,
            current_level: 0,
            level_data: vec![TroopLevelData { max_health: 10 }],
        }
    }
}
