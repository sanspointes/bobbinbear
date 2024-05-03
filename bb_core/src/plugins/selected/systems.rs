use std::ops::{Deref, DerefMut};

use bevy::{ecs::prelude::*, log::info};
use bevy_spts_uid::UidRegistry;

use super::Selected;

/// Propagates selected state into Selected::Proxy's `selected` field.
pub(super) fn sys_update_selected_proxies(
    mut q_selected: Query<(Entity, &mut Selected)>,
    uid_registry: Res<UidRegistry>,
) {
    let to_update: Vec<_> = q_selected
        .iter_mut()
        .filter_map(|(entity, selected)| match selected.deref() {
            Selected::Proxy { target, .. } => Some((entity, *target)),
            _ => None,
        })
        .collect();


    for (proxy_e, source_uid) in to_update {
        let is_source_selected = {
            let source_e = uid_registry.entity(source_uid);
            let (_, selected) = q_selected.get(source_e).unwrap();
            selected.is_selected()
        };

        let (_, mut source_selected) = q_selected.get_mut(proxy_e).unwrap();
        match source_selected.deref_mut() {
            Selected::Proxy {
                ref mut selected, ..
            } => {
                *selected = is_source_selected;
            }
            // SAFETY: Filtered by Selected::Proxy above.
            _ => panic!("Impossible."),
        }
    }
}
