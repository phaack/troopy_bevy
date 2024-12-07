// SendTroopsMessage

use bevy::prelude::Event;
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::core::{
    interaction::actions::{SendTroopsAction, UpgradeStructureAction, UseTavernAction},
    state::player::Player,
};

use super::protocol::PlayerId;

#[derive(Serialize, Deserialize,Debug, PartialEq, Clone, Event)]
pub struct SendTroopsMessage {
    pub player: Player,
    pub action: SendTroopsAction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Event)]
pub struct UseTavernMessage {
    pub player: Player,
    pub action: UseTavernAction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Event)]
pub struct UpgradeStructureMessage {
    pub player: Player,
    pub action: UpgradeStructureAction,
}

// #[message_protocol(protocol = "ActionMessageProtocol")]
// pub enum Messages {
//     SendTroops(SendTroopsMessage),
//     UseTavern(UseTavernMessage),
//     UpgradeStructure(UpgradeStructureMessage),
// }
