use std::collections::VecDeque;

use bevy::{ecs::system::SystemState, input::ButtonState, prelude::*, utils::HashSet};
use bevy_mod_raycast::RaycastSource;

use crate::{
    components::bbid::BBId,
    msgs::{frontend::FrontendMsg, Message},
    plugins::{
        input_plugin::{InputMessage, ModifiersState},
        selection_plugin::{Selectable, Selected},
    },
    types::BBCursor,
};

use super::{ToolFsmError, ToolFsmResult, ToolHandlerMessage};

#[derive(Resource, Debug, Clone)]
pub enum SelectFsm {
    Default { bbids: HashSet<BBId> },
    AwaitingDrag { bbids: HashSet<BBId> },
    // Dragging {},
    // SelectionBox {
    //     initial_world_pos: Vec2,
    //     min_pos: Vec2,
    //     max_pos: Vec2,
    // }
}

impl Default for SelectFsm {
    fn default() -> Self {
        Self::Default {
            bbids: HashSet::new(),
        }
    }
}

impl SelectFsm {
    /// Resets to default state, persisting any data that it can.
    fn reset(&self) -> ToolFsmResult<SelectFsm> {
        debug!("SelectFsm.reset()");
        match self {
            Self::Default { bbids } => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
            Self::AwaitingDrag { bbids } => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
        }
        .map(|new| (self.clone(), new))
    }

    fn pointer_down_with_hover(
        &self,
        bbid: BBId,
        modifiers: &ModifiersState,
    ) -> ToolFsmResult<SelectFsm> {
        debug!("SelectFsm.pointer_down_with_hover(bbid: {bbid:?}, modifiers: {modifiers:?})");
        match (self, modifiers.shift) {
            // When shift not pressed, selecting single
            (Self::Default { .. }, ButtonState::Released) => {
                let mut bbids = HashSet::new();
                bbids.insert(bbid);
                Ok(Self::AwaitingDrag { bbids })
            }
            (Self::Default { bbids }, ButtonState::Pressed) => {
                let mut bbids = bbids.clone();
                bbids.insert(bbid);
                Ok(Self::AwaitingDrag { bbids })
            }
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (self.clone(), new))
    }

    fn pointer_down_without_hover(&self, modifiers: &ModifiersState) -> ToolFsmResult<SelectFsm> {
        debug!("SelectFsm.pointer_down_without_hover(modifiers: {modifiers:?})");
        use ButtonState::*;
        match (self, modifiers.shift) {
            (Self::Default { .. }, Released) => Ok(Self::Default {
                bbids: HashSet::new(),
            }),
            (Self::Default { bbids }, Pressed) => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (self.clone(), new))
    }
}

pub fn msg_handler_select_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut select_sys_state = SystemState::<(
        // Selectables
        Query<(&BBId, &Selectable, &mut Selected, &Transform)>,
        // Raycaster
        Query<&RaycastSource<Selectable>>,
        // Prev hovers
        ResMut<SelectFsm>,
    )>::new(world);

    let (mut q_selectables, q_rc_source, mut fsm) = select_sys_state.get_mut(world);

    use InputMessage::*;
    use ToolHandlerMessage::*;
    let transition_result = match message {
        OnActivate => {
            debug!("SelectTool::OnActivate");
            responses.push_back(FrontendMsg::SetCursor(BBCursor::Default).into());
            Err(ToolFsmError::NoTransition)
        }
        OnDeactivate => {
            debug!("SelectTool::OnDeactivate");
            fsm.reset()
        }
        Input(PointerDown { modifiers, .. }) => {
            let rc_source = q_rc_source.single();

            let hits = rc_source.intersections();

            match hits.first() {
                // Early exit, if no hit then deselect.
                None => fsm.pointer_down_without_hover(modifiers),
                Some((hit_entity, data)) => {
                    let Ok((bbid, selectable, _, _)) = q_selectables.get_mut(*hit_entity) else {
                        error!("SelectTool: Hit entity {hit_entity:?} but querying for it failed.\n Hit data: {data:?}");
                        return;
                    };

                    if matches!(selectable, Selectable::Locked) {
                        debug!("SelectTool: Hit entity {hit_entity:?} with {bbid:?} but entity is {selectable:?}.");
                        return;
                    }

                    fsm.pointer_down_with_hover(*bbid, modifiers)
                }
            }
        }
        Input(PointerClick { .. }) => fsm.reset(),
        _ => Err(ToolFsmError::NoTransition),
    };

    if let Ok((ref old, ref new)) = transition_result {
        println!("Old: {old:?}\n New: {new:?}");
    }

    use SelectFsm::*;
    match &transition_result {
        //
        // On Default <-> Default or Default <-> AwaitingDrag, just select/deselect the entities
        //
        Ok((Default { bbids: old }, Default { bbids: new }))
        | Ok((AwaitingDrag { bbids: old }, Default { bbids: new }))
        | Ok((Default { bbids: old }, AwaitingDrag { bbids: new })) => {
            let to_remove: HashSet<_> = old.difference(new).collect();
            let to_add: HashSet<_> = new.difference(old).collect();

            for (bbid, _, mut selected, _transform) in q_selectables.iter_mut() {
                if to_remove.contains(bbid) {
                    *selected = Selected::No;
                }
                if to_add.contains(bbid) {
                    *selected = Selected::Yes;
                }
            }
        }
        Ok((from, to)) => {
            panic!("SelectTool: Unhandled state transition from tool message({message:?})\n - From {from:?}\n - To {to:?}.")
        }
        Err(_) => {}
    }

    // Save the new state back in the resource.
    if let Ok((_, new_state)) = transition_result {
        *fsm = new_state;
    }
}
