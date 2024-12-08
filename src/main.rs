//! This minimal example showcases how to setup the lightyear plugins.
//!
//! Run with
//! - `cargo run -- server`
//! - `cargo run -- client`
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod game;

use game::networking::*;
use bevy::prelude::*;
use clap::Parser;
use crate::game::networking::client::game_client::GameClientPlugin;
use crate::game::networking::server::game_server::GameServerPlugin;

#[derive(Parser, PartialEq, Debug)]
pub enum Cli {
    /// The program will act as server
    Server,
    /// The program will act as a client
    Client,
}

fn main() {
    let cli = Cli::parse();
    let mut app = App::new();
    match cli {
        Cli::Server => app.add_plugins(GameServerPlugin),
        Cli::Client => app.add_plugins(GameClientPlugin),
    };
    app.run();
}
