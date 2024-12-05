//! The client plugin.
use crate::game::core::interaction::ui::StructureInteractionPlugin;
use crate::game::core::state::player::Player;
use crate::game::core::structures::structure::{Structure, StructureType};
use crate::server::SERVER_ADDR;
use crate::shared;
use crate::shared::{shared_config, SharedPlugin};
use bevy::math::sampling::shape_sampling;
use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::render::primitives::Aabb;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
pub use lightyear::prelude::client::*;
use lightyear::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

pub struct ExampleClientPlugin;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);

/// Here we create the lightyear [`ClientPlugins`]
fn build_client_plugin() -> ClientPlugins {
    // Authentication is where you specify how the client should connect to the server
    // This is where you provide the server address.
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the client_address, because we open a UDP socket on the client
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth,
        io,
        config: NetcodeConfig::default(),
    };
    let config = ClientConfig {
        // part of the config needs to be shared between the client and server
        shared: shared_config(),
        net: net_config,
        ..default()
    };
    ClientPlugins::new(config)
}

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins(StructureInteractionPlugin);
        // add lightyear plugins
        app.add_plugins(build_client_plugin());
        // add our shared plugin containing the protocol + other shared behaviour
        app.add_plugins(SharedPlugin);

        // add our client-specific logic. Here we will just connect to the server
        app.add_systems(Startup, (connect_client, setup));
    }
}

/// Connect to the server
fn connect_client(mut commands: Commands) {
    commands.connect_client();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tick_manager: Res<TickManager>,
) {
    let shape = Mesh2dHandle(meshes.add(Circle::new(50.0)));
    let color = Color::rgb(0.8, 0.7, 0.6);

    // Spawn Camera with GlobalTransform
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Structure::default(Player::default(), StructureType::BaseTower),
        Aabb::from_min_max(Vec3::new(-50.0, -50.0, 0.0), Vec3::new(50.0, 50.0, 0.0)),
        // Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(100.0, 100.0, 0.0),
            ..default()
        },
        Structure::default(Player::default(), StructureType::BaseTower),
        Aabb::from_min_max(Vec3::new(-50.0, -50.0, 0.0), Vec3::new(50.0, 50.0, 0.0)),
        // Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(-100.0, -100.0, 0.0),
            ..default()
        },
        Structure::default(Player::default(), StructureType::BaseTower),
        Aabb::from_min_max(Vec3::new(-50.0, -50.0, 0.0), Vec3::new(50.0, 50.0, 0.0)),
        // Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
