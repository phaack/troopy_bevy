use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::core::troops::troop::{Troop, TroopType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
pub struct SendTroopsAction {
    pub from_structure: Entity,
    pub to_structure: Entity,
    pub troop_type: TroopType,
    pub count: u8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
pub struct UpgradeStructureAction {
    pub structure: Entity,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
pub struct UseTavernAction {
    pub troop_type: TroopType,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TroopyActions {
    SendTroops(SendTroopsAction),
    UpgradeStructure(UpgradeStructureAction),
    UseTavern(UseTavernAction),
}

impl SendTroopsAction {
    pub fn default(from_structure: &Entity, to_structure: &Entity) -> Self {
        SendTroopsAction {
            from_structure: from_structure.clone(),
            to_structure: to_structure.clone(),
            troop_type: TroopType::default(),
            count: 1,
        }
    }
}
