
use bb_vector_network::prelude::{BBGraph, BBNodeIndex};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_debug_text_overlay::screen_print;
use lyon_tessellation::StrokeOptions;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBIndex, BBNode, VectorGraphDirty},
    },
    constants::Z_INDEX_BB_NODE,
    msgs::{
        cmds::{
            add_remove_edge_cmd::{AddRemoveEdgeCmd, AddRemoveEdgeNode},
            add_remove_object_cmd::AddObjectCmd,
            Cmd, CmdMsg, MultiCmd,
        },
        effect::EffectMsg,
        MsgQue,
    },
    plugins::{
        bounds_2d_plugin::GlobalBounds2D,
        inspect_plugin::InspectArtifact,
        screen_space_root_plugin::ScreenSpaceRoot,
        selection_plugin::{get_raycast_hits_selectable, SelectableBundle},
        vector_graph_plugin::{Fill, VectorGraph, Stroke},
    },
    shared::{CachedMeshes, WorldUtils},
    utils::{vector::BBObjectVectorBundle, coordinates::LocalToWorld},
};

use super::{InputMessage, ToolHandler, ToolHandlerMessage};

#[derive(Resource, Debug, Clone, Default)]
pub enum PenFsm {
    #[default]
    Default,
    /// When Pen tool is constructing a line from a given node/pos to the cursor.
    LineEdgeFrom {
        target: Option<BBId>,
        start_node: AddRemoveEdgeNode,
    },
    /// When the pen tool has added an edge and is expecting to receive the EdgeAdded event to
    /// update its own internal state.
    AwaitingAddedEdge { target: BBId },
}

#[derive(Resource, Debug, Clone, Default)]
pub struct PenResource {
    state: PenFsm,
    cursor_node: Option<Entity>,
    cursor_line: Option<Entity>,
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
                let hit_node = {
                    let bb_node_idx = hit
                        .as_ref()
                        .and_then(|(e, _)| world.get::<BBIndex>(*e))
                        .map(|idx| BBNodeIndex(idx.0));
                    match bb_node_idx {
                        Some(idx) => AddRemoveEdgeNode::Existing(idx),
                        None => AddRemoveEdgeNode::New(*world_pos),
                    }
                };
                let target = hit
                    .and_then(|(e, _)| world.get::<InspectArtifact>(e).cloned())
                    .and_then(|a| world.try_bbid(a.0))
                    .and_then(|e| world.get::<BBId>(e));

                println!("Click - target: {target:?}");

