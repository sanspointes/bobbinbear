use bevy::{
    app::{App, Plugin},
    asset::Assets,
    core::Name,
    ecs::{
        component::Component,
        query::With,
        reflect::ReflectComponent,
        system::Resource,
        world::{FromWorld, World},
    },
    log::warn,
    math::{Vec2Swizzles, Vec3Swizzles},
    reflect::Reflect,
    render::mesh::Mesh,
    sprite::Mesh2dHandle,
    transform::components::GlobalTransform,
};
use bevy_spts_changeset::{builder::Changeset, commands_ext::WorldChangesetExt};
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::components::{EdgeVariant, Endpoint};

use crate::{
    api::scene::SceneApi,
    ecs::{
        InternalObject, ObjectBundle, ObjectType, Position, ProxiedPosition,
        ProxiedPositionStrategy,
    },
    plugins::{
        effect::Effect,
        model_view::View,
        selected::{raycast::SelectableRaycaster, Selectable},
        undoredo::UndoRedoApi,
    },
    utils::mesh::get_intersection_triangle_attribute_data,
    views::{
        vector_edge::{VectorEdgeVM, ATTRIBUTE_EDGE_T},
        vector_endpoint::VectorEndpointVM,
    },
};

mod resource;
mod utils;

use self::utils::{build_default_vector_graphic, split_edge_at_t_value};
use self::{
    resource::PenToolResource,
    utils::{
        build_next_endpoint_and_edge, get_current_building_prev_endpoint,
        get_current_building_vector_object, get_new_vector_graphic_material,
        BuildEndpointAndEdgeOptions, BuildEndpointAndEdgeTarget,
    },
};

use super::input::InputMessage;

pub struct PenToolPlugin;

impl Plugin for PenToolPlugin {
    fn build(&self, app: &mut App) {
        let pen_tool_resource = PenToolResource::from_world(&mut app.world);
        app.insert_resource(pen_tool_resource);
        app.insert_resource(PenTool::default());
    }
}

#[derive(Component, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
pub struct PenToolBuildingVectorObjectTag;

#[derive(Component, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
pub struct PenToolBuildingFromEndpointTag;

#[derive(Resource, Default, Debug, Clone)]
pub enum PenTool {
    #[default]
    Default,

    HoveringEdge(Uid),

    BuildingEdge,
    BuildingEdgeHoveringEndpoint(Uid),
}

