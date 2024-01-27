//! Pen Tool

mod pen_edge;

use std::default;

use bb_vector_network::prelude::{BBEdge, BBNodeIndex};
use bevy::{prelude::*, sprite::Mesh2dHandle, reflect::VariantFieldIter};
use bevy_debug_text_overlay::screen_print;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBIndex, BBNode},
    },
    constants::Z_INDEX_BB_NODE,
    msgs::{
        cmds::{inspect_cmd::InspectingTag, MultiCmd},
        effect::EffectMsg,
        tools::pen_tool::pen_edge::PenEdgeVariant,
        MsgQue,
    },
    plugins::{
        inspect_plugin::InspectArtifact, screen_space_root_plugin::ScreenSpaceRoot,
        selection_plugin::get_raycast_hits_selectable, vector_graph_plugin::VectorGraph,
    },
    shared::{CachedMeshes, WorldUtils},
    utils::coordinates::{WorldToLocal, WorldToScreenHelpers},
};

use self::pen_edge::PenEdge2;

use super::{InputMessage, ToolHandler, ToolHandlerMessage};

pub use pen_edge::sys_setup_pen_resource;

#[derive(Resource, Debug, Clone, Default)]
pub enum PenFsm {
    #[default]
    Default,
    /// When Pen tool is constructing a line from a given node/pos to the cursor.
    BuildingEdge {
        /// The temporary edge that's being constructed
        edge: PenEdge2,
        /// The node of that edge that we're currently building
        building_target: BBNode,
    },
    /// When the pen tool has added an edge and is expecting to receive the EdgeAdded event to
    /// update its own internal state.
    AwaitingAddedEdge { target: BBId, next_edge: PenEdge2 },
}

#[derive(Resource, Debug, Clone, Default)]
pub struct PenResource {
    state: PenFsm,
    cursor_node: Option<Entity>,
    cursor_line: Option<Entity>,
    cursor_line_node: Option<Vec<Entity>>,
}

enum CursorNodeStyle {
    Hidden,
    Control,
    Endpoint,
}

pub struct PenTool;
impl PenTool {
    fn update_cursor_node_style(
        world: &mut World,
        cursor_world: Entity,
        style: CursorNodeStyle,
        screen_pos: Vec2,
    ) {
        let entity = cursor_world;
        match style {
            CursorNodeStyle::Endpoint => {
                let handle = world
                    .resource::<CachedMeshes>()
                    .endpoint_node
                    .as_ref()
                    .unwrap()
                    .clone();
                *world.get_mut::<Mesh2dHandle>(entity).unwrap() = handle;
                *world.get_mut::<Visibility>(entity).unwrap() = Visibility::Visible;
                world.get_mut::<Transform>(entity).unwrap().translation =
                    screen_pos.extend(Z_INDEX_BB_NODE);
            }
            CursorNodeStyle::Control => {
                let handle = world
                    .resource::<CachedMeshes>()
                    .control_node
                    .as_ref()
                    .unwrap()
                    .clone();
                *world.get_mut::<Mesh2dHandle>(entity).unwrap() = handle;
                *world.get_mut::<Visibility>(entity).unwrap() = Visibility::Visible;
                world.get_mut::<Transform>(entity).unwrap().translation =
                    screen_pos.extend(Z_INDEX_BB_NODE);
            }
            CursorNodeStyle::Hidden => {
                *world.get_mut::<Visibility>(entity).unwrap() = Visibility::Hidden;
            }
        }
    }
}

impl ToolHandler for PenTool {
    fn setup(world: &mut World) {}