                match state {
                    PenFsm::Default => PenFsm::LineEdgeFrom {
                        start_node: hit_node,
                        target: target.copied(),
                    },
                    PenFsm::LineEdgeFrom { start_node, target } => {
                        let mut cmds: Vec<Box<dyn Cmd>> = Vec::new();

                        let target = target.unwrap_or_else(|| {
                            let bbid = BBId::default();
                            let cmd = AddObjectCmd::from_bundle(
                                world,
                                None,
                                (
                                    Name::from("Box"),
                                    bbid,
                                    BBObjectVectorBundle::default().with_transform(Transform {
                                        translation: Vec3::new(world_pos.x, world_pos.y, 0.),
                                        ..Default::default()
                                    }),
                                    GlobalBounds2D::default(),
                                    SelectableBundle::default(),
                                    Fill::color(Color::rgb_u8(50, 50, 50)),
                                ),
                            )
                            .unwrap();
                            cmds.push(Box::new(cmd));

                            bbid
                        });
                        let cmd = AddRemoveEdgeCmd::new_add_line(target, start_node, hit_node);
                        cmds.push(Box::new(cmd));

                        responder.push_internal(CmdMsg::from(MultiCmd::new(cmds)));
                        PenFsm::Default
                    }
                    pen_fsm => panic!("PenTool: Unhandled state {pen_fsm:?}."),
                }
            }
            Input(PointerMove { screen, .. }) => {
                let hit = get_raycast_hits_selectable(world)
                    .first()
                    .cloned()
                    .and_then(|(e, _)| match world.has_component::<BBNode>(e) {
                        true => Some(e),
                        false => None,
                    });

                let cursor_world = world.resource::<PenResource>().cursor_node.unwrap();
                let end_pos = match hit {
                    Some(e) => {
                        PenTool::update_cursor_node_style(
                            world,
                            cursor_world,
                            CursorNodeStyle::Hidden,
                            *screen,
                        );
                        world.get::<Transform>(e).unwrap().translation.xy()
                    }
                    None => {
                        PenTool::update_cursor_node_style(
                            world,
                            cursor_world,
                            CursorNodeStyle::Endpoint,
                            *screen,
                        );
                        *screen
                    }
                };

                let cursor_line = world.resource::<PenResource>().cursor_line.unwrap();

                match state {
                    PenFsm::Default | PenFsm::AwaitingAddedEdge { .. } => {
                        *world.get_mut::<Visibility>(cursor_line).unwrap() = Visibility::Hidden
                    }
                    PenFsm::LineEdgeFrom { target, start_node } => {
                        println!("target: {target:?}");
                        let start_pos = match (target, start_node) {
                            (Some(target), AddRemoveEdgeNode::Existing(start_idx)) => {
                                let global_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();
                                let graph = &world.bbid_get::<VectorGraph>(target).0;
                                let start_pos = graph.node(start_idx).unwrap().position();
                                println!("Graph pos: {start_pos}");
                                start_pos.local_to_world(&global_matrix)
                            }
                            (Some(target), AddRemoveEdgeNode::New(start_pos)) => {
                                start_pos
                            }
                            (None, AddRemoveEdgeNode::New(start_pos)) => {
                                start_pos
                            }
                            state => panic!("line from edge can't be in {state:?}"),
                        };
                        println!("Start pos {start_pos}");

                        let end_pos = end_pos - start_pos;

                        println!("End diff {end_pos}");

                        let mut g = BBGraph::new();
                        g.line(Vec2::ZERO, end_pos);

                        let mut graph = world.get_mut::<VectorGraph>(cursor_line).unwrap();
                        *graph = VectorGraph(g);

                        let mut transform = world.get_mut::<Transform>(cursor_line).unwrap();
                        transform.translation.x = start_pos.x;
                        transform.translation.y = start_pos.y;

                        let mut visibility = world.get_mut::<Visibility>(cursor_line).unwrap();
                        *visibility = Visibility::Visible;
                    }
                }

                state
            }
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
                    let end_node = graph.0.edge(*idx).unwrap().end_idx();

                    let mut res = world.resource_mut::<PenResource>();
                    res.state = PenFsm::LineEdgeFrom {
                        target: Some(*target),
                        start_node: AddRemoveEdgeNode::Existing(end_node),
                    }
                }
            }
            _ => (),
        }
    }
}

impl PenResource {}

pub fn sys_setup_pen_resource(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    res_vector: ResMut<CachedMeshes>,
    mut res_pen: ResMut<PenResource>,
    q_ss_root: Query<Entity, With<ScreenSpaceRoot>>,
) {
    let material = materials.add(ColorMaterial::default());
    let mesh = res_vector.endpoint_node.as_ref().unwrap().clone();

    let ss_root = q_ss_root.single();
    let e = commands
        .spawn((
            Name::from("PenLine".to_string()),
            MaterialMesh2dBundle {
                material,
                mesh,
                visibility: Visibility::Hidden,
                ..Default::default()
            }))
        .set_parent(ss_root)
        .id();

    res_pen.cursor_node = Some(e);

    let e = commands
        .spawn((
            Name::from("PenLine".to_string()),
            VectorGraph::default(),
            VectorGraphDirty::default(),
            Mesh2dHandle::default(),
            Handle::<ColorMaterial>::default(),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Hidden,
            ViewVisibility::default(),
            InheritedVisibility::default(),
            Stroke {
                color: Color::rgb(0.7, 0.7, 0.7),
                options: StrokeOptions::default(),
            }
        ))
        .set_parent(ss_root)
        .id();

    res_pen.cursor_line = Some(e);
}
