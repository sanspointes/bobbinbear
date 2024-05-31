//! # Position
//!
//! Module contains logic for a smart `Position` component that represents the position of an
//! object via a number of different methods.

use bevy::{
    ecs::{query::QueryEntityError, reflect::ReflectComponent},
    prelude::*,
};
use bevy_spts_uid::UidRegistry;
use bevy_spts_vectorgraphic::lyon_path::math::Point;

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

impl From<Position> for Point {
    fn from(value: Position) -> Self {
        Self::new(value.x, value.y)
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
    Viewport,
}
pub type ProxiedPosition = ProxiedComponent<Position, ProxiedPositionStrategy>;

pub fn sys_sort_sync_position_proxy_and_transform(
    uid_registry: Res<UidRegistry>,
    q_position: Query<(Entity, Option<&ProxiedPosition>), With<Position>>,
) -> Vec<Entity> {
    let mut values: Vec<_> = q_position
        .iter()
        .map(|(e, proxy)| {
            let mut depth = 0;
            let mut proxy_target = proxy.map(|p| p.target());
            while let Some(target) = proxy_target {
                let Ok(e) = uid_registry.get_entity(*target) else {
                    break;
                };
                depth += 1;
                proxy_target = q_position
                    .get(e)
                    .ok()
                    .and_then(|(_, t)| t)
                    .map(|t| t.target());
            }
            (e, depth)
        })
        .collect();

    // Sort by lowest (proxy source) first.
    values.sort_by(|(_, a), (_, b)| a.cmp(b));

    values.iter().map(|v| v.0).collect()
}

#[allow(clippy::too_many_arguments)]
fn sync_position_proxy_and_transform(
    to_update: Vec<Entity>,
    uid_registry: Res<UidRegistry>,
    q_camera: Query<(Entity, &Camera, &BobbinViewport)>,
    q_proxied_position: Query<(Option<&ProxiedPosition>, Option<&Parent>)>,
    mut q_position: Query<&mut Position>,
    mut q_transform: Query<&mut Transform>,
    mut q_global_transform: Query<&mut GlobalTransform>,
) -> Result<(), anyhow::Error> {
    let (e_camera, camera, viewport) = q_camera.single();
    let camera_global_transform = *q_global_transform.get(e_camera).unwrap();

    // warn!("---- sys_sync_position_proxy_and_transform ----");
    // warn!("{to_update:?}");
    for e in to_update {
        let (pos_proxy, parent) = q_proxied_position.get(e)?;
        // let name = q_name.get(e).unwrap();
        // warn!("**** {e:?} {name:?} Syncing proxy and transform on {e:?} (with parent {parent:?})");

        if let Some(pos_proxy) = pos_proxy {
            // Copy position across
            let source_e = uid_registry.get_entity(*pos_proxy.target())?;
            let [source_pos, mut self_pos] = q_position.get_many_mut([source_e, e])?;
            // warn!("- Updating position from proxy source ({self_pos:?} -> {source_pos:?})");
            *self_pos = *source_pos;

            match pos_proxy.state() {
                ProxiedPositionStrategy::Local => {
                    // Update transform
                    let mut transform = q_transform.get_mut(e)?;
                    transform.translation.x = source_pos.x;
                    transform.translation.y = source_pos.y;
                    // warn!("- Local Strategy: Copying to Transform.");
                }
                ProxiedPositionStrategy::Viewport => {
                    let e_target = uid_registry.get_entity(*pos_proxy.target())?;
                    let target_global_transform = q_global_transform.get(e_target)?;
                    if let Some(pos) = camera.world_to_ndc(
                        &camera_global_transform,
                        target_global_transform.translation(),
                    ) {
                        let viewport_pos = viewport.ndc_to_viewport(pos.xy());
                        // warn!("- Viewport Strategy: Transformed to {viewport_pos:?}.");
                        let mut transform = q_transform.get_mut(e)?;
                        transform.translation.x = viewport_pos.x;
                        transform.translation.y = viewport_pos.y;
                    }
                }
            }
        } else {
            let mut transform = q_transform.get_mut(e)?;
            let pos = q_position.get(e)?;
            // warn!("- Syncing Position {pos:?} into transform {:?}", transform.translation);
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }

        // Propagate change to position back to Transform
        if let Some(parent) = parent {
            let c_transform = q_transform.get(e)?;
            let [p_global, mut c_global] = q_global_transform.get_many_mut([parent.get(), e])?;

            let next_transform = p_global.mul_transform(*c_transform);
            // warn!("- Regenerating global transform {:?} -> {:?} ", c_global.translation(), next_transform.translation());
            *c_global = next_transform;
        } else {
            let c_transform = q_transform.get(e)?;
            let mut c_global = q_global_transform.get_mut(e)?;
            *c_global = (*c_transform).into();
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn sys_sync_position_proxy_and_transform(
    In(to_update): In<Vec<Entity>>,
    uid_registry: Res<UidRegistry>,
    q_camera: Query<(Entity, &Camera, &BobbinViewport)>,
    q_proxied_position: Query<(Option<&ProxiedPosition>, Option<&Parent>)>,
    q_position: Query<&mut Position>,
    q_transform: Query<&mut Transform>,
    q_global_transform: Query<&mut GlobalTransform>,
) {
    if let Err(reason) = sync_position_proxy_and_transform(
        to_update,
        uid_registry,
        q_camera,
        q_proxied_position,
        q_position,
        q_transform,
        q_global_transform,
    ) {
        error!("sync_position_proxy_and_transform: Error {reason:?}");
    }
}
