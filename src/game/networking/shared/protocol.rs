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

use crate::game::core::structures::structure::Structure;

use crate::game::networking::shared::messages::{SendTroopsMessage, UpgradeStructureMessage, UseTavernMessage};
// Channels
#[derive(Channel)]
pub struct GameChannel;

// Protocol
pub(crate) struct GameProtocolPlugin;

impl Plugin for GameProtocolPlugin {
    fn build(&self, app: &mut App) {
        // messages
        app.register_message::<SendTroopsMessage>(ChannelDirection::Bidirectional);
        app.register_message::<UseTavernMessage>(ChannelDirection::ClientToServer);
        app.register_message::<UpgradeStructureMessage>(ChannelDirection::ClientToServer);

        // components
        app.register_component::<Structure>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        // channels
        app.add_channel::<GameChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}
