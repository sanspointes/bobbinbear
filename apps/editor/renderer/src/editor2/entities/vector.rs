use std::{fmt::Debug, sync::Arc};

use bevy::{ecs::world::EntityMut, math::Vec3Swizzles, prelude::*, sprite::Mesh2dHandle};
use bevy_debug_text_overlay::screen_print;
use bevy_mod_raycast::RaycastMesh;
use bevy_prototype_lyon::{
    prelude::{
        tess::{
            geom::euclid::{Point2D, UnknownUnit},
            path::Path as TessPath,
        },
        *,
    },
    render::ShapeMaterial,
};
use debug_panic::debug_panic;

use crate::{
    debug_log,
    editor2::{
        camera::RaycastSelectable,
        constants::{DOC_ELEMENTS_Z_INDEX, FOCUS_RING_STROKE_WIDTH, HOVER_COLOR},
    },
};

use super::{Bounded, HoveredState, MovableTag, NeedsDelete, SelectableTag, SelectedState};

///
/// Vector Entity Resource
///

// Caches paths so they don't need to be re-calculated
#[derive(Resource)]
pub struct VectorCachedPaths {
    pub control_node: TessPath,
    pub endpoint_node: TessPath,
}
#[derive(Resource)]
pub struct VectorResource {
    pub cached_paths: VectorCachedPaths,
}

impl VectorResource {
    pub fn new() -> Self {
        let control_shape = shapes::Polygon {
            points: vec![Vec2::new(-3., -3.), Vec2::new(3., -3.), Vec2::new(3., 3.), Vec2::new(-3., 3.)],
            closed: true,
        };
        let control_node = GeometryBuilder::build_as(&control_shape).0;
        let endpoint_shape = shapes::Circle {
            radius: 5.,
            center: Vec2::ZERO,
        };
        let endpoint_node = GeometryBuilder::build_as(&endpoint_shape).0;

        Self {
            cached_paths: VectorCachedPaths {
                control_node,
                endpoint_node,
            },
        }
    }
}

#[derive(Component, Debug)]
pub struct TransientTag;

#[derive(Component, Debug)]
pub struct VectorObjectTag;

#[derive(Component, Default, PartialEq, Eq)]
pub enum VecNodeTag {
    #[default]
    Default,
    Active,
}

#[derive(Component)]
pub struct VecNodeLineTag {
    point: Entity,
    control: Entity,
}

#[derive(Component, Debug, Reflect)]
pub struct Ordered(pub usize);

#[derive(Component, Debug)]
pub struct EditableTag;

fn vec2_to_p2(v: &Vec2) -> Point2D<f32, UnknownUnit> {
    Point2D::<f32, UnknownUnit>::new(v.x, v.y)
}
fn p2_to_vec2(v: &Point2D<f32, UnknownUnit>) -> Vec2 {
    Vec2::new(v.x, v.y)
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
pub enum PathSegment {
    Begin(Vec2),
    Point(Vec2),
    Control(Vec2),
}

impl PathSegment {
    pub fn get_point(&self) -> &Vec2 {
        match self {
            Self::Begin(v) => &v,
            Self::Point(v) => &v,
            Self::Control(v) => &v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmbPath(pub Vec<PathSegment>);

impl From<TessPath> for EmbPath {
    fn from(value: TessPath) -> Self {
        let mut emb_path = Vec::<PathSegment>::new();

        for seg in &value {
            use tess::path::Event;

            match seg {
                Event::Begin { at } => emb_path.push(PathSegment::Begin(p2_to_vec2(&at))),
                Event::Line { to, .. } => emb_path.push(PathSegment::Point(p2_to_vec2(&to))),
                Event::Quadratic { to, ctrl, .. } => {
                    emb_path.push(PathSegment::Control(p2_to_vec2(&ctrl)));
                    emb_path.push(PathSegment::Point(p2_to_vec2(&to)));
                }
                Event::Cubic {
                    to, ctrl1, ctrl2, ..
                } => {
                    emb_path.push(PathSegment::Control(p2_to_vec2(&ctrl1)));
                    emb_path.push(PathSegment::Control(p2_to_vec2(&ctrl2)));
                    emb_path.push(PathSegment::Point(p2_to_vec2(&to)));
                }
                _ => {}
            }
        }
        Self(emb_path)
    }
}

#[derive(Default, Bundle, Clone)]
pub struct SelectableBundle {
    tag: SelectableTag,
    picker: RaycastMesh<RaycastSelectable>,
    hover_state: HoveredState,
    selected_state: SelectedState,
}
impl Debug for SelectableBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Mutex");
        d.field("tag", &self.tag);
        d.field("picker", &"RaycastMesh<RaycastSelectable>");
        d.field("hover_state", &self.hover_state);
        d.field("selected_state", &self.selected_state);
        d.finish_non_exhaustive()
    }
}

trait EntityExtrasFn: Fn(&mut EntityMut) + Send + Sync + 'static {
    fn call(&self, eb: &mut EntityMut);
}

impl<F: Fn(&mut EntityMut) + Send + Sync + 'static> EntityExtrasFn for F {
    fn call(&self, eb: &mut EntityMut) {
        self(eb)
    }
}

