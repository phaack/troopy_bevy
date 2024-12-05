use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
pub enum TroopType {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct SendTroopsAction {
    pub from_structure: Entity,
    pub to_structure: Entity,
    pub troop_type: TroopType,
    pub count: u8,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct UpgradeStructureAction {
    pub structure: Entity,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct UseTavernAction {
    pub troop_type: TroopType,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TroopyActions {
    SendTroops(SendTroopsAction),
    UpgradeStructure(UpgradeStructureAction),
    UseTavern(UseTavernAction),
}