fn handle_pen_tool_event(
    world: &mut World,
    _effects: &mut [Effect],
    event: &InputMessage,
    state: PenTool,
) -> PenTool {
    match (&state, event) {
        (PenTool::Default, InputMessage::PointerMove { world_pos, screen_pos, .. }) => {
            PenToolResource::resource_scope(world, |world, res| {
                res.preview.show_only_endpoint_0(world);
                res.preview.set_endpoint_0_world_pos(world, *world_pos);
            });

            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top_if_object_type(ObjectType::VectorEdge);
            match top {
                Some(hit) => PenTool::HoveringEdge(*hit.uid()),
                None => PenTool::Default,
            }
        }

        (PenTool::Default, InputMessage::PointerClick { world_pos, .. }) => {
            let vector_graphic = world
                .query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>()
                .get_single(world)
                .copied();

            let changeset = Changeset::scoped_commands(world, |world, commands| {
                let vector_graphic = {
                    if let Ok(vector_graphic) = vector_graphic {
                        vector_graphic
                    } else {
                        let material = get_new_vector_graphic_material(world);
                        build_default_vector_graphic(commands, material)
                    }
                };

                commands
                    .spawn((
                        Name::from("Endpoint"),
                        ObjectBundle::new(ObjectType::VectorEndpoint).with_position(world_pos.xy()),
                        Endpoint::default(),
                        VectorEndpointVM,
                        InternalObject,
                        PenToolBuildingFromEndpointTag,
                    ))
                    .set_parent(vector_graphic)
                    .uid();

                SceneApi::build_inspect_changeset(world, vector_graphic, commands);
            });
            UndoRedoApi::execute(world, changeset).unwrap();

            // Show the preview edge
            PenToolResource::resource_scope(world, |world, res| {
                res.preview.show_all(world);
                res.preview.set_endpoint_0_world_pos(world, *world_pos);
                res.preview
                    .set_endpoint_1_world_pos(world, *world_pos + 0.01);
            });

            PenTool::BuildingEdge
        }

        // (
        //     PenTool::Default,
        //     InputMessage::DragStart {
        //         world_pos,
        //         world_start_pos,
        //         ..
        //     },
        // ) => {
        //     PenToolResource::resource_scope(world, |world, res| {
        //         res.preview
        //             .set_endpoint_0_world_pos(world, *world_start_pos);
        //         res.preview.update_to_quadratic(world, *world_pos);
        //         res.preview.set_endpoint_1_world_pos(world, *world_pos);
        //     });
        //
        //     let changeset = Changeset::scoped_commands(world, |world, commands| {
        //         let vector_graphic = world
        //             .query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>()
        //             .get_single(world)
        //             .copied();
        //
        //         let vector_graphic = {
        //             if let Ok(vector_graphic) = vector_graphic {
        //                 vector_graphic
        //             } else {
        //                 let material = get_new_vector_graphic_material(world);
        //                 build_default_vector_graphic(commands, material)
        //             }
        //         };
        //
        //         commands
        //             .spawn((
        //                 Name::from("Endpoint"),
        //                 ObjectBundle::new(ObjectType::VectorEndpoint).with_position(world_pos.xy()),
        //                 Endpoint::default(),
        //                 VectorEndpointVM,
        //                 InternalObject,
        //                 PenToolBuildingFromEndpointTag,
        //             ))
        //             .set_parent(vector_graphic)
        //             .uid();
        //
        //         SceneApi::build_inspect_changeset(world, vector_graphic, commands);
        //     });
        //
        //     UndoRedoApi::execute(world, changeset).unwrap();
        //
        //     PenTool::BuildingCtrl1
        // }
        //
        // (PenTool::BuildingCtrl1, InputMessage::DragMove { world_pos, .. }) => {
        //     PenToolResource::resource_scope(world, |world, res| {
        //         res.preview.update_to_quadratic(world, *world_pos);
        //         res.preview.show_all(world);
        //         res.preview.set_endpoint_1_world_pos(world, *world_pos);
        //     });
        //
        //     PenTool::BuildingCtrl1
        // }
        //
        // (PenTool::BuildingCtrl1, InputMessage::DragEnd { world_pos, .. }) => {
        //     PenToolResource::resource_scope(world, |world, res| {
        //         res.preview.update_to_quadratic(world, *world_pos);
        //         res.preview.show_all(world);
        //         res.preview.set_endpoint_1_world_pos(world, *world_pos);
        //     });
        //
        //     PenTool::BuildingEdge
        // }
        (PenTool::BuildingEdge, InputMessage::PointerMove { world_pos, screen_pos, .. }) => {
            PenToolResource::resource_scope(world, |world, res| {
                let mut q_building_endpoint =
                    world.query_filtered::<&Uid, With<PenToolBuildingFromEndpointTag>>();
                let from_endpoint = *q_building_endpoint.single(world);
                let from_endpoint_pos = *world
                    .get::<Position>(from_endpoint.entity(world).unwrap())
                    .unwrap();
                res.preview
                    .set_endpoint_0_world_pos(world, from_endpoint_pos.0);
            });

            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top_if_object_type(ObjectType::VectorEndpoint);
            if let Some(hit) = top {
                let world_pos = world
                    .get::<GlobalTransform>(hit.uid().entity(world).unwrap())
                    .unwrap()
                    .translation();

                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, world_pos.xy());
                });

                PenTool::BuildingEdgeHoveringEndpoint(*hit.uid())
            } else {
                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, *world_pos);
                });

                PenTool::BuildingEdge
            }
        }

        (PenTool::BuildingEdge {}, InputMessage::PointerClick { world_pos, .. }) => {
            let parent_vector_graphic = get_current_building_vector_object(world).unwrap();
            let from_endpoint = get_current_building_prev_endpoint(world).unwrap();
            let from_endpoint_pos = *world
                .get::<Position>(from_endpoint.entity(world).unwrap())
                .unwrap();

            let mut builder = world.changeset();
            build_next_endpoint_and_edge(
                &mut builder,
                &BuildEndpointAndEdgeOptions {
                    parent_uid: parent_vector_graphic,
                    from_endpoint,
                    edge_variant: EdgeVariant::Line,
                },
                &BuildEndpointAndEdgeTarget::NewEndpoint {
                    world_pos: *world_pos,
                },
            );
            let changeset = builder.build();
            UndoRedoApi::execute(world, changeset).expect("Error building next edge changeset.");

            PenToolResource::resource_scope(world, |world, res| {
                res.preview
                    .set_endpoint_0_world_pos(world, from_endpoint_pos.0);
            });

            PenTool::BuildingEdge
        }

        (
            PenTool::BuildingEdgeHoveringEndpoint(_),
            InputMessage::PointerMove {
                world_pos,
                screen_pos,
                ..
            },
        ) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top_if_object_type(ObjectType::VectorEndpoint);

            if let Some(top) = top {
                let world_pos = world
                    .get::<GlobalTransform>(top.uid().entity(world).unwrap())
                    .unwrap()
                    .translation();

                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, world_pos.xy());
                });

                PenTool::BuildingEdgeHoveringEndpoint(*top.uid())
            } else {
                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, *world_pos);
                });

                PenTool::BuildingEdge
            }
        }

        (
            PenTool::BuildingEdgeHoveringEndpoint(hovered_endpoint),
            InputMessage::PointerClick {
                world_pos,
                screen_pos,
                ..
            },
        ) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let Some(top) = hits.top_if_object_type(ObjectType::VectorEndpoint) else {
                return PenTool::BuildingEdgeHoveringEndpoint(*hovered_endpoint);
            };

            let hit_endpoint_uid = *top.uid();
            let endpoint = *world.get::<Endpoint>(top.entity()).unwrap();

            let from_endpoint = get_current_building_prev_endpoint(world).unwrap();
            let parent_vector_graphic = get_current_building_vector_object(world).unwrap();

            let mut builder = world.changeset();
            let next_state = match (endpoint.prev_edge_entity(), endpoint.next_edge_entity()) {
                (Some(_), Some(_)) => {
                    let (_, endpoint_uid) = build_next_endpoint_and_edge(
                        &mut builder,
                        &BuildEndpointAndEdgeOptions {
                            parent_uid: parent_vector_graphic,
                            from_endpoint,
                            edge_variant: EdgeVariant::Line,
                        },
                        &BuildEndpointAndEdgeTarget::NewEndpoint {
                            world_pos: *world_pos,
                        },
                    );
                    builder.entity(endpoint_uid).insert(ProxiedPosition::new(
                        hit_endpoint_uid,
                        ProxiedPositionStrategy::Local,
                    ));
                    PenTool::BuildingEdgeHoveringEndpoint(*hovered_endpoint)
                }
                (_, Some(_)) | (None, None) => {
                    build_next_endpoint_and_edge(
                        &mut builder,
                        &BuildEndpointAndEdgeOptions {
                            parent_uid: parent_vector_graphic,
                            from_endpoint,
                            edge_variant: EdgeVariant::Line,
                        },
                        &BuildEndpointAndEdgeTarget::ExistingLinkPrevious(hit_endpoint_uid),
                    );
                    PenTool::Default
                }
                (Some(_), _) => {
                    build_next_endpoint_and_edge(
                        &mut builder,
                        &BuildEndpointAndEdgeOptions {
                            parent_uid: parent_vector_graphic,
                            from_endpoint,
                            edge_variant: EdgeVariant::Line,
                        },
                        &BuildEndpointAndEdgeTarget::ExistingLinkNext(hit_endpoint_uid),
                    );
                    PenTool::Default
                }
            };
            let changeset = builder.build();
            UndoRedoApi::execute(world, changeset).unwrap();
            next_state
        }

        (PenTool::HoveringEdge(uid), InputMessage::PointerMove { screen_pos, .. }) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            if let Some(top) = hits.top_if_object_type(ObjectType::VectorEdge) {
                PenTool::HoveringEdge(*top.uid())
            } else {
                PenTool::Default
            }
        }
        (PenTool::HoveringEdge(uid), InputMessage::PointerClick { screen_pos, .. }) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            if let Some(top) = hits.top() {
                let handle = world.get::<Mesh2dHandle>(top.entity()).unwrap();
                let mesh = world
                    .resource::<Assets<Mesh>>()
                    .get(handle.0.clone_weak())
                    .unwrap();
                let result = get_intersection_triangle_attribute_data(
                    mesh,
                    &top.intersection_data(),
                    ATTRIBUTE_EDGE_T.id,
                );
                let edge_entity = world.get::<View<VectorEdgeVM>>(top.entity()).unwrap();

                if let Ok(crate::utils::mesh::TriangleIntersectionAttributeData::Float32(t_value)) =
                    result
                {
                    let mut changeset = world.changeset();
                    match split_edge_at_t_value(
                        world,
                        &mut changeset,
                        edge_entity.model().entity(),
                        t_value,
                    ) {
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
        }
        (state, ev) => {
            warn!("PenTool: Unhandled state/ev\n\tstate: {state:?}\n\tev: {ev:?}");
            state.clone()
        }
    }
}

pub fn handle_pen_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut [Effect],
) -> Result<(), anyhow::Error> {
    let mut state = world.resource::<PenTool>().clone();

    for event in events {
        state = handle_pen_tool_event(world, _effects, event, state);
    }

    *world.resource_mut::<PenTool>() = state;

    Ok(())
}
