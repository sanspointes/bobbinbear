use std::sync::Arc;

use bevy::ecs::world::World;

use bevy_spts_changeset::prelude::{Change, ChangesetContext};
use bevy_spts_uid::Uid;

#[derive(Debug)]
pub struct LinkEdgeChange {
    edge: Uid,
    next_endpoint: Uid,
    prev_endpoint: Uid,
}

impl Change for LinkEdgeChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<(dyn Change + 'static)>, anyhow::Error> {
        let mut ss = SystemState::<Index<Uid>>::new(world);
        let mut index = ss.get_mut(world);

        let edge_e = index.single(self.edge);
        let next_endpoint_e = index.single(self.next_endpoint);
        let prev_endpoint_e = index.single(self.prev_endpoint);

        if let Some(mut endpoint) = world.get_mut::<Endpoint>(next_endpoint_e) {
            endpoint.prev_edge = Some(edge);
        } else {
            warn!("LinkEdgeCommand: Attempted to get Endpoint component on entity {next_endpoint:?}, but none found.");
            return;
        };
        if let Some(mut endpoint) = world.get_mut::<Endpoint>(prev_endpoint_e) {
            endpoint.next_edge = Some(edge);
        } else {
            warn!("LinkEdgeCommand: Attempted to get Endpoint component on entity {prev_endpoint:?}, but none found.");
        };
        
    }
}