impl std::fmt::Debug for dyn EntityExtrasFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EntityExtrasFn")
    }
}

#[derive(Debug, Clone)]
/// Spawns a bevy_prototype_lyon compatible bundle with extra fields
/// for selection, editing, etc.
///
/// * `parent`: Parent entity
/// * `name`: Name
/// * `origin`: Origin point (used for transform)
/// * `z_index`: Z-index (used for transform)
/// * `is_transient`: Transient entities are flagged with a component that they are temporary
/// * `fill`: Fill component
/// * `stroke`: Stroke component
/// * `selectable`:
/// * `path`:
/// * `is_editable`:
/// * `extra_bundles`:
pub struct VectorObjectSpawner {
    parent: Option<Entity>,
    name: Option<String>,
    origin: Vec2,
    z_index: f32,
    is_transient: bool,
    has_bounds: bool,
    fill: Option<Fill>,
    stroke: Option<Stroke>,
    selectable: Option<SelectableBundle>,
    movable: bool,
    path: TessPath,
    is_editable: bool,
    extra_bundles: Vec<Arc<dyn EntityExtrasFn>>,
    extra_nodes_fn: Vec<Arc<dyn EntityExtrasFn>>,
}

impl VectorObjectSpawner {
    pub fn new() -> Self {
        Self {
            parent: None,
            name: None,
            origin: Vec2::ZERO,
            z_index: DOC_ELEMENTS_Z_INDEX,
            is_transient: false,
            has_bounds: true,
            fill: None,
            stroke: None,
            selectable: None,
            movable: false,
            path: TessPath::new(),
            is_editable: false,
            extra_bundles: vec![],
            extra_nodes_fn: vec![],
        }
    }

    pub fn with_parent(mut self, parent: Entity) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_transient(mut self, transient: bool) -> Self {
        self.is_transient = transient;
        self
    }

    pub fn with_origin(mut self, origin: Vec2) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn with_fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    pub fn with_path(mut self, path: TessPath) -> Self {
        self.path = path;
        self
    }

    pub fn with_bounds(mut self, bounds: bool) -> Self {
        self.has_bounds = bounds;
        self
    }

    pub fn with_selectable(mut self, is_selectable: bool) -> Self {
        self.has_bounds = self.has_bounds || is_selectable;
        self.selectable = if is_selectable {
            Some(SelectableBundle::default())
        } else {
            None
        };
        self
    }

    pub fn with_movable(mut self, is_movable: bool) -> Self {
        self.has_bounds = self.has_bounds || is_movable;
        self.movable = is_movable;
        self
    }

    pub fn with_shape_editable(mut self, editable: bool) -> Self {
        self.has_bounds = self.has_bounds || editable;
        self.is_editable = editable;
        self
    }

    pub fn with_extra<F>(mut self, builder: F) -> Self
    where
        F: Fn(&mut EntityMut) + Send + Sync + 'static,
    {
        self.extra_bundles.push(Arc::new(builder));
        self
    }

    fn with_extra_wrapped(mut self, builder: Arc<dyn EntityExtrasFn>) -> Self {
        self.extra_bundles.push(builder);
        self
    }

