use bevy::{
    ecs::system::SystemState,
    input::ButtonState,
    math::Vec3Swizzles,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_mod_raycast::prelude::RaycastSource;

use crate::{
    components::bbid::BBId,
    msgs::{
        api::ApiEffectMsg,
        cmds::{
            inspect_cmd::InspectCmd, move_objects_cmd::MoveObjectsCmd,
            select_objects_cmd::SelectObjectsCmd, Cmd, CmdMsg, MultiCmd,
        },
        MsgQue,
    },
    plugins::{
        input_plugin::{InputMessage, ModifiersState},
        inspect_plugin::InspectState,
        selection_plugin::{Selectable, Selected},
    },
    types::BBCursor,
};

use super::{ToolFsmError, ToolFsmResult, ToolHandlerMessage};

#[derive(Resource, Debug, Clone)]
pub enum SelectFsm {
    Default {
        bbids: HashSet<BBId>,
    },
    DoubleClickReturn {
        bbids: HashSet<BBId>,
        target: BBId,
    },
    AwaitingMoveSelected {
        bbids: HashSet<BBId>,
        initial_world_pos: Vec2,
    },
    MovingSelected {
        initial_positions: HashMap<BBId, Vec2>,
        initial_world_pos: Vec2,
        world_offset: Vec2,
    }, // Dragging {},
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
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.reset()");

        let result = match self {
            Self::Default { bbids } => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
            Self::AwaitingMoveSelected { bbids, .. } => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
            Self::MovingSelected {
                initial_positions, ..
            } => Ok(Self::Default {
                bbids: initial_positions.clone().into_keys().collect(),
            }),
            Self::DoubleClickReturn { bbids, .. } => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
        };

        result.map(|new| (self.clone(), new))
    }

    fn pointer_down(
        &self,
        bbid: Option<&BBId>,
        modifiers: &ModifiersState,
        initial_world_pos: &Vec2,
    ) -> ToolFsmResult<SelectFsm> {
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.pointer_down_with_hover(bbid: {bbid:?}, modifiers: {modifiers:?})");

        let result = match self {
            Self::Default { bbids } => {
                let pointer_down_on_selected = match bbid {
                    Some(bbid) => bbids.contains(bbid),
                    None => false,
                };

                match (pointer_down_on_selected, modifiers.shift) {
                    // If pointer down on already selected, do not deselect / add select.
                    (true, _) => Ok(Self::AwaitingMoveSelected {
                        bbids: bbids.clone(),
                        initial_world_pos: *initial_world_pos,
                    }),

                    // If pointer down and not pressing shift, just select the new element
                    (false, ButtonState::Released) => {
                        let mut bbids = HashSet::new();
                        if let Some(bbid) = bbid {
                            bbids.insert(*bbid);
                        }
                        Ok(Self::AwaitingMoveSelected {
                            bbids,
                            initial_world_pos: *initial_world_pos,
                        })
                    }

                    // If pointer down and not pressing try to add new element to selection
                    (false, ButtonState::Pressed) => {
                        let mut bbids = bbids.clone();
                        if let Some(bbid) = bbid {
                            bbids.insert(*bbid);
                        }
                        Ok(Self::AwaitingMoveSelected {
                            bbids,
                            initial_world_pos: *initial_world_pos,
                        })
                    }
                }
            }
            _ => Err(ToolFsmError::NoTransition),
        };

        result.map(|new| (self.clone(), new))
    }

    fn pointer_click(
        &self,
        bbid: Option<&BBId>,
        modifiers: &ModifiersState,
    ) -> ToolFsmResult<SelectFsm> {
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.pointer_click(bbid: {bbid:?}, modifiers: {modifiers:?})");

        let result = match (self, modifiers.shift) {
            // If click without shift, deselect all but clicked element
            (Self::AwaitingMoveSelected { bbids, .. }, ButtonState::Released) => {
                let mut bbids = HashSet::new();
                if let Some(bbid) = bbid {
                    bbids.insert(*bbid);
                }
                Ok(Self::Default { bbids })
            }
            // If click WITH shift, element has already been selected by pointerdown so transiton
            // back to default but keep selection.
            (Self::AwaitingMoveSelected { bbids, .. }, ButtonState::Pressed) => Ok(Self::Default {
                bbids: bbids.clone(),
            }),
            (_, _) => Err(ToolFsmError::NoTransition),
        };

        result.map(|new| (self.clone(), new))
    }

    fn start_drag(&self, world_offset: &Vec2, world: &mut World) -> ToolFsmResult<SelectFsm> {
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.start_drag(world_offset: {world_offset:?})");

        let result = match self {
            Self::AwaitingMoveSelected {
                bbids,
                initial_world_pos,
            } => {
                let mut q_dragging = world.query::<(&BBId, &Selected, &Transform)>();

                let iter_selected_and_known =
                    q_dragging.iter(world).filter(|(bbid, selected, _)| {
                        matches!(selected, Selected::Yes) && bbids.contains(*bbid)
                    });

                let initial_positions: HashMap<BBId, Vec2> = iter_selected_and_known.fold(
                    HashMap::new(),
                    |mut map, (bbid, _, transform)| {
                        map.insert(*bbid, transform.translation.xy());
                        map
                    },
                );
                Ok(SelectFsm::MovingSelected {
                    initial_positions,
                    initial_world_pos: *initial_world_pos,
                    world_offset: *world_offset,
                })
            }
            _ => Err(ToolFsmError::NoTransition),
        };

        result.map(|new| (self.clone(), new))
    }

    fn move_drag(&self, world_offset: &Vec2) -> ToolFsmResult<SelectFsm> {
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.move_drag(world_offset: {world_offset:?})");

        let result = match self {
            Self::MovingSelected {
                initial_positions,
                initial_world_pos,
                ..
            } => Ok(SelectFsm::MovingSelected {
                initial_positions: initial_positions.clone(),
                initial_world_pos: *initial_world_pos,
                world_offset: *world_offset,
            }),
            _ => Err(ToolFsmError::NoTransition),
        };

        result.map(|new| (self.clone(), new))
    }

    fn end_drag(&self) -> ToolFsmResult<SelectFsm> {
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.end_drag()");

        let result = match self {
            Self::MovingSelected {
                initial_positions, ..
            } => Ok(SelectFsm::Default {
                bbids: initial_positions.clone().into_keys().collect(),
            }),
            _ => Err(ToolFsmError::NoTransition),
        };

        result.map(|new| (self.clone(), new))
    }

    fn double_click(&self, bbid: Option<&BBId>) -> ToolFsmResult<SelectFsm> {
        let result = match (self, bbid) {
            (Self::AwaitingMoveSelected { bbids, .. }, Some(bbid)) => Ok(Self::DoubleClickReturn {
                target: *bbid,
                bbids: bbids.clone(),
            }),
            _ => Err(ToolFsmError::NoTransition),
        };

        result.map(|new| (self.clone(), new))
    }
}

