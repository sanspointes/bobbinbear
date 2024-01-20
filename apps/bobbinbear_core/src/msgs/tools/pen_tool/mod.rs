mod pen_edge;

use bb_vector_network::prelude::BBNodeIndex;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_debug_text_overlay::screen_print;
use lyon_tessellation::StrokeOptions;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBIndex, BBNode},
    },
    constants::Z_INDEX_BB_NODE,
    msgs::{
        cmds::{
            add_remove_edge_cmd::{AddRemoveEdgeCmd, AddRemoveEdgeNode},
            add_remove_object_cmd::AddObjectCmd,
            inspect_cmd::{InspectCmd, InspectingTag},
            Cmd, MultiCmd,
        },
        effect::EffectMsg,
        tools::pen_tool::pen_edge::PenEdgeVariant,
        MsgQue,
    },
    plugins::{
        bounds_2d_plugin::GlobalBounds2D,
        inspect_plugin::InspectArtifact,
        screen_space_root_plugin::ScreenSpaceRoot,
        selection_plugin::{get_raycast_hits_selectable, SelectableBundle},
        vector_graph_plugin::{Fill, Stroke, VectorGraph},
    },
    shared::{CachedMeshes, WorldUtils},
    utils::{vector::BBObjectVectorBundle, coordinates::WorldToLocal},
};

use self::pen_edge::PenEdge2;

use super::{InputMessage, ToolHandler, ToolHandlerMessage};

pub use pen_edge::sys_setup_pen_resource;

#[derive(Resource, Debug, Clone, Default)]
pub enum PenFsm {
    #[default]
    Default,
    /// When Pen tool is constructing a line from a given node/pos to the cursor.
    BuildingEdge(PenEdge2),
    /// When the pen tool has added an edge and is expecting to receive the EdgeAdded event to
    /// update its own internal state.
    AwaitingAddedEdge { target: BBId },
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
                let target_world_matrix =
                    target.map(|bbid| world.bbid_get::<GlobalTransform>(bbid).compute_matrix());

                let hit_node = {
                    hit.as_ref()
                        .and_then(|(e, _)| world.get::<BBIndex>(*e))
                        .map(|idx| BBNodeIndex(idx.0))
                };

                println!("Click - target: {target:?}");

                match state {
                    PenFsm::Default => match (target, hit_node) {
                        (Some(target), Some(node_idx)) => PenFsm::BuildingEdge(PenEdge2::new_line_from_node(world, target, node_idx)),
                        (Some(target), None) => {
                            PenFsm::BuildingEdge(PenEdge2::World(Some(target), PenEdgeVariant::Line { start: *world_pos, start_node: None, end: *world_pos, end_node: None }))
                        },
                        (None, None) => {
                            PenFsm::BuildingEdge(PenEdge2::World(None, PenEdgeVariant::Line { start: *world_pos, start_node: None, end: *world_pos, end_node: None }))
                        }
                        (None, Some(_)) => panic!("PenTool: Input(PointerClick) impossible state. Can't have no target but reference BBNodeIndex."),
                    },
                    PenFsm::BuildingEdge(edge) => {
                        let mut cmds: Vec<Box<dyn Cmd>> = Vec::new();
                        // Unwrap or create BBGraph to add the edge to.
                        let ( target, world_matrix ) = edge.target().map(|target| (target, world.bbid_get::<GlobalTransform>(target).compute_matrix())).unwrap_or_else(|| {
                            let bbid = BBId::default();
                            let transform = Transform {
                                translation: Vec3::new(world_pos.x, world_pos.y, 0.),
                                ..Default::default()
                            };
                            let world_matrix = transform.compute_matrix();

                            let cmd = AddObjectCmd::from_bundle(
                                world,
                                None,
                                (
                                    Name::from("Box"),
                                    bbid,
                                    BBObjectVectorBundle::default().with_transform(transform),
                                    GlobalBounds2D::default(),
                                    SelectableBundle::default(),
                                    Fill::color(Color::rgb_u8(50, 50, 50)),
                                    Stroke {
                                        color: Color::rgb(0.7, 0.7, 0.7),
                                        options: StrokeOptions::default(),
                                    },
                                ),
                            )
                            .unwrap();
                            cmds.push(Box::new(cmd));
                            cmds.push(Box::new(InspectCmd::inspect(bbid)));
                            (bbid, world_matrix)
                        });

                        let ss_root = ScreenSpaceRoot::get_from_world(world);
                        let cmd = match edge.to_local_with_matrix(target, ss_root, &world_matrix).variant() {
                            PenEdgeVariant::Line { start, start_node, end, end_node } => {
                                let cmd = AddRemoveEdgeCmd::new_add_line(target, AddRemoveEdgeNode::from_idx_or_local_pos(*start_node, *start), AddRemoveEdgeNode::from_idx_or_local_pos(*end_node, *end));
                                Box::new(cmd)
                            }
                            PenEdgeVariant::Quadratic { start, start_node, ctrl1, end, end_node } => {
                                let cmd = AddRemoveEdgeCmd::new_add_quadratic(target, AddRemoveEdgeNode::from_idx_or_local_pos(*start_node, *start), *ctrl1, AddRemoveEdgeNode::from_idx_or_local_pos(*end_node, *end));
                                Box::new(cmd)
                            }
                            PenEdgeVariant::Cubic { start, start_node, ctrl1, ctrl2, end, end_node } => {
                                let cmd = AddRemoveEdgeCmd::new_add_cubic(target, AddRemoveEdgeNode::from_idx_or_local_pos(*start_node, *start), *ctrl1, *ctrl2, AddRemoveEdgeNode::from_idx_or_local_pos(*end_node, *end));
                                Box::new(cmd)
                            }
                        };
                        cmds.push(cmd);

                        responder.push_internal(MultiCmd::new(cmds));

                        PenFsm::AwaitingAddedEdge { target }
                    }
                    pen_fsm => panic!("PenTool: Unhandled state {pen_fsm:?}."),
                }
            }
            Input(PointerMove { screen, world: world_pos, .. }) => {
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
                    PenFsm::BuildingEdge(edge) => {
                        let target = edge
                            .target()
                            .or(hit.map(|e| world.get::<InspectArtifact>(e).unwrap().0));
                        let hit_idx = hit.map(|e| BBNodeIndex(world.get::<BBIndex>(e).unwrap().0));

                        let next_edge = match target {
                            Some(target) => {
                                edge.to_local(world, target)
                            }
                            None => {
                                edge.to_world(world)
                            }
                        };

                        let next_edge = match next_edge {
                            PenEdge2::Local(target, mut edge) => {
                                let world_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();
                                *edge.end_node_mut() = hit_idx;
                                *edge.end_mut() = world_pos.world_to_local(&world_matrix);
                                println!("Updated edge {edge:?}");
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

                        PenFsm::BuildingEdge(next_edge)
                    }
                }
            }
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
                },
            ) => {
                if *target == curr_target {
                    let graph = world.bbid_get::<VectorGraph>(*target);
                    let end_idx = graph.0.edge(*idx).unwrap().end_idx();
                    let pos = graph.0.node(end_idx).unwrap().position();

                    let mut res = world.resource_mut::<PenResource>();
                    res.state = PenFsm::BuildingEdge(PenEdge2::Local(
                        *target,
                        PenEdgeVariant::Line {
                            start: pos,
                            start_node: Some(end_idx),
                            end: pos + 0.1,
                            end_node: None,
                        },
                    ));
                }
            }
            _ => (),
        }
    }
}
