//! This module contains code for the pen_tool on-screen representation of the edge that's about to
//! be added.
//!
//! It has some fairly complex requirements so that's why it's been modularised.
//! It has to represent an edge of a BBGraph that hasn't been created yet.
//! - The BBGraph may not have been created yet, in which case we create the bbgraph first.
//! - References to pre-existing nodes are in local space but for .

use bb_vector_network::prelude::{BBGraph, BBNodeIndex};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use lyon_tessellation::StrokeOptions;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBNode, VectorGraphDirty},
    },
    plugins::{
        screen_space_root_plugin::ScreenSpaceRoot,
        vector_graph_plugin::{Stroke, VectorGraph, Fill}, selection_plugin::SelectableBundle, bounds_2d_plugin::GlobalBounds2D,
    },
    shared::CachedMeshes,
    utils::{coordinates::{LocalToScreen, LocalToWorld, ScreenToWorld, WorldToLocal, WorldToScreen}, vector::BBObjectVectorBundle}, msgs::cmds::{Cmd, inspect_cmd::InspectCmd, add_remove_object_cmd::AddObjectCmd, add_remove_edge_cmd::{AddRemoveEdgeCmd, AddRemoveEdgeNode}},
};

use super::PenResource;

#[derive(Component)]
struct PenToolEdgeTag;
#[derive(Component)]
struct PenToolNodeTag;

