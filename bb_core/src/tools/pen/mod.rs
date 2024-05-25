use bevy::{
    app::{App, Plugin}, asset::Assets, ecs::{system::Resource, world::World}, input::ButtonState, log::warn, render::mesh::{Indices, Mesh}, sprite::Mesh2dHandle
};
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_uid::Uid;

use crate::{
    ecs::{ObjectType, Position},
    plugins::{
        effect::Effect, model_view::View, selected::{
            raycast::{SelectableHit, SelectableHitsWorldExt},
            Hovered, Selected, SelectedApi,
        }, undoredo::UndoRedoApi
    }, utils::mesh::get_intersection_triangle_attribute_data, views::vector_edge::{VectorEdgeVM, ATTRIBUTE_EDGE_T},
};

mod utils;

use self::utils::split_edge_at_t_value;

use super::input::InputMessage;

pub struct PenToolPlugin;

impl Plugin for PenToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PenTool::default());
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub enum PenTool {
    #[default]
    Default,

    HoveringEdge(Uid),
}

pub fn handle_pen_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut [Effect],
) -> Result<(), anyhow::Error> {
    let mut state = world.resource::<PenTool>().clone();

    for event in events {
        state = match (&state, event) {
            (PenTool::Default, InputMessage::PointerMove { .. }) => {
                let top = world.selectable_hits().top();
                match top {
                    Some(SelectableHit { entity, uid, ty, data }) => {
                        if matches!(*ty, crate::ecs::ObjectType::VectorEdge) {
                            PenTool::HoveringEdge(*uid)
                        } else {
                            PenTool::Default
                        }
                    }
                    _ => PenTool::Default
                }
            }
            (PenTool::HoveringEdge(uid), InputMessage::PointerMove { .. }) => {
                let top = world.selectable_hits().top();
                let is_hovering_edge = top.map_or(false, |top| matches!(top.ty, ObjectType::VectorEdge));
                if !is_hovering_edge {
                    PenTool::Default
                } else {
                    PenTool::HoveringEdge(*uid)
                }
            },
            (PenTool::HoveringEdge(uid), InputMessage::PointerClick { .. }) => {
                let top = world.selectable_hits().top();
                if let Some(top) = top {
                    let handle = world.get::<Mesh2dHandle>(top.entity).unwrap();
                    let mesh = world.resource::<Assets<Mesh>>().get(handle.0.clone_weak()).unwrap();
                    let result = get_intersection_triangle_attribute_data(mesh, &top.data, ATTRIBUTE_EDGE_T.id);
                    let edge_entity = world.get::<View<VectorEdgeVM>>(top.entity).unwrap();

                    if let Ok(crate::utils::mesh::TriangleIntersectionAttributeData::Float32(t_value)) = result {
                        let mut changeset = world.changeset();
                        match split_edge_at_t_value(world, &mut changeset, edge_entity.model().entity(), t_value) {
                            Ok(_) => {
                                UndoRedoApi::execute(world, changeset.build()).unwrap();
                            }
                            Err(reason) => {
                                warn!("Could not split edge because {reason:?}");
                            }
                        }
                        PenTool::Default
                    } else {
                        PenTool::HoveringEdge(*uid)
                    }
                } else {
                    PenTool::HoveringEdge(*uid)
                }
            },
            (state, ev) => {
                warn!("PenTool: Unhandled state/ev\n\tstate: {state:?}\n\tev: {ev:?}");
                state.clone()
            }
        }
    }

    *world.resource_mut::<PenTool>() = state;

    Ok(())
}
