use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::{default, Camera2dBundle, Circle, ColorMaterial, Commands, Mesh, Res, ResMut, Transform};
use bevy::render::primitives::Aabb;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use lightyear::client::networking::ClientCommands;
use lightyear::prelude::TickManager;
use crate::game::core::interaction::structure_interaction::StructureInteractionPlugin;
use crate::game::core::state::player::Player;
use crate::game::core::structures::structure::{Structure, StructureType};
use crate::game::networking::client::build_client_plugin;
use crate::game::networking::shared::shared::SharedPlugin;

pub struct GameClientPlugin;
impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        // add lightyear plugins
        app.add_plugins(build_client_plugin());
        // add our shared plugin containing the protocol + other shared behaviour
        app.add_plugins(SharedPlugin);
        
        

        // add our client-specific logic. Here we will just connect to the server
        app.add_systems(Startup, (connect_client, setup));
        app.add_plugins(StructureInteractionPlugin);
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
