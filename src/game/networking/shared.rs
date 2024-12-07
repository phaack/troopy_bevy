//! This module contains the shared code between the client and the server.
use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::utils::Duration;

use lightyear::shared::config::Mode;
use lightyear::{prelude::*, shared::events::components::MessageEvent};

use super::messages::{SendTroopsMessage, UpgradeStructureMessage, UseTavernMessage};
use super::protocol::GameChannel;

pub const FIXED_TIMESTEP_HZ: f64 = 64.0;

pub const SERVER_REPLICATION_INTERVAL: Duration = Duration::from_millis(100);

/// The [`SharedConfig`] must be shared between the `ClientConfig` and `ServerConfig`
pub fn shared_config() -> SharedConfig {
    SharedConfig {
        // send an update every 100ms
        server_replication_send_interval: SERVER_REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ),
        },
        mode: Mode::Separate,
    }
}

#[derive(Clone)]
pub struct SharedPlugin;

impl Plugin for SharedPlugin {
    fn build(&self, app: &mut App) {
        // Register your protocol, which is shared between client and server
        app.add_channel::<GameChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        app.add_event::<MessageEvent<SendTroopsMessage>>();
        app.add_event::<MessageEvent<UseTavernMessage>>();
        app.add_event::<MessageEvent<UpgradeStructureMessage>>();

        if app.is_plugin_added::<RenderPlugin>() {
            app.add_systems(Startup, init);
        }
    }
}

fn init(mut commands: Commands) {}
