use std::any::TypeId;

use bevy_ecs::event::Event;
use bevy_spts_uid::Uid;

#[derive(Event, Clone)]
pub enum ChangesetEvent {
    Spawned(Uid),
    Despawned(Uid),
    Changed(Uid, TypeId),
}
