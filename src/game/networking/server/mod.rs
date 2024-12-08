//! The server side of the example.
//! It is possible (and recommended) to run the server in headless mode (without any rendering plugins).
//!
//! The server will:
//! - spawn a new player entity for each client that connects
//! - read inputs from the clients and move the player entities accordingly
//!
//! Lightyear will handle the replication of entities automatically if you add a `Replicate` component to them.

pub mod game_server;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::shared::events::components::MessageEvent;
use lightyear::shared::log::add_log_layer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::game::core::structures::structure::Structure;

use crate::game::networking::shared::messages::SendTroopsMessage;
use crate::game::networking::shared::shared::{shared_config, SharedPlugin, SERVER_REPLICATION_INTERVAL};

pub(crate) const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

/// Here we create the lightyear [`ServerPlugins`]
fn build_server_plugin() -> ServerPlugins {
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };
    let config = ServerConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        // we can specify multiple net configs here, and the server will listen on all of them
        // at the same time. Here we will only use one
        net: vec![net_config],
        replication: ReplicationConfig {
            // we will send updates to the clients every 100ms
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    ServerPlugins::new(config)
}

