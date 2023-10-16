use std::collections::VecDeque;

use bevy::{
    ecs::system::SystemState,
    input::ButtonState,
    math::Vec3Swizzles,
    prelude::*,
    utils::{AHasher, HashMap, HashSet},
};
use bevy_mod_raycast::RaycastSource;

use crate::{
    components::bbid::{BBId, BBIdUtils},
    msgs::{
        cmds::{move_objects_cmd::MoveObjectsCmd, CmdMsg},
        frontend::FrontendMsg,
        Message,
    },
    plugins::{
        input_plugin::{InputMessage, ModifiersState},
        selection_plugin::{Selectable, Selected},
    },
    types::BBCursor,
    utils::coordinates::world_to_screen,
};

use super::{ToolFsmError, ToolFsmResult, ToolHandlerMessage};

#[derive(Resource, Debug, Clone)]
pub enum SelectFsm {
    Default {
        bbids: HashSet<BBId>,
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
}

pub fn msg_handler_select_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    let fsm = world.resource::<SelectFsm>().clone();

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
        // Handle Pointer down and Click events
        Input(ev @ PointerDown {
            modifiers,
            world: world_pos,
            ..
        })
        | Input(ev @ PointerClick {
            world: world_pos,
            modifiers,
            ..
        }) => {
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
                InputMessage::PointerDown { .. } => fsm.pointer_down(hit_bbid, modifiers, world_pos),
                InputMessage::PointerClick { .. } => fsm.pointer_click(hit_bbid, modifiers),
                _ => panic!("Unhandled InputMessage variant in select_tool. This should never happen."),
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
            let to_remove: HashSet<_> = old.difference(new).collect();
            let to_add: HashSet<_> = new.difference(old).collect();

            for (bbid, mut selected) in world.query::<(&BBId, &mut Selected)>().iter_mut(world) {
                if to_remove.contains(bbid) {
                    *selected = Selected::No;
                }
                if to_add.contains(bbid) {
                    *selected = Selected::Yes;
                }
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
            responses.push_back(CmdMsg::from(cmd).into());
        }

        // When movement selection complete, flag that the last command cannot be repeated.
        Ok((MovingSelected { .. }, Default { .. })) => {
            responses.push_back(CmdMsg::DisallowRepeated.into());
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
        *r_fsm = new_state;
    }
}
