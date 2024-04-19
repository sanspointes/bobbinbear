use bevy::{core_pipeline::contrast_adaptive_sharpening, prelude::*};

#[derive(Component, Clone, Copy, Debug)]
pub struct SyncedPosition(Entity);

impl SyncedPosition {
    pub fn new(other: Entity) -> Self {
        Self(other)
    }

    pub fn get_other(&self) -> Entity {
        self.0
    }
}

#[allow(clippy::type_complexity)]
pub fn sys_sync_positions(
    mut param_set: ParamSet<(
        // q_changed
        Query<(Entity, &SyncedPosition, &Transform), Changed<Transform>>,
        // q_other
        Query<&mut Transform>,
    )>,
) {
    let moved: Vec<_> = param_set
        .p0()
        .iter()
        .map(|(e, synced_position, transform)| (e, *synced_position, transform.translation))
        .collect();

    let mut q_other = param_set.p1();
    for (e, synced_with, position) in moved {
        let other_entity = synced_with.get_other();
        let Some(mut transform) = q_other.get_mut(other_entity).ok() else {
            warn!("Entity with SyncedPosition({e:?}) references entity without a Transform component.");
            continue;
        };

        transform.translation = position;
    }
}