    pub fn with_extra_for_nodes<F>(mut self, builder: F) -> Self
    where
        F: Fn(&mut EntityMut) + Send + Sync + 'static,
    {
        self.extra_nodes_fn.push(Arc::new(builder));
        self
    }

    pub fn spawn_with_world(&self, world: &mut World) -> Entity {
        let id = {
            let mut e = world.spawn_empty();
            let id = e.id();
            self.build_vector_object(&mut e);

            if let Some(parent) = self.parent {
                debug_log!("Spawning {id:?} as parent to {parent:?}");
                world.entity_mut(parent).push_children(&[id]);
            }

            id
        };

        if self.is_editable {
            let (endpoint_path, controlpoint_path) = {
                let res = world.resource::<VectorResource>();
                (
                    res.cached_paths.endpoint_node.clone(),
                    res.cached_paths.control_node.clone(),
                )
            };

            let emb_path: EmbPath = self.path.clone().into();
            let mut last_endpoint_index = 0;
            let children: Vec<_> = emb_path
                .0
                .into_iter()
                .enumerate()
                .map(|(i, path_seg)| {
                    let to = path_seg.get_point();

                    // Endpoint nodes are space 3 apar
                    match path_seg {
                        PathSegment::Control(_) | PathSegment::Begin(_) => {}
                        _ => {
                            debug_log!("Non control endpoint {i:?}");
                            last_endpoint_index = i;
                        }
                    }
                    let this_index = (last_endpoint_index * 3) + i - last_endpoint_index;
                    let mut node = Self::new_vector_node(
                        this_index,
                        to,
                        path_seg,
                        &endpoint_path,
                        &controlpoint_path,
                    );
                    for extra in self.extra_nodes_fn.iter() {
                        node = node.with_extra_wrapped(extra.clone());
                    }
                    node.with_parent(id).spawn_with_world(world)
                })
                .collect();

            world.entity_mut(id).push_children(&children);
        }

        id
    }

    fn build_vector_object(&self, builder: &mut EntityMut) -> () {
        let name_string = self.name.clone().unwrap_or("VectorObject".to_string());
        builder.insert((
            Name::from(name_string),
            Transform {
                translation: Vec3::new(self.origin.x, self.origin.y, self.z_index),
                ..Default::default()
            },
            GlobalTransform::default(),
            ComputedVisibility::default(),
            Visibility::default(),
            Handle::<ShapeMaterial>::default(),
            Mesh2dHandle::default(),
            Path(self.path.clone()),
        ));

        if self.has_bounds {
            builder.insert(Bounded::default());
        }

        if let Some(selectable) = &self.selectable {
            builder.insert(selectable.clone());
        }

        if self.movable {
            builder.insert(MovableTag);
        }

        if self.is_transient {
            builder.insert(TransientTag);
        }

        if let Some(fill) = self.fill {
            builder.insert(fill);
        }
        if let Some(stroke) = self.stroke {
            builder.insert(stroke);
        }

        for extra in self.extra_bundles.iter() {
            extra(builder);
        }
    }

    /// Generates a new control node vector object for modifying the mesh
    pub fn new_vector_node(
        index: usize,
        position: &Vec2,
        path_seg: PathSegment,
        endpoint_path: &TessPath,
        controlpoint_path: &TessPath,
    ) -> Self {
        let path = match path_seg {
            PathSegment::Control(_) => controlpoint_path,
            _ => endpoint_path,
        };
        VectorObjectSpawner::new()
            .with_name("Node".into())
            .with_path(path.clone())
            .with_fill(Fill::color(Color::WHITE))
            .with_selectable(true)
            .with_movable(true)
            .with_origin(*position)
            .with_z_index(1.)
            .with_stroke(Stroke::new(HOVER_COLOR, FOCUS_RING_STROKE_WIDTH))
            .with_extra(move |entity| {
                entity.insert((path_seg, Ordered(index), VecNodeTag::default()));
            })
    }
}