pub fn msg_handler_select_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responder: &mut MsgQue,
) {
    let _span = debug_span!("msg_handler_select_tool").entered();

    let fsm = world.resource::<SelectFsm>().clone();

    use InputMessage::*;
    use ToolHandlerMessage::*;

    let transition_result = match message {
        OnActivate => {
            debug!("SelectTool::OnActivate");
            responder.notify_effect(ApiEffectMsg::SetCursor(BBCursor::Default));
            Err(ToolFsmError::NoTransition)
        }
        OnDeactivate => {
            debug!("SelectTool::OnDeactivate");
            fsm.reset()
        }
        // Handle Pointer down and Click events
        Input(
            ev @ PointerDown {
                modifiers,
                world: world_pos,
                ..
            },
        )
        | Input(
            ev @ PointerClick {
                world: world_pos,
                modifiers,
                ..
            },
        )
        | Input(
            ev @ DoubleClick {
                world: world_pos,
                modifiers,
                ..
            },
        ) => {
            let mut select_sys_state = SystemState::<(
                // Selectables
                Query<(&BBId, &Selectable, &mut Selected, &Transform)>,
                // Raycaster
                Query<&RaycastSource<Selectable>>,
            )>::new(world);
            let (mut q_selectables, q_rc_source) = select_sys_state.get_mut(world);
            let hits = q_rc_source.single().intersections();

            let hit_bbid = match hits.first() {
                // Early exit, if no hit then deselect.
                None => None,
                Some((hit_entity, data)) => {
                    let Ok((bbid, selectable, _, _)) = q_selectables.get_mut(*hit_entity) else {
                        error!("SelectTool: Hit entity {hit_entity:?} but querying for it failed.\n Hit data: {data:?}");
                        return;
                    };

                    if matches!(selectable, Selectable::Locked) {
                        debug!("SelectTool: Hit entity {hit_entity:?} with {bbid:?} but entity is {selectable:?}.");
                        return;
                    }

                    Some(bbid)
                }
            };

            

            match ev {
                InputMessage::PointerDown { .. } => {
                    fsm.pointer_down(hit_bbid, modifiers, world_pos)
                }
                InputMessage::PointerClick { .. } => fsm.pointer_click(hit_bbid, modifiers),
                InputMessage::DoubleClick { .. } => fsm.double_click(hit_bbid),
                _ => panic!(
                    "Unhandled InputMessage variant in select_tool. This should never happen."
                ),
            }
        }
        Input(DragStart { world_offset, .. }) => fsm.start_drag(world_offset, world),
        Input(DragMove { world_offset, .. }) => fsm.move_drag(world_offset),
        Input(DragEnd { .. }) => fsm.end_drag(),

        _ => Err(ToolFsmError::NoTransition),
    };

    #[cfg(feature = "debug_select")]
    if let Ok((ref old, ref new)) = transition_result {
        println!("Old: {old:?}\n New: {new:?}");
    }

    use SelectFsm::*;
    match &transition_result {
        //
        // On Default <-> Default or Default <-> AwaitingDrag, just select/deselect the entities
        //
        Ok((Default { bbids: old }, Default { bbids: new }))
        | Ok((AwaitingMoveSelected { bbids: old, .. }, Default { bbids: new }))
        | Ok((Default { bbids: old }, AwaitingMoveSelected { bbids: new, .. })) => {
            // Uninspect if click outside of selected element.
            let state = world.resource::<State<InspectState>>();
            info!("{state:?} {:?}", new);
            if new.is_empty() && !state.eq(&InspectState::None)  {
                world.resource_mut::<NextState<InspectState>>().set(InspectState::None);
            }

            if !new.eq(old) {
                let to_select: Vec<BBId> = new.difference(old).cloned().collect();
                let to_deselect: Vec<BBId> = old.difference(new).cloned().collect();

                let cmd = SelectObjectsCmd::select_deselect(to_select, to_deselect);
                responder.push_internal(CmdMsg::from(cmd))
            }
        }

        // When starting or continuing to move selected elements
        Ok((
            AwaitingMoveSelected { .. },
            MovingSelected {
                initial_positions,
                world_offset,
                ..
            },
        ))
        | Ok((
            MovingSelected { .. },
            MovingSelected {
                initial_positions,
                world_offset,
                ..
            },
        )) => {
            let bbids: Vec<_> = initial_positions.keys().cloned().collect();
            let cmd = MoveObjectsCmd::from_multiple(bbids, *world_offset);
            responder.push_internal(CmdMsg::from(cmd));
        }

        // When movement selection complete, flag that the last command cannot be repeated.
        Ok((MovingSelected { .. }, Default { .. })) => {
            responder.push_internal(CmdMsg::DisallowRepeated);
        }

        Ok((AwaitingMoveSelected { bbids, .. }, DoubleClickReturn { target, .. })) => {
            let mut cmds: Vec<Box<dyn Cmd>> = Vec::new();
            let bbids: Vec<_> = bbids.into_iter().cloned().collect();
            cmds.push(Box::new(SelectObjectsCmd::deselect(bbids)));
            cmds.push(Box::new(InspectCmd::inspect(*target)));
            let cmd = MultiCmd::new(cmds);
            responder.push_internal(CmdMsg::from(cmd));
        }

        // Error on valid transitions that are not handled
        Ok((from, to)) => {
            panic!("SelectTool: Unhandled state transition from tool message({message:?})\n - From {from:?}\n - To {to:?}.")
        }

        Err(_) => {}
    }

    // Save the new state back in the resource.
    if let Ok((_, new_state)) = transition_result {
        let mut r_fsm = world.resource_mut::<SelectFsm>();
        match new_state {
            DoubleClickReturn { bbids, .. } => {
                *r_fsm = Default {
                    bbids: bbids.clone(),
                }
            }
            new_state => {
                *r_fsm = new_state;
            }
        }
    }
}
