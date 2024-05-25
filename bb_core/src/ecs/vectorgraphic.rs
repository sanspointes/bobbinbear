use bevy::{
    ecs::{
        change_detection::DetectChangesMut, query::{Changed, Without}, system::{Query, Res}
    },
    log::warn,
    math::Vec3Swizzles,
    transform::components::Transform,
};
use bevy_spts_uid::UidRegistry;
use bevy_spts_vectorgraphic::components::{Edge, EdgeVariant, Endpoint};

use super::Position;

/// Uses the difference between Position and Transform to find the distance moved
/// and updates the endpoints to move that distance.
///
/// Transform later gets updated to Position near end of frame.
#[allow(clippy::type_complexity)]
pub fn sys_update_endpoint_positions_on_edge_move(
    r_uid_registry: Res<UidRegistry>,
    mut q_endpoints: Query<(&Endpoint, &mut Position), Without<Edge>>,
    q_edges: Query<(&Edge, &Position, &Transform), (Changed<Position>, Without<Endpoint>)>,
) {
    for (edge, pos, transform) in q_edges.iter() {
        let diff = pos.0 - transform.translation.xy();
        warn!("Moving edge by diff {diff}");
        if let Ok((_, mut position)) = q_endpoints.get_mut(r_uid_registry.entity(edge.next_endpoint_uid())) {
            position.0 += diff;
        }
        if let Ok((_, mut position)) = q_endpoints.get_mut(r_uid_registry.entity(edge.prev_endpoint_uid())) {
            position.0 += diff;
        }
    }
}

pub fn sys_cleanup_edge_positions_to_bounding_box(
    r_uid_registry: Res<UidRegistry>,
    q_endpoints: Query<(&Endpoint, &Position), Without<Edge>>,
    mut q_edges: Query<(&Edge, &EdgeVariant, &mut Position), Without<Endpoint>>,
) {
    for (edge, edge_variant, mut position) in q_edges.iter_mut() {
        let Ok((_, prev_position)) =
            q_endpoints.get(r_uid_registry.entity(edge.prev_endpoint_uid()))
        else {
            warn!("Could not get prev_endpoint position.");
            continue;
        };
        let Ok((_, next_position)) =
            q_endpoints.get(r_uid_registry.entity(edge.next_endpoint_uid()))
        else {
            warn!("Could not get next_endpoint position.");
            continue;
        };

        let mut min = prev_position.0.min(next_position.0);

        match edge_variant {
            EdgeVariant::Line => (),
            EdgeVariant::Quadratic { ctrl1 } => {
                min = min.min(*ctrl1);
            }
            EdgeVariant::Cubic { ctrl1, ctrl2 } => {
                min = min.min(*ctrl1);
                min = min.min(*ctrl2);
            }
        }

        // Must bypass change detection to avoid infinite loop
        position.bypass_change_detection().0 = min;
    }
}