pub fn sys_setup_pen_resource(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    res_vector: ResMut<CachedMeshes>,
    mut res_pen: ResMut<PenResource>,
    q_ss_root: Query<Entity, With<ScreenSpaceRoot>>,
) {
    let material = materials.add(ColorMaterial::default());
    let endpoint_mesh = res_vector.endpoint_node.as_ref().unwrap().clone();
    let control_mesh = res_vector.control_node.as_ref().unwrap().clone();

    let ss_root = q_ss_root.single();
    let e = commands
        .spawn((
            Name::from("PenNode".to_string()),
            PenToolNodeTag,
            MaterialMesh2dBundle {
                material: material.clone(),
                mesh: endpoint_mesh.clone(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
        ))
        .set_parent(ss_root)
        .id();

    res_pen.cursor_node = Some(e);

    let e = commands
        .spawn((
            PenToolEdgeTag,
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
            },
        ))
        .set_parent(ss_root)
        .id();

    res_pen.cursor_line = Some(e);

    let nodes = [
        BBNode::Endpoint,
        BBNode::Ctrl1,
        BBNode::Ctrl2,
        BBNode::Endpoint,
    ];
    let cursor_line_nodes = nodes.iter().map(|bb_node| {
        let e = commands
            .spawn((
                Name::from("PenEdgeNode"),
                *bb_node,
                PenToolNodeTag,
                MaterialMesh2dBundle {
                    material: material.clone(),
                    mesh: match bb_node {
                        BBNode::Endpoint => endpoint_mesh.clone(),
                        _ => control_mesh.clone(),
                    },
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
            ))
            .set_parent(ss_root)
            .id();

        e
    });
    res_pen.cursor_line_node = Some(cursor_line_nodes.collect());
}

#[derive(Debug, Clone, Copy)]
pub enum PenEdgeVariant {
    Line {
        start: Vec2,
        start_node: Option<BBNodeIndex>,
        end: Vec2,
        end_node: Option<BBNodeIndex>,
    },
    Quadratic {
        start: Vec2,
        start_node: Option<BBNodeIndex>,
        ctrl1: Vec2,
        end: Vec2,
        end_node: Option<BBNodeIndex>,
    },
    Cubic {
        start: Vec2,
        start_node: Option<BBNodeIndex>,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: Vec2,
        end_node: Option<BBNodeIndex>,
    },
}

impl PenEdgeVariant {
    pub fn start_mut(&mut self) -> &mut Vec2 {
        match self {
            Self::Line { start, .. }
            | Self::Quadratic { start, .. }
            | Self::Cubic { start, .. } => start,
        }
    }
    pub fn start_node_mut(&mut self) -> &mut Option<BBNodeIndex> {
        match self {
            Self::Line { start_node, .. }
            | Self::Quadratic { start_node, .. }
            | Self::Cubic { start_node, .. } => start_node,
        }
    }

    pub fn try_ctrl1_mut(&mut self) -> Option<&mut Vec2> {
        match self {
            Self::Line { .. } => None,
            Self::Quadratic { ctrl1, .. } | Self::Cubic { ctrl1, .. } => Some(ctrl1),
        }
    }

    pub fn try_ctrl2_mut(&mut self) -> Option<&mut Vec2> {
        match self {
            Self::Line { .. } | Self::Quadratic { .. } => None,
            Self::Cubic { ctrl2, .. } => Some(ctrl2),
        }
    }

    pub fn end_mut(&mut self) -> &mut Vec2 {
        match self {
            Self::Line { end, .. } | Self::Quadratic { end, .. } | Self::Cubic { end, .. } => end,
        }
    }
    pub fn end_node_mut(&mut self) -> &mut Option<BBNodeIndex> {
        match self {
            Self::Line { end_node, .. }
            | Self::Quadratic { end_node, .. }
            | Self::Cubic { end_node, .. } => end_node,
        }
    }

    pub fn as_local_to_world(&mut self, world_matrix: &Mat4) -> &mut Self {
        use PenEdgeVariant::*;
        match self {
            Line {
                start,
                start_node,
                end,
                end_node,
            } => {
                *start = start.local_to_world(world_matrix);
                *end = end.local_to_world(world_matrix);
            }
            Quadratic {
                start,
                start_node,
                ctrl1,
                end,
                end_node,
            } => {
                *start = start.local_to_world(world_matrix);
                *ctrl1 = ctrl1.local_to_world(world_matrix);
                *end = end.local_to_world(world_matrix);
            }
            Cubic {
                start,
                start_node,
                ctrl1,
                ctrl2,
                end,
                end_node,
            } => {
                *start = start.local_to_world(world_matrix);
                *ctrl1 = ctrl1.local_to_world(world_matrix);
                *ctrl2 = ctrl1.local_to_world(world_matrix);
                *end = end.local_to_world(world_matrix);
            }
        }

        self
    }

    pub fn as_world_to_local(&mut self, world_matrix: &Mat4) -> &mut Self {
        use PenEdgeVariant::*;
        match self {
            Line {
                start,
                start_node,
                end,
                end_node,
            } => {
                *start = start.world_to_local(world_matrix);
                *end = end.world_to_local(world_matrix);
            }
            Quadratic {
                start,
                start_node,
                ctrl1,
                end,
                end_node,
            } => {
                *start = start.world_to_local(world_matrix);
                *ctrl1 = ctrl1.world_to_local(world_matrix);
                *end = end.world_to_local(world_matrix);
            }
            Cubic {
                start,
                start_node,
                ctrl1,
                ctrl2,
                end,
                end_node,
            } => {
                *start = start.world_to_local(world_matrix);
                *ctrl1 = ctrl1.world_to_local(world_matrix);
                *ctrl2 = ctrl1.world_to_local(world_matrix);
                *end = end.world_to_local(world_matrix);
            }
        }
        self
    }

    pub fn as_world_to_screen(&mut self, ss_root: &ScreenSpaceRoot) -> &mut Self {
        use PenEdgeVariant::*;
        match self {
            Line {
                start,
                start_node,
                end,
                end_node,
            } => {
                *start = start.world_to_screen(ss_root);
                *end = end.world_to_screen(ss_root);
            }
            Quadratic {
                start,
                start_node,
                ctrl1,
                end,
                end_node,
            } => {
                *start = start.world_to_screen(ss_root);
                *ctrl1 = ctrl1.world_to_screen(ss_root);
                *end = end.world_to_screen(ss_root);
            }
            Cubic {
                start,
                start_node,
                ctrl1,
                ctrl2,
                end,
                end_node,
            } => {
                *start = start.world_to_screen(ss_root);
                *ctrl1 = ctrl1.world_to_screen(ss_root);
                *ctrl2 = ctrl1.world_to_screen(ss_root);
                *end = end.world_to_screen(ss_root);
            }
        }
        self
    }

    pub fn as_screen_to_world(&mut self, ss_root: &ScreenSpaceRoot) -> &mut Self {
        use PenEdgeVariant::*;
        match self {
            Line {
                start,
                start_node,
                end,
                end_node,
            } => {
                *start = start.screen_to_world(ss_root);
                *end = end.screen_to_world(ss_root);
            }
            Quadratic {
                start,
                start_node,
                ctrl1,
                end,
                end_node,
            } => {
                *start = start.screen_to_world(ss_root);
                *ctrl1 = ctrl1.screen_to_world(ss_root);
                *end = end.screen_to_world(ss_root);
            }
            Cubic {
                start,
                start_node,
                ctrl1,
                ctrl2,
                end,
                end_node,
            } => {
                *start = start.screen_to_world(ss_root);
                *ctrl1 = ctrl1.screen_to_world(ss_root);
                *ctrl2 = ctrl1.screen_to_world(ss_root);
                *end = end.screen_to_world(ss_root);
            }
        }
        self
    }

    pub fn as_quadratic(&self, ctrl1: Vec2) -> Self {
        match self {
            Self::Line {
                start,
                start_node,
                end,
                end_node,
            } => Self::Quadratic {
                start: *start,
                start_node: *start_node,
                ctrl1,
                end: *end,
                end_node: *end_node,
            },
            Self::Quadratic {
                start,
                start_node,
                end,
                end_node,
                ..
            } => Self::Quadratic {
                start: *start,
                start_node: *start_node,
                ctrl1,
                end: *end,
                end_node: *end_node,
            },
            Self::Cubic {
                start,
                start_node,
                end,
                end_node,
                ..
            } => Self::Quadratic {
                start: *start,
                start_node: *start_node,
                ctrl1,
                end: *end,
                end_node: *end_node,
            },
        }
    }

    pub fn as_cubic(&self, ctrl1: Vec2, ctrl2: Vec2) -> Self {
        match self {
            Self::Line {
                start,
                start_node,
                end,
                end_node,
            } => Self::Cubic {
                start: *start,
                start_node: *start_node,
                ctrl1,
                ctrl2,
                end: *end,
                end_node: *end_node,
            },
            Self::Quadratic {
                start,
                start_node,
                end,
                end_node,
                ..
            } => Self::Cubic {
                start: *start,
                start_node: *start_node,
                ctrl1,
                ctrl2,
                end: *end,
                end_node: *end_node,
            },
            Self::Cubic {
                start,
                start_node,
                end,
                end_node,
                ..
            } => Self::Cubic {
                start: *start,
                start_node: *start_node,
                ctrl1,
                ctrl2,
                end: *end,
                end_node: *end_node,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PenEdge2 {
    Local(BBId, PenEdgeVariant),
    World(Option<BBId>, PenEdgeVariant),
    Screen(Option<BBId>, PenEdgeVariant),
}

impl PenEdge2 {
    pub fn new_line_from_node(world: &mut World, bbid: BBId, node_idx: BBNodeIndex) -> Self {
        let g = world.bbid_get::<VectorGraph>(bbid);
        let local_pos = g.0.node(node_idx).unwrap().position();
        Self::Local(
            bbid,
            PenEdgeVariant::Line {
                start: local_pos,
                start_node: Some(node_idx),
                end: local_pos,
                end_node: None,
            },
        )
    }

    pub fn variant(&self) -> &PenEdgeVariant {
        match self {
            PenEdge2::Local(_, variant)
            | PenEdge2::World(_, variant)
            | PenEdge2::Screen(_, variant) => variant,
        }
    }

    pub fn variant_mut(&mut self) -> &mut PenEdgeVariant {
        match self {
            PenEdge2::Local(_, variant)
            | PenEdge2::World(_, variant)
            | PenEdge2::Screen(_, variant) => variant,
        }
    }

    pub fn set_start_from_world(
        &mut self,
        world_pos: Vec2,
        ss_root: &ScreenSpaceRoot,
        world_matrix: &Mat4,
    ) {
        match self {
            PenEdge2::Local(_, edge) => {
                let local_pos = world_pos.world_to_local(world_matrix);
                let start = edge.start_mut();
                *start = local_pos;
            }
            PenEdge2::Screen(_, edge) => {
                let screen_pos = world_pos.world_to_screen(ss_root);
                let start = edge.start_mut();
                *start = screen_pos;
            }
            PenEdge2::World(_, edge) => {
                let start = edge.start_mut();
                *start = world_pos;
            }
        }
    }

    pub fn set_start_from_node(
        &mut self,
        local_pos: Vec2,
        idx: BBNodeIndex,
        ss_root: &ScreenSpaceRoot,
        world_matrix: &Mat4,
    ) {
        match self {
            PenEdge2::Local(_, edge) => {
                *edge.start_mut() = local_pos;
                *edge.start_node_mut() = Some(idx);
            }
            PenEdge2::Screen(_, edge) => {
                let screen_pos = local_pos.local_to_screen(world_matrix, ss_root);
                *edge.start_mut() = screen_pos;
                *edge.start_node_mut() = Some(idx);
            }
            PenEdge2::World(_, edge) => {
                let world_pos = local_pos.local_to_world(world_matrix);
                *edge.start_mut() = world_pos;
                *edge.start_node_mut() = Some(idx);
            }
        }
    }

    pub fn set_end_from_world(
        &mut self,
        world_pos: Vec2,
        ss_root: &ScreenSpaceRoot,
        world_matrix: &Mat4,
    ) {
        match self {
            PenEdge2::Local(_, edge) => {
                let screen_pos = world_pos.world_to_local(world_matrix);
                let end = edge.end_mut();
                *end = screen_pos;
            }
            PenEdge2::Screen(_, edge) => {
                let screen_pos = world_pos.world_to_screen(ss_root);
                let end = edge.end_mut();
                *end = screen_pos;
            }
            PenEdge2::World(_, edge) => {
                let end = edge.end_mut();
                *end = world_pos;
            }
        }
    }

    pub fn set_end_from_node(
        &mut self,
        local_pos: Vec2,
        idx: BBNodeIndex,
        ss_root: &ScreenSpaceRoot,
        world_matrix: &Mat4,
    ) {
        match self {
            PenEdge2::Local(_, edge) => {
                *edge.end_mut() = local_pos;
                *edge.end_node_mut() = Some(idx);
            }
            PenEdge2::Screen(_, edge) => {
                let screen_pos = local_pos.local_to_screen(world_matrix, ss_root);
                *edge.end_mut() = screen_pos;
                *edge.end_node_mut() = Some(idx);
            }
            PenEdge2::World(_, edge) => {
                let world_pos = local_pos.local_to_world(world_matrix);
                *edge.end_mut() = world_pos;
                *edge.end_node_mut() = Some(idx);
            }
        }
    }

    /// Returns the currently targetted entity that we're trying to apply the temporary edge to.
    pub fn target(&self) -> Option<BBId> {
        use PenEdge2::*;
        match self {
            Local(target, _) => Some(*target),
            World(target, _) | Screen(target, _) => *target,
        }
    }

    /// Creates a copy of this temporary edge in screen coordinate space.  Useful for rendering to
    /// the screen (see `draw`).
    pub fn to_screen(self, world: &mut World) -> Self {
        match self {
            PenEdge2::Local(target, mut variant) => {
                let global_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();
                variant.as_local_to_world(&global_matrix);
                let ss_root = ScreenSpaceRoot::get_from_world(world);
                variant.as_world_to_screen(ss_root);
                PenEdge2::Screen(Some(target), variant)
            }
            PenEdge2::World(target, mut variant) => {
                let ss_root = ScreenSpaceRoot::get_from_world(world);
                variant.as_world_to_screen(ss_root);
                PenEdge2::Screen(target, variant)
            }
            screen => screen,
        }
    }

    /// Creates a copy of this temporary edge in world coordinate space.
    pub fn to_world(self, world: &mut World) -> Self {
        match self {
            PenEdge2::Screen(target, mut variant) => {
                let ss_root = ScreenSpaceRoot::get_from_world(world);
                variant.as_screen_to_world(&ss_root);
                PenEdge2::World(target, variant)
            }
            PenEdge2::Local(target, mut variant) => {
                let global_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();
                variant.as_local_to_world(&global_matrix);
                PenEdge2::World(Some(target), variant)
            }
            world => world,
        }
    }

    /// Creates a copy of this temporary edge in local coordinates of an entity using the
    /// screenspace root and the global matrix.
    pub fn to_local_with_matrix(
        self,
        target: BBId,
        ss_root: &ScreenSpaceRoot,
        global_matrix: &Mat4,
    ) -> Self {
        match self {
            PenEdge2::Screen(_, mut variant) => {
                variant.as_screen_to_world(ss_root);
                variant.as_world_to_local(global_matrix);
                PenEdge2::Local(target, variant)
            }
            PenEdge2::World(_, mut variant) => {
                variant.as_world_to_screen(ss_root);
                PenEdge2::Local(target, variant)
            }
            local => local,
        }
    }
    /// Creates a copy of this temporary edge in local coordinates of a given entity.
    pub fn to_local(self, world: &mut World, target: BBId) -> Self {
        let global_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();
        let ss_root = ScreenSpaceRoot::get_from_world(world);
        self.to_local_with_matrix(target, ss_root, &global_matrix)
    }

    /// Converts a world coordinate into the same coordinate space as this temporary edge.
    pub fn world_to_coordinate_space(&self, world: &mut World, world_pos: Vec2) -> Vec2 {
        match self {
            PenEdge2::Local(target, _) => {
                let world_matrix =
                    world.bbid_get::<GlobalTransform>(*target).compute_matrix();
                world_pos.world_to_local(&world_matrix)
            }
            PenEdge2::World(_, _) => world_pos,
            PenEdge2::Screen(_, _) => {
                world_pos.world_to_screen(ScreenSpaceRoot::get_from_world(world))
            }
        }
    }

    /// Gets the commands to apply this temporary edge to the world.
    pub fn get_commands(&self, world: &mut World, world_pos: Vec2) -> (BBId, Vec<Box<dyn Cmd>>) {
        let mut cmds: Vec<Box<dyn Cmd>> = Vec::new();
        // Unwrap or create BBGraph to add the edge to.
        let (target, world_matrix) = self
            .target()
            .map(|target| {
                (
                    target,
                    world.bbid_get::<GlobalTransform>(target).compute_matrix(),
                )
            })
            .unwrap_or_else(|| {
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
        let cmd = match self
            .to_local_with_matrix(target, ss_root, &world_matrix)
            .variant()
        {
            PenEdgeVariant::Line {
                start,
                start_node,
                end,
                end_node,
            } => {
                let cmd = AddRemoveEdgeCmd::new_add_line(
                    target,
                    AddRemoveEdgeNode::from_idx_or_local_pos(*start_node, *start),
                    AddRemoveEdgeNode::from_idx_or_local_pos(*end_node, *end),
                );
                Box::new(cmd)
            }
            PenEdgeVariant::Quadratic {
                start,
                start_node,
                ctrl1,
                end,
                end_node,
            } => {
                let cmd = AddRemoveEdgeCmd::new_add_quadratic(
                    target,
                    AddRemoveEdgeNode::from_idx_or_local_pos(*start_node, *start),
                    *ctrl1,
                    AddRemoveEdgeNode::from_idx_or_local_pos(*end_node, *end),
                );
                Box::new(cmd)
            }
            PenEdgeVariant::Cubic {
                start,
                start_node,
                ctrl1,
                ctrl2,
                end,
                end_node,
            } => {
                let cmd = AddRemoveEdgeCmd::new_add_cubic(
                    target,
                    AddRemoveEdgeNode::from_idx_or_local_pos(*start_node, *start),
                    *ctrl1,
                    *ctrl2,
                    AddRemoveEdgeNode::from_idx_or_local_pos(*end_node, *end),
                );
                Box::new(cmd)
            }
        };
        cmds.push(cmd);

        (target, cmds)
    }

    pub fn draw(&self, world: &mut World) {
        let screen_edge = self.to_screen(world);
        let (mut vis, mut graph, mut graph_dirty) = world
            .query_filtered::<(
                &mut Visibility,
                &mut VectorGraph,
                &mut VectorGraphDirty,
            ), With<PenToolEdgeTag>>()
            .single_mut(world);
        *vis = Visibility::Visible;

        let mut g = BBGraph::new();
        println!("PenEdge2::draw() screen_edge: {screen_edge:?}");
        match screen_edge.variant() {
            PenEdgeVariant::Line { start, end, .. } => {
                g.line(*start, *end);
            }
            PenEdgeVariant::Quadratic {
                start, ctrl1, end, ..
            } => {
                let (_, e) = g.quadratic(*start, *ctrl1, *end);
                let (_, l) = g.line_from(e.start_idx(), *ctrl1);
                g.line_from_to(e.end_idx(), l.end_idx());
            }
            PenEdgeVariant::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
                ..
            } => {
                let (_, e) = g.cubic(*start, *ctrl1, *ctrl2, *end);
                g.line_from(e.start_idx(), *ctrl1);
                g.line_from(e.end_idx(), *ctrl2);
            }
        }
        *graph = VectorGraph(g);
        *graph_dirty = VectorGraphDirty::Dirty;
    }
}
