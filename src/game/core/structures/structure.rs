use bevy::prelude::*;

use crate::game::core::state::player::Player;

pub type ResourceType = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructureType {
    BaseTower,
    RegularTower,
    Tavern,
    GoldMine,
}

pub struct StructureLevelData {
    pub max_resources: ResourceType,
    pub resource_generatrion_rate: ResourceType,
}

#[derive(Component)]
pub struct Structure {
    pub owner: Player,
    pub structure_type: StructureType,
    pub resources: ResourceType,
    pub level_data: Vec<StructureLevelData>,
    pub level: u8,
}

impl Structure {
    pub fn new(
        owner: Player,
        structure_type: StructureType,
        level_data: Vec<StructureLevelData>,
    ) -> Self {
        Self {
            owner,
            structure_type,
            resources: 0,
            level_data,
            level: 0,
        }
    }
    pub fn default(owner: Player, structure_type: StructureType) -> Self {
        Self {
            owner,
            structure_type,
            resources: 0,
            level_data: vec![StructureLevelData {
                max_resources: 100,
                resource_generatrion_rate: 1,
            }],
            level: 0,
        }
    }
    pub fn upgrade(&mut self) {
        if self.level < self.level_data.len() as u8 {
            self.level += 1;
        }
    }
    pub fn generate_resources(&mut self) {
        if self.level < self.level_data.len() as u8 {
            self.resources += self.level_data[self.level as usize].resource_generatrion_rate;
        }
    }
}
