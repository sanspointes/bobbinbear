//! Contains logic for effects (side effect events) to keep the JS state synced with the bevy
//! state.

use std::collections::VecDeque;

use bevy::{prelude::*, reflect::Typed};

mod js_event_que;

use bevy_spts_changeset::events::{ChangedType, ChangesetEvent};
use bevy_spts_uid::Uid;
pub use effects::*;
pub use js_event_que::EffectQue;

use crate::{ecs::ProxiedComponent, plugins::{inspecting::Inspected, selected::Selected}};

use super::inspecting::handle_inspection_changed;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Effect>();
        app.insert_resource(EffectQue::new());
        app.add_systems(
            PostUpdate,
            sys_collect_changeset_events.before(sys_emit_effects),
        );
        app.add_systems(Last, sys_emit_effects);
    }
}

#[allow(clippy::type_complexity)]
/// Collects all the changeset events into effect events.  Sends them via EventWriter
pub fn sys_collect_changeset_events(
    mut ev_spawned: EventReader<ChangesetEvent>,
    mut ev_effect_writer: EventWriter<Effect>,
    q_all: Query<(&Uid, &Selected, Option<&ProxiedComponent<Selected>>, &Visibility)>,
) {
    let mut spawned_uids = vec![];
    let mut despawned_uids = vec![];
    let mut changed_uids = vec![];

    let mut selection_changed = false;
    let mut inspected = None;
    let mut uninspected = None;

    for ev in ev_spawned.read() {
        match ev {
            ChangesetEvent::Spawned(uid) => spawned_uids.push(*uid),
            ChangesetEvent::Despawned(uid) => despawned_uids.push(*uid),
            ChangesetEvent::Changed(uid, type_id, changed_type) => {
                changed_uids.push(*uid);
                if *type_id == Selected::type_info().type_id() {
                    selection_changed = true;
                }

                if *type_id == Inspected::type_info().type_id() {
                    if matches!(changed_type, ChangedType::Inserted | ChangedType::Applied) {
                        inspected = Some(*uid);
                    } else if matches!(changed_type, ChangedType::Removed) {
                        uninspected = Some(*uid);
                    }
                }
            }
        }
    }

    if !despawned_uids.is_empty() {
        ev_effect_writer.send(Effect::EntitiesDespawned(despawned_uids));
    }
    if !spawned_uids.is_empty() {
        ev_effect_writer.send(Effect::EntitiesSpawned(spawned_uids));
    }
    if !changed_uids.is_empty() {
        ev_effect_writer.send(Effect::EntitiesChanged(changed_uids));
    }
    if selection_changed {
        ev_effect_writer.send(Effect::SelectionChanged(
            q_all
                .iter()
                .filter_map(|(uid, selected, maybe_proxy, _)| {
                    if matches!(*selected, Selected::Selected) && maybe_proxy.is_none() {
                        Some(*uid)
                    } else {
                        None
                    }
                })
                .collect(),
        ));
    }

    if inspected.is_some() || uninspected.is_some() {
        ev_effect_writer.send(Effect::InspectionChanged {
            inspected,
            uninspected,
        });
    }
}

pub fn sys_emit_effects(world: &mut World) {
    let mut effects: VecDeque<Effect> = world.resource_mut::<Events<Effect>>().drain().collect();
    while let Some(ev) = effects.pop_front() {
        let res = world.get_resource_mut::<EffectQue>().unwrap();
        res.push_effect(ev.clone());

        #[allow(clippy::single_match)]
        match ev {
            Effect::InspectionChanged {
                inspected,
                uninspected,
            } => handle_inspection_changed(&mut effects, world, inspected, uninspected),
            _ => (),
        }
    }
    let mut res = world.get_resource_mut::<EffectQue>().unwrap();
    res.forward_effects_to_js();
}

#[allow(non_snake_case, clippy::empty_docs)]
mod effects {
    use bevy::ecs::event::Event;
    use bevy_spts_fragments::prelude::Uid;
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;

    use crate::tools::BobbinTool;

    #[derive(Event, Tsify, Serialize, Deserialize, Debug, Clone)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    #[serde(tag = "tag", content = "value")]
    pub enum Effect {
        Ready,
        // Whenever the selection changes
        SelectionChanged(Vec<Uid>),

        EntitiesSpawned(Vec<Uid>),
        EntitiesChanged(Vec<Uid>),
        EntitiesDespawned(Vec<Uid>),

        InspectionChanged {
            inspected: Option<Uid>,
            uninspected: Option<Uid>,
        },
        ToolChanged(BobbinTool),
    }
}