// Whenever a vector node has moved we need to update the data of the PathSegment
// associated with that entity.
pub fn handle_vec_node_moved(
    mut q_editable_vectors: Query<
        (&Transform, &mut PathSegment),
        (With<VecNodeTag>, Changed<Transform>),
    >,
) {
    for (transform, mut path_seg) in q_editable_vectors.iter_mut() {
        match path_seg.as_mut() {
            PathSegment::Begin { .. } => *path_seg = PathSegment::Begin(transform.translation.xy()),
            PathSegment::Point { .. } => *path_seg = PathSegment::Point(transform.translation.xy()),
            PathSegment::Control(_) => {
                *path_seg = PathSegment::Control(transform.translation.xy());
            }
        }
    }
}

// When a VecNodeTag's data is updated, we need to rebuild the parent VectorObjectTag's mesh.
pub fn handle_vec_object_node_updated(
    mut q_vector_objects: Query<
        (&Children, &mut Path, &mut Bounded),
        (With<VectorObjectTag>, Without<VecNodeTag>),
    >,
    q_changed_nodes: Query<
        &Parent,
        (
            With<VecNodeTag>,
            Or<(Changed<PathSegment>, Added<VecNodeTag>, Added<NeedsDelete>)>,
        ),
    >,
    q_all_nodes: Query<(Entity, &Ordered, &PathSegment), With<VecNodeTag>>,
    q_node_lines: Query<(Entity, &VecNodeLineTag, &mut Path)>, 
) {
    let changed_parents = {
        let mut changed: Vec<_> = q_changed_nodes.iter().map(|parent| parent.get()).collect();
        changed.sort();
        changed.dedup();
        changed
    };

    for parent in changed_parents {
        if let Ok((children, mut path, mut bounded)) = q_vector_objects.get_mut(parent) {
            let mut pb = tess::path::Path::builder().with_svg();
            let mut ordered_children: Vec<_> = q_all_nodes.iter_many(children).collect();
            ordered_children.sort_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap()); // Sort by ordered

            let mut control_points: (Option<Vec2>, Option<Vec2>) = (None, None);
            for (entity, _, path_seg) in ordered_children {
                match path_seg {
                    PathSegment::Begin(at) => {
                        pb.move_to(vec2_to_p2(at));
                    }
                    PathSegment::Point(to) => {
                        match (control_points.0.take(), control_points.1.take()) {
                            (Some(ctrl1), Some(ctrl2)) => {
                                let mut builder = VectorObjectSpawner::new();
                                pb.cubic_bezier_to(
                                    vec2_to_p2(&ctrl1),
                                    vec2_to_p2(&ctrl2),
                                    vec2_to_p2(to),
                                );
                            }
                            (Some(ctrl), None) => {
                                pb.quadratic_bezier_to(vec2_to_p2(&ctrl), vec2_to_p2(to));
                            }
                            (None, _) => {
                                pb.line_to(vec2_to_p2(to));
                            }
                        }
                    }
                    PathSegment::Control(p) => {
                        if control_points.0.is_none() {
                            control_points.0 = Some(p.clone());
                        } else if control_points.1.is_none() {
                            control_points.1 = Some(p.clone());
                        } else {
                            panic!("handle_vec_object_node_updated: Too many control points without a node to use it.");
                        }
                    }
                }
            }

            *path = Path(pb.build());
            *bounded = Bounded::NeedsCalculate;
        } else {
            #[cfg(debug_assertions)]
            panic!(
                "handle_vec_object_node_updated: Could not get vector object {:?} to update.",
                parent
            );
        }
    }
}

pub fn debug_vector_node_order(
    q_nodes: Query<(&Ordered, &PathSegment), With<VecNodeTag>>,
) {
    let mut ordered: Vec<_> = q_nodes.into_iter().collect();
    ordered.sort_by(|a,b| a.0.0.partial_cmp(&b.0.0).unwrap());
    for (ordered, path_seg) in ordered {
        let i = ordered.0;
        screen_print!(push, sec: 0.01, "Node {i} - {path_seg:?}");
    }
}

/// Updates the lines between control and point nodes in a vector object
///
pub fn update_vector_nodes_control_lines(
    q_nodes: Query<(&Transform, &Ordered, &PathSegment), (With<VecNodeTag>)>,
) {

}
