use std::any::TypeId;

use bevy_ecs::event::Event;
use bevy_spts_uid::Uid;

#[derive(Debug, Clone)]
pub enum ChangedType {
    Inserted,
    Applied,
    Mutated,
    Removed,
}

#[derive(Debug, Event, Clone)]
pub enum ChangesetEvent {
    Spawned(Uid),
    Despawned(Uid),
    Changed(Uid, TypeId, ChangedType),
}
