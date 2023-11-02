use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    tess::path::{Event, Path as TessPath},
    Path, ShapeBundle, Stroke,
};

use crate::{
    components::{
        bbid::BBId,
        scene::{BBObject, BBPathEvent, BBIndex},
    },
    msgs::cmds::inspect_cmd::InspectingTag,
    plugins::{inspect_plugin::InspectArtifact, selection_plugin::SelectableBundle},
};

use super::InspectState;

pub struct InspectVectorPlugin;

impl Plugin for InspectVectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(InspectState::InspectVector),
            sys_handle_enter_inspect_vector,
        )
        .add_systems(
            OnExit(InspectState::InspectVector),
            sys_handle_exit_inspect_vector,
        );
    }
}

///
fn sys_handle_enter_inspect_vector(
    mut commands: Commands,
    q_inspected_vector: Query<(Entity, &BBId, &Path), (With<BBObject>, With<InspectingTag>)>,
) {
    let (entity, bbid, path) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    info!("sys_handle_exit_inspect_vector: Inspecting {:?}", bbid);

    commands.entity(entity).with_children(|builder| {
        for (i, seg) in path.0.iter().enumerate() {
            let path_seg_bbid = BBId::default();
            let mut path_seg_builder = builder.spawn((
                BBPathEvent::from(seg),
                Stroke::new(Color::BLACK, 2.),
                InspectArtifact(*bbid),
                BBIndex(i),
                path_seg_bbid,
                SelectableBundle::default(),
            ));

            let mut pb = TessPath::builder();

            // TODO: Add nodes
            match seg {
                Event::Line { from, to } => {
                    path_seg_builder.insert(Name::from("Line"));
                    pb.begin(from);
                    pb.line_to(to);
                    pb.end(false);
                }
                Event::Quadratic { from, ctrl, to } => {
                    path_seg_builder.insert(Name::from("Quadratic"));
                    pb.begin(from);
                    pb.quadratic_bezier_to(ctrl, to);
                    pb.end(false);
                }
                Event::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    path_seg_builder.insert(Name::from("Cubic"));
                    pb.begin(from);
                    pb.cubic_bezier_to(ctrl1, ctrl2, to);
                    pb.end(false);
                }
                Event::Begin { at: _ } => {
                    path_seg_builder.insert(Name::from("Begin"));
                }
                Event::End { last, first, close } => {
                    path_seg_builder.insert(Name::from("Close"));
                    if close {
                        pb.begin(last);
                        pb.line_to(first);
                        pb.end(false);
                    }
                }
            }

            let seg_path = pb.build();
            path_seg_builder.insert(ShapeBundle {
                path: Path(seg_path),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    ..Default::default()
                },
                ..Default::default()
            });
            println!("Inspecting entity with {seg:?}");
        }
    });
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

// fn sys_handle_path_change(
//     q_changed_path:
// ) {
//
// }
