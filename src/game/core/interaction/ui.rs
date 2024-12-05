use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::game::core::interaction::actions::SendTroopsAction;
use crate::game::core::state::player::Player;
use crate::game::core::structures::structure::Structure;
use crate::messages::{SendTroopsMessage, UpgradeStructureMessage};

/// Resource to keep track of the drag state
#[derive(Default, Resource)]
struct DragState {
    is_dragging: bool,
    start_entity: Option<Entity>,
    current_hover_entity: Option<Entity>,
    path: Vec<Entity>,
}

pub struct StructureInteractionPlugin;

impl Plugin for StructureInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragState>();
        app.add_systems(
            Update,
            (
                mouse_interaction_start,
                mouse_interaction_move,
                mouse_interaction_end,
            ),
        );
    }
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .init_resource::<DragState>()
//         .add_systems(Update, mouse_button_input)
//         .add_systems(Update, mouse_move_system)
//         .add_systems(Update, mouse_button_up_system)
//         .run();
// }

/// Function to check if a point is inside an entity's AABB
fn point_in_aabb(point: Vec3, transform: &Transform, aabb: &Aabb) -> bool {
    // Transform the point into the entity's local space
    let local_point = transform.compute_matrix().inverse().transform_point3(point);

    local_point.x >= aabb.min().x
        && local_point.x <= aabb.max().x
        && local_point.y >= aabb.min().y
        && local_point.y <= aabb.max().y
    // && local_point.z >= aabb.min().z
    // && local_point.z <= aabb.max().z
}

/// System to handle mouse button down events
fn mouse_interaction_start(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    structures: Query<(Entity, &Transform, &Aabb), With<Structure>>,
    mut drag_state: ResMut<DragState>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        // Get the cursor position in world coordinates
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let (camera, camera_transform) = camera_query.single();

                if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    let world_pos = ray.origin;

                    // Iterate over structures to see if any contains the cursor position
                    for (entity, transform, aabb) in structures.iter() {
                        // Check if world_pos is inside the AABB of the structure
                        if point_in_aabb(world_pos, transform, aabb) {
                            // Start drag operation
                            drag_state.is_dragging = true;
                            drag_state.start_entity = Some(entity);
                            drag_state.current_hover_entity = Some(entity);
                            drag_state.path.push(entity);
                            break; // Assuming only one entity can be under the cursor
                        }
                    }
                }
            }
        }
    }
}

/// System to handle mouse movement and hovering over structures during dragging
fn mouse_interaction_move(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    structures: Query<(Entity, &Transform, &Aabb), With<Structure>>,
    mut drag_state: ResMut<DragState>,
) {
    if drag_state.is_dragging {
        // Get the cursor position in world coordinates
        if let Ok(window) = windows.get_single() {
            if let Some(cursor_pos) = window.cursor_position() {
                let (camera, camera_transform) = camera_query.single();

                if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    let world_pos = ray.origin;

                    // Iterate over structures to see if any contains the cursor position
                    for (entity, transform, aabb) in structures.iter() {
                        if point_in_aabb(world_pos, transform, aabb) {
                            // If the entity is different from current_hover_entity and if it's not
                            // already in the path, add it to the path
                            if Some(entity) != drag_state.current_hover_entity
                                && !drag_state.path.contains(&entity)
                            {
                                drag_state.current_hover_entity = Some(entity);
                                drag_state.path.push(entity);
                            }
                            break;
                        } else {
                            drag_state.current_hover_entity = None;
                        }
                    }
                }
            }
        }
    }
}

/// System to handle mouse button up events and finalize the drag operation
fn mouse_interaction_end(
    buttons: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DragState>,
    mut send_troops_event: EventWriter<SendTroopsMessage>,
    mut upgrade_structure_event: EventWriter<UpgradeStructureMessage>,
) {
    if buttons.just_released(MouseButton::Left) && drag_state.is_dragging {
        drag_state.is_dragging = false;
        // Handle drag and drop logic here
        if let Some(start_entity) = drag_state.start_entity {
            let end_entity = drag_state.current_hover_entity;
            // React to drag from start_entity to end_entity
            if let Some(end_entity) = end_entity {
                info!("Dragged from {:?} to {:?}", start_entity, end_entity);
                // send troops event
                send_troops_event.send(SendTroopsMessage {
                    player: Player::new(Some(0)),
                    action: SendTroopsAction::default(&start_entity, &end_entity),
                });
            } else {
                info!("Dragged from {:?} to empty space", start_entity);
                // upgrade structure event
            }
            // with all entities in the path
            info!(
                "Entities in the path: {:?}",
                drag_state
                    .path
                    .iter()
                    .map(|entity| entity.to_string())
                    .collect::<Vec<String>>(),
            );
        }
        // Reset drag_state
        drag_state.start_entity = None;
        drag_state.current_hover_entity = None;
        drag_state.path.clear();
    }
}
