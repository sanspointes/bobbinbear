//! Contains logic for effects (side effect events) to keep the JS state synced with the bevy
//! state.

use bevy::{prelude::*, reflect::Typed};

mod js_event_que;

use bevy_spts_changeset::events::{ChangedType, ChangesetEvent};
use bevy_spts_uid::Uid;
pub use effects::*;
pub use js_event_que::EffectQue;

use crate::{inspecting::Inspected, selected::Selected};

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<Effect>();
        app.insert_resource(EffectQue::new());
        app.add_systems(
            Last,
            (sys_collect_changeset_events, sys_emit_effects).chain(),
        );
    }
}

#[allow(clippy::type_complexity)]
/// Collects all the changeset events into effect events.  Sends them via EventWriter
pub fn sys_collect_changeset_events(
    mut ev_spawned: EventReader<ChangesetEvent>,
    mut q_all: ParamSet<(Query<(&Uid, &Selected, &Visibility)>,)>,
    mut effect_writer: EventWriter<Effect>,
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
        effect_writer.send(Effect::EntitiesDespawned(despawned_uids));
    }
    if !spawned_uids.is_empty() {
        effect_writer.send(Effect::EntitiesSpawned(spawned_uids));
    }
    if !changed_uids.is_empty() {
        effect_writer.send(Effect::EntitiesChanged(changed_uids));
    }
    if selection_changed {
        effect_writer.send(Effect::SelectionChanged(
            q_all
                .p0()
                .iter()
                .filter_map(|(uid, selected, _)| {
                    if matches!(*selected, Selected::Selected) {
                        Some(*uid)
                    } else {
                        None
                    }
                })
                .collect(),
        ));
    }

    if inspected.is_some() || uninspected.is_some() {
        effect_writer.send(Effect::InspectionChanged {
            inspected,
            uninspected,
        });
    }
}

pub fn sys_emit_effects(mut res: ResMut<EffectQue>, mut ev_effects: EventReader<Effect>) {
    for ev in ev_effects.read() {
        res.push_effect(ev.clone());
    }
    res.forward_effects_to_js();
}

#[allow(non_snake_case)]
mod effects {
    use bevy::ecs::event::Event;
    use bevy_spts_fragments::prelude::Uid;
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;

    #[derive(Event, Tsify, Serialize, Deserialize, Debug, Clone)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    #[serde(tag = "tag", content = "value")]
    pub enum Effect {
        // Whenever the selection changes
        SelectionChanged(Vec<Uid>),

        EntitiesSpawned(Vec<Uid>),
        EntitiesChanged(Vec<Uid>),
        EntitiesDespawned(Vec<Uid>),

        InspectionChanged {
            inspected: Option<Uid>,
            uninspected: Option<Uid>,
        },
    }
}
