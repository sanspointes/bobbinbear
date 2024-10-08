use bevy::{math::Vec3A, prelude::*, render::primitives::Aabb, sprite::Mesh2dHandle};

use crate::editor::EditorSet;

#[derive(Component, Default, Debug, Reflect, Copy, Clone)]
/// Gets the global coords AABB of an entity
#[reflect(Component)]
pub enum GlobalBounds2D {
    #[default]
    NeedsCalculate,
    Calculated(Rect),
}

impl GlobalBounds2D {
    pub fn from_global_vec3as(vertices: Vec<Vec3A>) -> Self {
        let mut min = Vec3A::MAX;
        let mut max = Vec3A::MIN;
        for vert in vertices {
            min = vert.min(min);
            max = vert.max(max);
        }
        Self::Calculated(Rect::new(min.x, min.y, max.x, max.y))
    }

    pub fn reset_on_entity(world: &mut World, entity: Entity) {
        if let Some(mut bounds) = world.get_mut::<GlobalBounds2D>(entity) {
            *bounds = GlobalBounds2D::NeedsCalculate;
        }
    }
}

/// This plugin simply copies the Bevy `Aabb` component to a custom Aabb2D component
/// that wraps `Rect` and provides methods to intersect with it.
pub struct Bounds2DPlugin;
impl Plugin for Bounds2DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            // Occurs after `CalculateBounds` system set
            sys_update_global_bounds_2d.in_set(EditorSet::PostPlugins),
        )
        .register_type::<GlobalBounds2D>();
    }
}

#[allow(clippy::type_complexity)]
pub fn sys_update_global_bounds_2d(
    r_meshes: Res<Assets<Mesh>>,
    mut param_set: ParamSet<(
        // Query for changes to global bounds
        Query<(Entity, &GlobalBounds2D)>,
        // Query for all GlobalBounds2D entities.
        Query<(
            &Mesh2dHandle,
            &GlobalTransform,
            &mut GlobalBounds2D,
            &mut Aabb,
        )>,
    )>,
    mut to_update_que: Local<Vec<Entity>>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_update_global_bounds_2d").entered();
    // Get changes / additions of NeedsCalculate and store in a que
    let to_update: Vec<Entity> = {
        param_set
            .p0()
            .iter()
            .filter_map(|(e, global_bounds)| match global_bounds {
                GlobalBounds2D::NeedsCalculate => Some(e),
                GlobalBounds2D::Calculated(_) => None,
            })
            .collect()
    };

    #[cfg(feature = "debug_bounds")]
    if !to_update.is_empty() {
        debug!(
            "sys_update_global_bounds_2d: Found new entities to update bounds on {to_update:?}."
        );
    }

    to_update_que.extend(to_update);

    let mut next_to_update_que = Vec::<Entity>::with_capacity(to_update_que.len());
    let mut q_calculatable = param_set.p1();

    for entity in &to_update_que {
        if let Ok((mesh_handle, global_transform, mut global_bounds, mut aabb)) =
            q_calculatable.get_mut(*entity)
        {
            let Some(mesh) = r_meshes.get(&mesh_handle.0) else {
                next_to_update_que.push(*entity); // Try again next frame
                #[cfg(feature = "debug_bounds")]
                debug!(
                    "sys_update_global_bounds_2d: Could not get mesh to calculate global bounds on {entity:?}."
                );
                continue;
            };

            let Some(verts) = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .and_then(|attr| attr.as_float3())
            else {
                warn!("sys_update_global_bounds_2d: No position attribute on mesh {entity:?}.");
                continue;
            };
            let mut min = Vec3::MAX;
            let mut max = Vec3::MIN;
            let global_matrix = global_transform.compute_matrix();
            let verts: Vec<_> = verts
                .iter()
                .map(|vert_float3| {
                    let p = Vec3A::from(*vert_float3);
                    min.x = min.x.min(p.x);
                    min.y = min.y.min(p.y);
                    min.z = min.z.min(p.z);
                    max.x = max.x.max(p.x);
                    max.y = max.y.max(p.y);
                    max.z = max.z.max(p.z);
                    global_matrix.transform_point3a(p)
                })
                .collect();

            let new_bounds = GlobalBounds2D::from_global_vec3as(verts);
            #[cfg(feature = "debug_bounds")]
            debug!("sys_update_global_bounds_2d: Calculated bounds {new_bounds:?} on {entity:?}.");
            *global_bounds = new_bounds;
            *aabb = Aabb::from_min_max(min, max);
        }
    }
    *to_update_que = next_to_update_que;
}
