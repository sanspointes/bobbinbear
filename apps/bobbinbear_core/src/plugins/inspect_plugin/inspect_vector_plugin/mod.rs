mod utils;

use bevy::prelude::*;

use bevy_prototype_lyon::{
    prelude::{
        tess::{
            math::Point,
            path::{Event, Path as TessPath},
        },
        GeometryBuilder, Path,
    },
    shapes,
};

use crate::{
    components::{
        bbid::BBId,
        scene::BBObject,
    },
    msgs::cmds::inspect_cmd::InspectingTag,
    plugins::{
        inspect_plugin::{
            inspect_vector_plugin::utils::{
                spawn_bbnodes_of_segment, spawn_bbpathevent_of_segment,
            },
            InspectArtifact,
        },
        screen_space_root_plugin::ScreenSpaceRootTag,
    },
};

use super::InspectState;

pub struct InspectVectorPlugin;

impl Plugin for InspectVectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VectorResource::new())
            .add_systems(
                OnEnter(InspectState::InspectVector),
                sys_handle_enter_inspect_vector,
            )
            .add_systems(
                OnExit(InspectState::InspectVector),
                sys_handle_exit_inspect_vector,
            );
    }
}

type TessEvent = Event<Point, Point>;

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
            points: vec![
                Vec2::new(-3., -3.),
                Vec2::new(3., -3.),
                Vec2::new(3., 3.),
                Vec2::new(-3., 3.),
            ],
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

/// Generates all of the entities require to inspect the currently inspecting BBVector entity.
fn sys_handle_enter_inspect_vector(
    mut commands: Commands,
    res: Res<VectorResource>,
    q_inspected_vector: Query<(Entity, &BBId, &Path, &GlobalTransform), (With<BBObject>, With<InspectingTag>)>,
    q_screenspace: Query<(Entity, &ScreenSpaceRootTag)>,
) {
    let (entity, bbid, path, global_transform) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    info!("sys_handle_exit_inspect_vector: Inspecting {:?}", bbid);

    let (ss_root_entity, ss_root) = q_screenspace.single();

    for (i, seg) in path.0.iter().enumerate() {
        spawn_bbnodes_of_segment(
            &mut commands,
            &res,
            *bbid,
            global_transform,
            seg,
            ss_root_entity,
            ss_root
        );
        spawn_bbpathevent_of_segment(
            &mut commands,
            *bbid,
            seg,
            i,
            ss_root_entity,
            ss_root,
        );
    }
}

fn sys_handle_exit_inspect_vector(
    mut commands: Commands,
    q_inspected_vector: Query<(Entity, &BBId, &Path), (With<BBObject>, With<InspectingTag>)>,
    q_inspect_artifacts: Query<(Entity, &InspectArtifact)>,
) {
    let (_entity, bbid, _path) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    info!("sys_handle_exit_inspect_vector: Uinspecting {:?}", bbid);

    let to_remove = q_inspect_artifacts
        .iter()
        .filter_map(|(entity, inspect_artifact)| {
            if inspect_artifact.0 == *bbid {
                Some(entity)
            } else {
                None
            }
        });

    for e in to_remove {
        if let Some(e) = commands.get_entity(e) {
            e.despawn_recursive();
        } else {
            warn!("sys_handle_exit_inspect_vector: Attempted to despawn {e:?} but no entity found.")
        }
    }
}