    fn handle_msg(world: &mut World, msg: &ToolHandlerMessage, responder: &mut MsgQue) {
        use InputMessage::*;
        use ToolHandlerMessage::*;

        let state = world.resource::<PenResource>().state.clone();
        screen_print!("PenState: {state:?}");
        let next_state = match msg {
            Input(PointerClick {
                world: world_pos,
                modifiers: _modifiers,
                ..
            }) => {
                let hit = get_raycast_hits_selectable(world).first().cloned();
                let target = world
                    .query_filtered::<&BBId, With<InspectingTag>>()
                    .get_single(world)
                    .ok()
                    .copied();

                let hit_node = {
                    hit.as_ref()
                        .and_then(|(e, _)| world.get::<BBIndex>(*e))
                        .map(|idx| BBNodeIndex(idx.0))
                };

                println!("Click - target: {target:?}");

                match state {
                    PenFsm::Default => match (target, hit_node) {
                        (Some(target), Some(node_idx)) => {
                            let edge = PenEdge2::new_line_from_node(world, target, node_idx);
                            let building_target = BBNode::Endpoint;
                            PenFsm::BuildingEdge {edge, building_target}
                        },
                        (target, None) => {
                            let edge = PenEdge2::World(target, PenEdgeVariant::Line { start: *world_pos, start_node: None, end: *world_pos, end_node: None });
                            let building_target = BBNode::Endpoint;
                            PenFsm::BuildingEdge {edge, building_target}
                        },
                        (None, Some(_)) => panic!("PenTool: Input(PointerClick) impossible state. Can't have no target but reference BBNodeIndex."),
                    },
                    PenFsm::BuildingEdge{ mut edge, .. } => {
                        *edge.variant_mut().end_node_mut() = hit_node;

                        println!("Adding {edge:?}");

                        let (target, cmds) = edge.get_commands(world, *world_pos);
                        responder.push_internal(MultiCmd::new(cmds));

                        let end_pos = edge.variant().end();
                        let end_node = edge.variant().end_node();
                        let variant = PenEdgeVariant::Line { start: end_pos, start_node: end_node, end: end_pos, end_node: None };
                        PenFsm::AwaitingAddedEdge { target, next_edge: PenEdge2::Local(target, variant) }
                    }
                    pen_fsm => panic!("PenTool: Unhandled state {pen_fsm:?}."),
                }
            }

            Input(PointerMove {
                screen,
                world: world_pos,
                ..
            }) => {
                let hit = get_raycast_hits_selectable(world)
                    .first()
                    .cloned()
                    .and_then(|(e, _)| match world.has_component::<BBNode>(e) {
                        true => Some(e),
                        false => None,
                    });

                let cursor_line = world.resource::<PenResource>().cursor_line.unwrap();

                match &state {
                    PenFsm::Default | PenFsm::AwaitingAddedEdge { .. } => {
                        *world.get_mut::<Visibility>(cursor_line).unwrap() = Visibility::Hidden;
                        state
                    }
                    PenFsm::BuildingEdge {
                        edge,
                        building_target,
                    } => {
                        let target = edge
                            .target()
                            .or(hit.map(|e| world.get::<InspectArtifact>(e).unwrap().0));
                        let hit_idx = hit.map(|e| BBNodeIndex(world.get::<BBIndex>(e).unwrap().0));

                        let next_edge = match target {
                            Some(target) => edge.to_local(world, target),
                            None => edge.to_world(world),
                        };

                        let next_edge = match next_edge {
                            PenEdge2::Local(target, mut edge) => {
                                let world_matrix =
                                    world.bbid_get::<GlobalTransform>(target).compute_matrix();
                                *edge.end_node_mut() = hit_idx;
                                *edge.end_mut() = world_pos.world_to_local(&world_matrix);
                                PenEdge2::Local(target, edge)
                            }
                            PenEdge2::World(target, mut edge) => {
                                *edge.end_node_mut() = hit_idx;
                                *edge.end_mut() = *world_pos;
                                PenEdge2::World(target, edge)
                            }
                            _ => panic!("Impossible"),
                        };

                        next_edge.draw(world);

                        PenFsm::BuildingEdge {
                            edge: next_edge,
                            building_target: *building_target,
                        }
                    }
                }
            }

            Input(DragStart {
                world: world_pos,
                world_pressed,
                ..
            }) => {
                let hit = get_raycast_hits_selectable(world)
                    .first()
                    .cloned()
                    .and_then(|(e, _)| match world.has_component::<BBNode>(e) {
                        true => Some(e),
                        false => None,
                    });

                match &state {
                    PenFsm::Default => {
                        let target = hit.map(|e| world.get::<InspectArtifact>(e).unwrap().0);
                        let hit_node = hit.map(|e| BBNodeIndex(world.get::<BBIndex>(e).unwrap().0));

                        match (target, hit_node) {
                            (Some(target), Some(node_idx)) => {
                                let pos = world.bbid_get::<VectorGraph>(target).0.node(node_idx).unwrap().position();
                                let world_matrix =
                                    world.bbid_get::<GlobalTransform>(target).compute_matrix();
                                let end_pos = world_pos.world_to_local(&world_matrix);

                                let edge = PenEdge2::Local(target, PenEdgeVariant::Quadratic { start: pos, start_node: Some(node_idx), ctrl1: end_pos, end: end_pos, end_node: None });
                                let building_target = BBNode::Ctrl1;

                                PenFsm::BuildingEdge {edge, building_target}
                            },
                            (target, None) => {
                                let edge = PenEdge2::World(target, PenEdgeVariant::Quadratic { start: *world_pressed, start_node: None, ctrl1: *world_pos, end: *world_pos, end_node: None });
                                let building_target = BBNode::Ctrl1;
                                PenFsm::BuildingEdge {edge, building_target}
                            },
                            (None, Some(_)) => panic!("PenTool: Input(DragStart) impossible state. Can't have no target but reference BBNodeIndex."),
                        }
                    }
                    PenFsm::BuildingEdge {
                        mut edge,
                        building_target,
                    } => {
                        let mouse_pos = edge.world_to_coordinate_space(world, *world_pos);

                        let (next_variant, next_building_target) = match (edge.variant_mut(), building_target) {
                            (variant @ PenEdgeVariant::Line { .. }, BBNode::Endpoint) => {
                                let mut next_variant = variant.as_quadratic(mouse_pos);
                                *next_variant.end_mut() = mouse_pos;
                                let next_building_target = BBNode::Ctrl2;
                                ( next_variant, next_building_target )
                            }
                            (variant @ PenEdgeVariant::Quadratic { .. }, BBNode::Endpoint) => {
                                let ctrl1 = *variant.try_ctrl1_mut().unwrap();
                                let mut next_variant = variant.as_cubic(ctrl1, mouse_pos);
                                *next_variant.end_mut() = mouse_pos;
                                let next_building_target = BBNode::Ctrl2;
                                ( next_variant, next_building_target )
                            }
                            state => panic!("PenTool: Input(DragStart) impossible state {state:?}."),
                        };

                        *edge.variant_mut() = next_variant;
                        PenFsm::BuildingEdge {
                            edge,
                            building_target: next_building_target,
                        }
                    }
                    _ => panic!("Unhandled."),
                }
            }

            Input(DragMove {
                world: world_pos, ..
            }) => match &state {
                PenFsm::BuildingEdge {
                    mut edge,
                    building_target,
                } => {
                    let coordinate_pos = edge.world_to_coordinate_space(world, *world_pos);

                    let next_variant = match (*edge.variant(), building_target) {
                        (PenEdgeVariant::Quadratic { .. }, BBNode::Ctrl1) => {
                            edge.variant().as_quadratic(coordinate_pos)
                        }
                        (PenEdgeVariant::Quadratic { .. }, BBNode::Ctrl2) => {
                            if let Some(pos) = edge.variant_mut().try_ctrl1_mut() {
                                *pos = coordinate_pos;
                            }
                            if let Some(v) = edge.variant().try_get_inverse_ctrl_pos() {
                                edge.variant().as_quadratic(v)
                            } else {
                                *edge.variant()
                            }
                        }
                        (PenEdgeVariant::Cubic { ctrl2, .. }, BBNode::Ctrl1) => {
                            edge.variant().as_cubic(coordinate_pos, ctrl2)
                        }
                        (PenEdgeVariant::Cubic { ctrl1, .. }, BBNode::Ctrl2) => {
                            if let Some(pos) = edge.variant_mut().try_ctrl2_mut() {
                                *pos = coordinate_pos;
                            }
                            if let Some(v) = edge.variant().try_get_inverse_ctrl_pos() {
                                edge.variant().as_cubic(ctrl1, v)
                            } else {
                                *edge.variant()
                            }
                        }
                        (variant, build_target) => panic!("PenTool: Input(DragMove) impossible (variant, building_target) - ({variant:?}, {build_target:?})"),
                    };

                    let next_edge = match edge {
                        PenEdge2::Local(target, _) => PenEdge2::Local(target, next_variant),
                        PenEdge2::World(target, _) => PenEdge2::World(target, next_variant),
                        PenEdge2::Screen(target, _) => PenEdge2::Screen(target, next_variant),
                    };
                            
                    next_edge.draw(world);
                    PenFsm::BuildingEdge {
                        edge: next_edge,
                        building_target: *building_target,
                    }
                }
                state => panic!("PenTool: Input(DragMove) impossible state {state:?}."),
            },

            Input(DragEnd {
                world: world_pos, ..
            }) => match &state {
                PenFsm::BuildingEdge {
                    mut edge,
                    building_target,
                } => {
                    let coord_pos = edge.world_to_coordinate_space(world, *world_pos);

                    match (edge.variant_mut(), building_target) {
                        (variant @ PenEdgeVariant::Quadratic { .. }, BBNode::Ctrl1) => {
                            PenFsm::BuildingEdge {
                                edge,
                                building_target: BBNode::Endpoint,
                            }
                        }
                        (variant @ PenEdgeVariant::Quadratic { .. }, BBNode::Ctrl2) => {
                            let end_pos = *variant.end_mut();
                            let next_ctrl1_pos = variant.try_get_inverse_ctrl_pos().unwrap();
                            let next_edge = PenEdgeVariant::Quadratic {
                                start: end_pos,
                                start_node: None,
                                ctrl1: next_ctrl1_pos,
                                end: next_ctrl1_pos,
                                end_node: None,
                            };

                            let (target, cmds) = edge.get_commands(world, *world_pos);
                            responder.push_internal(MultiCmd::new(cmds));

                            PenFsm::AwaitingAddedEdge {
                                target,
                                next_edge: PenEdge2::Local(target, next_edge),
                            }
                        }
                        (variant @ PenEdgeVariant::Cubic { .. }, BBNode::Ctrl2) => {
                            let end_pos = *variant.end_mut();
                            let next_ctrl1_pos = variant.try_get_inverse_ctrl_pos().unwrap();
                            let next_edge = PenEdgeVariant::Quadratic {
                                start: end_pos,
                                start_node: None,
                                ctrl1: next_ctrl1_pos,
                                end: next_ctrl1_pos,
                                end_node: None,
                            };

                            let (target, cmds) = edge.get_commands(world, *world_pos);
                            responder.push_internal(MultiCmd::new(cmds));

                            PenFsm::AwaitingAddedEdge {
                                target,
                                next_edge: PenEdge2::Local(target, next_edge),
                            }
                        }
                        // (variant @ PenEdgeVariant::Quadratic { .. }, BBNode::Endpoint) => {
                        //     *variant.end_mut() = coord_pos;
                        //
                        // }
                        // (variant @ PenEdgeVariant::Cubic { .. }, BBNode::Endpoint) => {
                        //     *variant.end_mut() = coord_pos;
                        //
                        //     let end_pos = *variant.end_mut();
                        //     let next_ctrl1_pos = variant.try_get_inverse_ctrl_pos().unwrap();
                        //     let next_edge = PenEdgeVariant::Quadratic {
                        //         start: end_pos,
                        //         start_node: None,
                        //         ctrl1: next_ctrl1_pos,
                        //         end: next_ctrl1_pos,
                        //         end_node: None,
                        //     };
                        //
                        //     let (target, cmds) = edge.get_commands(world, *world_pos);
                        //     responder.push_internal(MultiCmd::new(cmds));
                        //
                        //     PenFsm::AwaitingAddedEdge {
                        //         target,
                        //         next_edge: PenEdge2::Local(target, next_edge),
                        //     }
                        // }
                        (variant, target) => {
                            panic!("PenTool: Input(DragEnd) while BuildingEdge - impossible variant {variant:?} and target {target:?}.")
                        }
                    }
                }
                state => panic!("PenTool: Input(DragMove) impossible state {state:?}."),
            },

            Input(Keyboard { key, .. }) => match key {
                KeyCode::Escape => {
                    let pen_res = world.resource::<PenResource>();
                    let cursor_node_e = pen_res.cursor_node.unwrap();
                    let cursor_line = pen_res.cursor_line.unwrap();
                    PenTool::update_cursor_node_style(
                        world,
                        cursor_node_e,
                        CursorNodeStyle::Hidden,
                        Vec2::ZERO,
                    );
                    *world.get_mut::<Visibility>(cursor_line).unwrap() = Visibility::Hidden;

                    PenFsm::Default
                }
                KeyCode::B => state,
                _ => state,
            },
            _ => state,
        };

        world.resource_mut::<PenResource>().state = next_state;
    }

    fn handle_effects(world: &mut World, event: &EffectMsg) {
        use EffectMsg::*;

        let state = world.resource_mut::<PenResource>().state.clone();

        #[allow(clippy::single_match)]
        match (event, state) {
            (
                EdgeAdded { target, idx },
                PenFsm::AwaitingAddedEdge {
                    target: curr_target,
                    mut next_edge,
                },
            ) => {
                if *target == curr_target {
                    let graph = world.bbid_get::<VectorGraph>(*target);
                    let added_edge = *graph.0.edge(*idx).unwrap();

                    let start_node = next_edge.variant_mut().start_node_mut();
                    *start_node = Some(added_edge.end_idx());

                    let mut res = world.resource_mut::<PenResource>();
                    res.state = PenFsm::BuildingEdge { edge: next_edge, building_target: BBNode::Endpoint };

                    next_edge.draw(world);
                }
            }
            _ => (),
        }
    }
}
