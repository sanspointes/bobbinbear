//! # Position
//! 
//! Module contains logic for a smart `Position` component that represents the position of an
//! object via a number of different methods.

use std::ops::Deref;

use bevy::{ecs::reflect::ReflectComponent, prelude::*};
use bevy_spts_uid::{Uid, UidRegistry};

use crate::plugins::viewport::BobbinViewport;

use super::ProxiedComponent;

#[derive(Component, Clone, Copy, Reflect, Default, PartialEq, Deref, Debug)]
#[reflect(Component)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(pos: impl Into<Vec2>) -> Self {
        Self(pos.into())
    }
}

#[derive(Default, Reflect, Debug)]
#[reflect(Debug, Default)]
/// Declares how we want to proxy the target objects position
pub enum ProxiedPositionStrategy {
    #[default]
    Local,
    /// Copies the target objects position in viewport space, useful for UI elements.
    /// Proxy object must be a child of Camera/Viewport.
    Viewport { target_world_position: Vec3 },
}
pub type ProxiedPosition = ProxiedComponent<Position, ProxiedPositionStrategy>;


#[allow(clippy::single_match)]
pub fn sys_update_proxied_component_position_state(
    q_global_transform: Query<&GlobalTransform, Without<Camera>>,
    mut q_proxied: Query<(&mut Position, &mut ProxiedPosition)>,
    q_proxy_source: Query<&Position, Without<ProxiedPosition>>,
    uid_registry: Res<UidRegistry>,
) {
    for (mut proxy_value, mut proxy) in q_proxied.iter_mut() {
        let target_uid = *proxy.target();
        let target_entity = uid_registry.entity(target_uid);
        let strategy = proxy.state_mut();

        let target_value = q_proxy_source.get(target_entity).unwrap();
        if *target_value != *proxy_value.deref() {
            *proxy_value = *target_value;
        }
        match strategy {
            ProxiedPositionStrategy::Viewport { target_world_position: ref mut target_world_translation } => {
                let Ok(global_transform) = q_global_transform.get(target_entity) else {
                    warn!("sys_pre_update_positions: Can't get target for ProxiedPosition {target_uid}.");
                    continue;
                };
                let curr_world_translation = global_transform.translation();
                if curr_world_translation != *target_world_translation {
                    *target_world_translation = curr_world_translation;
                }
                // if let Some(pos) = camera.world_to_ndc(camera_global_transform, *target_world_position) {
                //     let pos = viewport.ndc_to_viewport(pos.xy());
                //     transform.translation.x = pos.x;
                //     transform.translation.y = pos.y;
                // }
            }
            _ => {},
        }
    }
}

pub fn sys_update_positions(
    q_camera: Query<(&Camera, &BobbinViewport, &GlobalTransform), With<Camera>>,
    mut q_positioned: Query<(&Position, Option<&ProxiedPosition>, &mut Transform), Without<Camera>>,
    // mut q_positioned: Query<(&Position, &ProxiedPosition, &mut Transform), (Without<Camera>, Or<(Changed<Position>, Changed<ProxiedPosition>)>)>,
) {
    let (camera, viewport, camera_global_transform) = q_camera.single();

    for (pos, proxy, mut transform) in q_positioned.iter_mut() {
        let maybe_strategy = proxy.map(|p| p.state());

        match maybe_strategy {
            None | Some(ProxiedPositionStrategy::Local) => {
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
            Some(ProxiedPositionStrategy::Viewport { target_world_position }) => {
                if let Some(pos) = camera.world_to_ndc(camera_global_transform, *target_world_position) {
                    let pos = viewport.ndc_to_viewport(pos.xy());
                    transform.translation.x = pos.x;
                    transform.translation.y = pos.y;
                }
            }
        }
    }
}
