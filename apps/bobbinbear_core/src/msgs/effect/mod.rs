//! Contains `EffectMsg`.  A very widely shared event that indicates that
//! the state of the app has changed some how.
//!
//! The child modules define handlers for different effects groups.

mod graph;

use bb_vector_network::prelude::BBEdgeIndex;
use bevy::prelude::*;

use crate::components::{
    bbid::{BBId, BBIdUtils},
    scene::{BBNode, BBObject},
};

use super::MsgQue;

pub struct EffectMsgPlugin;
impl Plugin for EffectMsgPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Event, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectMovedEffect {
    pub target: BBId,
    pub world_position: Vec3,
}

#[derive(Event, Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Effects describe side effects that occur during a message.
pub enum EffectMsg {
    //Camera Events
    CameraMoved {
        translation: Vec2,
        delta: Vec2,
    },
    CameraZoomed {
        zoom_level: f32,
        delta: f32,
    },

    // Object Events
    ObjectSelectionChanged {
        /// Objects that were just selected
        selected: Vec<BBId>,
        /// Objects that were just deselected
        deselected: Vec<BBId>,
        /// The currently selected objects in the scene
        currently_selected: Vec<BBId>,
    },
    ObjectAdded {
        bbid: BBId,
    },
    ObjectRemoved {
        bbid: BBId,
    },
    ObjectMoved(ObjectMovedEffect),

    /// All side effects / events related to inspection / uninspection.
    ObjectInspected {
        object_type: BBObject,
        target: BBId,
    },
    ObjectUninspected {
        object_type: BBObject,
        target: BBId,
    },

    GraphStructureChanged {
        bbid: BBId,
    },
    GraphNeedsRemesh {
        bbid: BBId,
    },

    // Edge Events
    EdgeAdded {
        target: BBId,
        idx: BBEdgeIndex,
    },
    EdgeRemoved {
        target: BBId,
        idx: BBEdgeIndex,
    },
}

impl EffectMsg {
    pub fn send(self, world: &mut World) {
        let mut effects = world.resource_mut::<Events<EffectMsg>>();
        effects.send(self);
    }
}

/// Primary handler for `EffectMsg` that implement the business logic app that should result from
/// an effect.
pub(super) fn msg_handler_effect(world: &mut World, effect: &EffectMsg, responder: &mut MsgQue) {
    #[allow(clippy::single_match)]
    match effect {
        EffectMsg::ObjectInspected {
            object_type,
            target,
        } => match object_type {
            BBObject::Vector => graph::handle_graph_inspected(world, *target),
        },
        EffectMsg::EdgeAdded { target, .. } => {
            graph::handle_inspected_graph_changed(world, *target);
        }
        EffectMsg::EdgeRemoved { target, .. } => {
            graph::handle_inspected_graph_changed(world, *target);
        }
        EffectMsg::ObjectUninspected {
            object_type,
            target,
        } => match object_type {
            BBObject::Vector => graph::handle_graph_uninspected(world, *target),
        },
        EffectMsg::ObjectMoved(effect @ ObjectMovedEffect { target, .. }) => {
            println!("Object moved {target}");
            if world.try_bbid_get::<BBNode>(*target).is_some() {
                println!("BBNode moved!");
                graph::handle_bb_node_moved(world, effect, responder);
            }
        }
        _ => (),
    }
}
