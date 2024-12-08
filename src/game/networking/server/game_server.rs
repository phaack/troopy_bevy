use bevy::app::{App, Plugin, Startup, Update};
use bevy::log::{Level, LogPlugin};
use bevy::MinimalPlugins;
use bevy::prelude::{default, Commands, EventReader};
use bevy::state::app::StatesPlugin;
use lightyear::prelude::server::ServerCommands;
use lightyear::shared::events::components::MessageEvent;
use crate::game::networking::server::{build_server_plugin};
use crate::game::networking::shared::messages::SendTroopsMessage;
use crate::game::networking::shared::shared::SharedPlugin;

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MinimalPlugins, StatesPlugin));
        // add lightyear plugins
        app.add_plugins(build_server_plugin());
        // add our shared plugin containing the protocol + other shared behaviour
        app.add_plugins(SharedPlugin); 
        
        
        
        app.add_plugins(LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
            ..default()
        });
        // add our server-specific logic. Here we will just start listening for incoming connections
        app.add_systems(Startup, start_server);
        // app.add_systems(Update, receive_send_troops_message);
    }
}

/// Start the server
fn start_server(mut commands: Commands) {
    commands.start_server();
}

// fn receive_send_troops_message(mut events: EventReader<MessageEvent<SendTroopsMessage>>) {
//     for event in events.read() {
//         println!("Received SendTroopsMessage: {:?}", event.message);
//     }
// }
