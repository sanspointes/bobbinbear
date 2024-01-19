use std::collections::HashSet;

use bevy::{
    ecs::system::BoxedSystem, input::ButtonState, math::Vec3Swizzles, prelude::*, utils::HashMap,
};

use crate::{
    components::bbid::{BBId, BBIdUtils},
    msgs::{
        api::ApiEffectMsg,
        cmds::{
            inspect_cmd::{InspectCmd, InspectingTag}, move_objects_cmd::MoveObjectsCmd,
            select_objects_cmd::SelectObjectsCmd, Cmd, CmdMsg, MultiCmd,
        },
        MsgQue,
    },
    plugins::{
        input_plugin::InputMessage,
        selection_plugin::{get_raycast_hits_selectable, Selected},
    },
    types::BBCursor,
};

use super::{ToolHandler, ToolHandlerMessage};

#[derive(Resource, Debug, Clone)]
pub enum SelectFsm {
    Default {
        bbids: HashSet<BBId>,
    },
    PointerDownAt {
        bbids: HashSet<BBId>,
        initial_world_pos: Vec2,
    },
    MovingSelected {
        bbids: HashSet<BBId>,
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

pub struct OnSelectMoved(Vec<BoxedSystem>);
impl OnSelectMoved {
    /// Run a callback system every time this event listener is triggered. This can be a closure or
    /// a function, as described by bevy's documentation. The only notable difference from Bevy
    /// systems is that the callback system can access a resource with event data,
    /// [`ListenerInput`]. You can more easily access this with the system params
    /// [`Listener`](crate::callbacks::Listener) and [`ListenerMut`](crate::callbacks::ListenerMut).
    pub fn run<Marker>(callback: impl IntoSystem<(), (), Marker>) -> Self {
        Self(vec![Box::new(IntoSystem::into_system(callback))])
    }
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
    fn reset(&self) -> SelectFsm {
        #[cfg(feature = "debug_select")]
        debug!("SelectFsm.reset()");

        let result = match self {
            Self::Default { bbids } => Self::Default {
                bbids: bbids.clone(),
            },
            Self::PointerDownAt { bbids, .. } => Self::Default {
                bbids: bbids.clone(),
            },
            Self::MovingSelected {
                initial_positions, ..
            } => Self::Default {
                bbids: initial_positions.clone().into_keys().collect(),
            },
        };

        result
    }

    pub fn get_selected(&self) -> &HashSet<BBId> {
        match self {
            Self::Default { bbids } => bbids,
            Self::MovingSelected { bbids, .. } => bbids,
            Self::PointerDownAt { bbids, .. } => bbids,
        }
    }
    pub fn get_selected_mut(&mut self) -> &mut HashSet<BBId> {
        match self {
            Self::Default { bbids } => bbids,
            Self::MovingSelected { bbids, .. } => bbids,
            Self::PointerDownAt { bbids, .. } => bbids,
        }
    }

    pub fn set_selected(&mut self, world: &mut World, new: HashSet<BBId>, responder: &mut MsgQue) {
        let mut old = self.get_selected_mut();
        let to_select: Vec<BBId> = new.difference(old).cloned().collect();
        let to_deselect: Vec<BBId> = old.difference(&new).cloned().collect();

        let cmd = SelectObjectsCmd::select_deselect(to_select, to_deselect);
        responder.push_internal(CmdMsg::from(cmd));

        *old = new;
    }

    pub fn select_single(&mut self, world: &mut World, bbid: BBId, responder: &mut MsgQue) {
        let mut new = HashSet::new();
        new.insert(bbid);
        self.set_selected(world, new, responder);
    }

    pub fn add_select_single(&mut self, world: &mut World, bbid: BBId, responder: &mut MsgQue) {
        let mut new = self.get_selected().clone();
        new.insert(bbid);
        self.set_selected(world, new, responder);
    }
    pub fn remove_select_single(&mut self, world: &mut World, bbid: BBId, responder: &mut MsgQue) {
        let mut new = self.get_selected().clone();
        new.remove(&bbid);
        self.set_selected(world, new, responder);
    }
    pub fn clear_select(&mut self, world: &mut World, responder: &mut MsgQue) {
        let new = HashSet::new();
        self.set_selected(world, new, responder);
    }
}

pub struct SelectTool;

impl ToolHandler for SelectTool {
    fn setup(world: &mut World) {}
    fn handle_msg(world: &mut World, msg: &ToolHandlerMessage, responder: &mut MsgQue) {
        let _span = debug_span!("msg_handler_select_tool").entered();

        let mut fsm = world.resource::<SelectFsm>().clone();

        use InputMessage::*;
        use ToolHandlerMessage::*;

        let next_fsm = match msg {
            OnActivate => {
                debug!("SelectTool::OnActivate");
                responder.notify_effect(ApiEffectMsg::SetCursor(BBCursor::Default));
                fsm
            }
            OnDeactivate => {
                debug!("SelectTool::OnDeactivate");
                fsm.reset()
            }
            Input(PointerDown {
                world: world_pos, ..
            }) => match fsm {
                SelectFsm::Default { bbids } => SelectFsm::PointerDownAt {
                    bbids,
                    initial_world_pos: *world_pos,
                },
                state => {
                    panic!("select_tool: Input(PointerDown) should never occur in state {state:?}.")
                }
            },
            Input(PointerClick {
                modifiers,
                ..
            }) => {
                let hit = get_raycast_hits_selectable(world).first().cloned();
                println!(
                    "SelectFsm (Input(PointerClick)) {:?}, {:?} {:?}",
                    &fsm, hit, modifiers.shift
                );

                match (&fsm, hit, modifiers.shift) {
                    (
                        SelectFsm::PointerDownAt { .. },
                        Some((entity, _)),
                        ButtonState::Released,
                    ) => {
                        let bbid = world.get::<BBId>(entity).unwrap();
                        println!("Select single {bbid}");
                        fsm.select_single(world, *bbid, responder);
                    }
                    (
                        SelectFsm::PointerDownAt { bbids, .. },
                        Some((entity, _)),
                        ButtonState::Pressed,
                    ) => {
                        let bbid = world.get::<BBId>(entity).unwrap();
                        println!("Add select single {bbid}");
                        if bbids.contains(bbid) {
                            fsm.remove_select_single(world, *bbid, responder);
                        } else {
                            fsm.add_select_single(world, *bbid, responder);
                        }
                    }
                    (SelectFsm::PointerDownAt { .. }, None, ButtonState::Released) => {
                        fsm.clear_select(world, responder);
                        let inspected_entity = world.query::<&InspectingTag>().get_single(world);
                        if inspected_entity.is_ok() {
                            let cmd = InspectCmd::uninspect();
                            responder.push_internal(cmd);
                        }
                    }
                    _ => (),
                }

                SelectFsm::Default {
                    bbids: fsm.get_selected().clone(),
                }
            }
            Input(DoubleClick {
                ..
            }) => {
                let hit = get_raycast_hits_selectable(world).first().cloned();
                match (&fsm, hit) {
                    (
                        SelectFsm::PointerDownAt {
                            bbids,
                            ..
                        },
                        Some((entity, _)),
                    ) => {
                        let target = world.get::<BBId>(entity).copied();
                        let mut cmds: Vec<Box<dyn Cmd>> = vec![];
                        let bbids_vec: Vec<_> = bbids.iter().cloned().collect();
                        cmds.push(Box::new(SelectObjectsCmd::deselect(bbids_vec)));
                        cmds.push(Box::new(InspectCmd::new(target)));
                        responder.push_internal(CmdMsg::from(MultiCmd::new(cmds)));

                        SelectFsm::Default {
                            bbids: bbids.clone(),
                        }
                    }
                    _ => fsm,
                }
            }
            Input(DragStart {
                world_offset,
                modifiers,
                ..
            }) => {
                let hit = get_raycast_hits_selectable(world).first().cloned();

                println!("Drag start {hit:?}.");
                match (&fsm, hit, modifiers.shift) {
                    (
                        SelectFsm::PointerDownAt {
                            bbids,
                            ..
                        },
                        None,
                        _,
                    ) => SelectFsm::Default {
                        bbids: bbids.clone(),
                    },
                    (
                        SelectFsm::PointerDownAt {
                            initial_world_pos,
                            ..
                        },
                        Some((entity, _)),
                        button_state,
                    ) => {
                        let initial_world_pos = *initial_world_pos;
                        let target = *world.get::<BBId>(entity).unwrap();
                        println!("Drag Start. Target: {target:?}");
                        match button_state {
                            ButtonState::Released => {
                                fsm.select_single(world, target, responder);
                            }
                            ButtonState::Pressed => fsm.add_select_single(world, target, responder),
                        }
                        let bbids = fsm.get_selected().clone();
                        let to_move: Vec<_> =
                            bbids.iter().map(|bbid| (world.bbid(*bbid), bbid)).collect();

                        let initial_positions: HashMap<_, _> = to_move
                            .into_iter()
                            .map(|(e, bbid)| {
                                let pos = world.get::<Transform>(e).unwrap().translation.xy();
                                (*bbid, pos)
                            })
                            .collect();
                        let cmd = MoveObjectsCmd::from_multiple(
                            initial_positions.keys().cloned().collect(),
                            *world_offset,
                        );
                        responder.push_internal(cmd);
                        SelectFsm::MovingSelected {
                            bbids,
                            initial_positions,
                            initial_world_pos,
                            world_offset: *world_offset,
                        }
                    }
                    state => panic!(
                        "select_tool: Input(DragStart) should never occur in state ({state:?})."
                    ),
                }
            }
            Input(DragMove { world_offset, .. }) => match fsm {
                SelectFsm::Default { .. } => fsm,
                SelectFsm::MovingSelected {
                    bbids,
                    initial_positions,
                    initial_world_pos,
                    ..
                } => {
                    let cmd = MoveObjectsCmd::from_multiple(
                        initial_positions.keys().cloned().collect(),
                        *world_offset,
                    );
                    responder.push_internal(cmd);
                    SelectFsm::MovingSelected {
                        bbids,
                        initial_positions,
                        initial_world_pos,
                        world_offset: *world_offset,
                    }
                }
                state => {
                    panic!("select_tool: Input(DragMove) should never occur in state {state:?}.")
                }
            },
            Input(DragEnd { world_offset, .. }) => match &fsm {
                SelectFsm::Default { .. } => fsm,
                SelectFsm::MovingSelected {
                    bbids,
                    initial_positions,
                    ..
                } => {
                    let cmd = MoveObjectsCmd::from_multiple(
                        initial_positions.keys().cloned().collect(),
                        *world_offset,
                    );
                    responder.push_internal(cmd);
                    SelectFsm::Default {
                        bbids: bbids.clone(),
                    }
                }
                state => {
                    panic!("select_tool: Input(DragEnd) should never occur in state {state:?}.")
                }
            },
            _ => fsm,
        };

        bevy_debug_text_overlay::screen_print!("SelectFsm: {:?}", &next_fsm);

        let mut fsm = world.resource_mut::<SelectFsm>();
        *fsm = next_fsm;
    }

    fn handle_effects(_world: &mut World, _event: &crate::msgs::effect::EffectMsg) {}
}
