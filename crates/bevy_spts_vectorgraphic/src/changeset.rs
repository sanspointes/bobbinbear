use std::{any::Any, sync::Arc};

use anyhow::anyhow;
use bevy::prelude::*;

use bevy_spts_changeset::{as_any::AsAny, prelude::*};
use bevy_spts_uid::{Uid, UidRegistry};

use crate::prelude::*;

#[derive(Debug)]
pub struct LinkEdgeChange {
    pub edge: Uid,
    pub next_endpoint: Uid,
    pub prev_endpoint: Uid,
}

impl Change for LinkEdgeChange {
    fn apply(
        &self,
        world: &mut World,
        _cx: &mut ChangesetContext,
    ) -> Result<Arc<(dyn Change + 'static)>, anyhow::Error> {
        let reg = world.resource_mut::<UidRegistry>();

        let next_endpoint_e = reg.get_entity(self.next_endpoint)?;
        let prev_endpoint_e = reg.get_entity(self.prev_endpoint)?;

        if let Some(mut endpoint) = world.get_mut::<Endpoint>(next_endpoint_e) {
            endpoint.prev_edge = Some(self.edge);
        } else {
            return Err(anyhow!("LinkEdgeCommand: Attempted to get Endpoint component on entity {:?}, but none found.", self.next_endpoint));
        };
        if let Some(mut endpoint) = world.get_mut::<Endpoint>(prev_endpoint_e) {
            endpoint.next_edge = Some(self.edge);
        } else {
            return Err(anyhow!("LinkEdgeCommand: Attempted to get Endpoint component on entity {:?}, but none found.", self.prev_endpoint));
        };

        Ok(Arc::new(UnlinkEdgeChange {
            edge: self.edge,
        }))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), NotRepeatableReason> {
        if self.type_id() != other.type_id() {
            return Err(NotRepeatableReason::DifferentType(
                self.type_name(),
                other.type_name(),
            ));
        }
        Err(NotRepeatableReason::ChangesWorldLayout)
    }
}

#[derive(Debug)]
pub struct UnlinkEdgeChange {
    pub edge: Uid,
}

impl Change for UnlinkEdgeChange {
    fn apply(
        &self,
        world: &mut World,
        _cx: &mut ChangesetContext,
    ) -> Result<Arc<(dyn Change + 'static)>, anyhow::Error> {
        let reg = world.resource_mut::<UidRegistry>();

        let edge_e = reg.get_entity(self.edge)?;
        let edge = *world.get::<Edge>(edge_e).unwrap();

        let reg = world.resource_mut::<UidRegistry>();
        let next_endpoint_e = reg.get_entity(edge.next_endpoint)?;
        let prev_endpoint_e = reg.get_entity(edge.prev_endpoint)?;

        if let Some(mut endpoint) = world.get_mut::<Endpoint>(next_endpoint_e) {
            endpoint.prev_edge = None;
        } else {
            return Err(anyhow!("LinkEdgeCommand: Attempted to get Endpoint component on entity {:?}, but none found.", edge.next_endpoint));
        };
        if let Some(mut endpoint) = world.get_mut::<Endpoint>(prev_endpoint_e) {
            endpoint.next_edge = None;
        } else {
            return Err(anyhow!("LinkEdgeCommand: Attempted to get Endpoint component on entity {:?}, but none found.", edge.prev_endpoint));
        };

        Ok(Arc::new(LinkEdgeChange {
            edge: self.edge,
            next_endpoint: edge.next_endpoint,
            prev_endpoint: edge.prev_endpoint,
        }))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), NotRepeatableReason> {
        if self.type_id() != other.type_id() {
            return Err(NotRepeatableReason::DifferentType(
                self.type_name(),
                other.type_name(),
            ));
        }
        Err(NotRepeatableReason::ChangesWorldLayout)
    }
}

pub trait VectorGraphicChangesetExt<'w> {
    fn spawn_edge<'a>(
        &'a mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Uid,
        next_endpoint: Uid,
    ) -> EntityChangeset<'w, 'a>;

    fn despawn_edge(&mut self, edge_uid: Uid);
}

impl<'w> VectorGraphicChangesetExt<'w> for ChangesetCommands<'w> {
    fn spawn_edge<'a>(
        &'a mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Uid,
        next_endpoint: Uid,
    ) -> EntityChangeset<'w, 'a> {
        let mut edge = self.spawn_empty();
        edge.insert(Edge {
            next_endpoint,
            prev_endpoint,
        })
        .insert(edge_variant);
        let uid = edge.uid();

        self.add(Arc::new(LinkEdgeChange {
            edge: uid,
            next_endpoint,
            prev_endpoint,
        }));

        self.entity(uid)
    }

    fn despawn_edge(&mut self, edge_uid: Uid) {
        self.add(Arc::new(UnlinkEdgeChange {
            edge: edge_uid,
        }));
        self.despawn(edge_uid);
    }
}
