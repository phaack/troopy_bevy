//! This file contains the shared [`Protocol`] that defines the messages that can be sent between the client and server.
//!
//! You will need to define the [`Components`], [`Messages`] and [`Inputs`] that make up the protocol.
//! You can use the `#[protocol]` attribute to specify additional behaviour:
//! - how entities contained in the message should be mapped from the remote world to the local world
//! - how the component should be synchronized between the `Confirmed` entity and the `Predicted`/`Interpolated` entity
use std::ops::{Add, Mul};

use bevy::ecs::entity::MapEntities;
use bevy::prelude::{
    default, Bundle, Color, Component, Deref, DerefMut, Entity, EntityMapper, Vec2,
};
use bevy::prelude::{App, Plugin};
use serde::{Deserialize, Serialize};

use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;

use super::messages::{SendTroopsMessage, UpgradeStructureMessage, UseTavernMessage};

// Player
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    id: PlayerId,
}

impl PlayerBundle {
    pub(crate) fn new(id: ClientId) -> Self {
        // Generate pseudo random color from client id.
        Self { id: PlayerId(id) }
    }
}

// Components
#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(ClientId);

// Example of a component that contains an entity.
// This component, when replicated, needs to have the inner entity mapped from the Server world
// to the client World.
// You will need to derive the `MapEntities` trait for the component, and register
// app.add_map_entities<PlayerParent>() in your protocol
#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerParent(Entity);

impl MapEntities for PlayerParent {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.0 = entity_mapper.map_entity(self.0);
    }
}

// Channels
//
#[derive(Channel)]
pub struct GameChannel;

// Messages

//
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// pub enum Inputs {
//     SendTroops((PlayerId, u8)),
//     None,
// }

// Protocol
pub(crate) struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // messages
        app.register_message::<SendTroopsMessage>(ChannelDirection::ClientToServer);
        app.register_message::<UseTavernMessage>(ChannelDirection::ClientToServer);
        app.register_message::<UpgradeStructureMessage>(ChannelDirection::ClientToServer);

        // components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        // channels
        app.add_channel::<GameChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
