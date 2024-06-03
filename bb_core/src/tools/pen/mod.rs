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
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, Mesh2dHandle},
    transform::components::GlobalTransform,
};
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::{
    components::{EdgeVariant, Endpoint, VectorGraphic, VectorGraphicPathStorage},
    lyon_components::{FillOptions, StrokeOptions},
    material::{FillColor, StrokeColor, VectorGraphicMaterial},
    prelude::VectorGraphicChangesetExt,
};

use crate::{
    api::scene::SceneApi,
    ecs::{
        InternalObject, ObjectBundle, ObjectType, Position, ProxiedPosition,
        ProxiedPositionStrategy,
    },
    plugins::{
        effect::Effect,
        model_view::View,
        selected::{
            raycast::{SelectableHit, SelectableHitsWorldExt},
            ProxiedSelectable,
        },
        undoredo::UndoRedoApi,
    },
    utils::mesh::get_intersection_triangle_attribute_data,
    views::{
        vector_edge::{VectorEdgeVM, ATTRIBUTE_EDGE_T},
        vector_endpoint::VectorEndpointVM,
    },
};

mod utils;

use self::utils::{split_edge_at_t_value, PenToolResource};

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
        (PenTool::Default, InputMessage::PointerMove { world_pos, .. }) => {
            PenToolResource::resource_scope(world, |world, res| {
                res.preview.show_only_endpoint_0(world);
                res.preview.set_endpoint_0_world_pos(world, *world_pos);
            });
            let top = world.selectable_hits().top();
            match top {
                Some(SelectableHit { uid, ty, .. }) => {
                    if matches!(*ty, crate::ecs::ObjectType::VectorEdge) {
                        PenTool::HoveringEdge(*uid)
                    } else {
                        PenTool::Default
                    }
                }
                _ => PenTool::Default,
            }
        }

        (PenTool::Default, InputMessage::PointerClick { world_pos, .. }) => {
            let mut materials = world.resource_mut::<Assets<VectorGraphicMaterial>>();
            let material = materials.add(VectorGraphicMaterial::default());

            let vector_graphic = world
                .query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>()
                .get_single(world)
                .copied();

            let mut builder = world.changeset();
            let vector_graphic = {
                if let Ok(vector_graphic) = vector_graphic {
                    vector_graphic
                } else {
                    let vector_graphic = builder
                        .spawn((
                            Name::from("Shape"),
                            ObjectBundle::new(ObjectType::Vector),
                            VectorGraphic::default(),
                            VectorGraphicPathStorage::default(),
                            StrokeOptions::default().with_line_width(5.),
                            StrokeColor(Color::BLACK),
                            FillOptions::default(),
                            FillColor(Color::GRAY.with_a(0.5)),
                            material,
                            PenToolBuildingVectorObjectTag,
                        ))
                        .uid();
                    vector_graphic
                }
            };

            builder
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

            let mut changeset = builder.build();
            changeset.extend(SceneApi::build_inspect_changeset(world, vector_graphic));

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

        (PenTool::BuildingEdge, InputMessage::PointerMove { world_pos, .. }) => {
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

            let top = world.selectable_hits().top();
            let hovering_endpoint = match top {
                Some(SelectableHit { uid, ty, .. }) => {
                    if matches!(ty, ObjectType::VectorEndpoint) {
                        Some(*uid)
                    } else {
                        None
                    }
                }
                None => None,
            };

            if let Some(hovering_endpoint) = hovering_endpoint {
                let world_pos = world
                    .get::<GlobalTransform>(hovering_endpoint.entity(world).unwrap())
                    .unwrap()
                    .translation();

                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, world_pos.xy());
                });

                PenTool::BuildingEdgeHoveringEndpoint(hovering_endpoint)
            } else {
                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, *world_pos);
                });

                PenTool::BuildingEdge
            }
        }

        (PenTool::BuildingEdge {}, InputMessage::PointerClick { world_pos, .. }) => {
            let mut q_building_endpoint =
                world.query_filtered::<&Uid, With<PenToolBuildingFromEndpointTag>>();

            let from_endpoint = *q_building_endpoint.single(world);
            let from_endpoint_pos = *world
                .get::<Position>(from_endpoint.entity(world).unwrap())
                .unwrap();

            let mut q_building_vector_object =
                world.query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>();
            let vector_object = *q_building_vector_object.single(world);

            let mut builder = world.changeset();
            builder
                .entity(from_endpoint)
                .remove::<PenToolBuildingFromEndpointTag>();
            let e1 = builder
                .spawn((
                    Name::from("Endpoint"),
                    ObjectBundle::new(ObjectType::VectorEndpoint).with_position(world_pos.xy()),
                    Endpoint::default(),
                    VectorEndpointVM,
                    InternalObject,
                    PenToolBuildingFromEndpointTag,
                ))
                .set_parent(vector_object)
                .uid();

            builder
                .spawn_edge(EdgeVariant::Line, from_endpoint, e1)
                .insert((
                    Name::from("Edge"),
                    ObjectBundle::new(ObjectType::VectorEdge),
                    InternalObject,
                ))
                // .insert(ObjectBundle::new(ObjectType::VectorSegment))
                .set_parent(vector_object);

            let changeset = builder.build();
            UndoRedoApi::execute(world, changeset).expect("Error building next edge changeset.");

            PenToolResource::resource_scope(world, |world, res| {
                res.preview
                    .set_endpoint_0_world_pos(world, from_endpoint_pos.0);
            });

            PenTool::BuildingEdge
        }

        (PenTool::BuildingEdgeHoveringEndpoint(_), InputMessage::PointerMove { world_pos, .. }) => {
            let top = world.selectable_hits().top();
            let hovering_endpoint = match top {
                Some(SelectableHit { uid, ty, .. }) => {
                    if matches!(ty, ObjectType::VectorEndpoint) {
                        Some(*uid)
                    } else {
                        None
                    }
                }
                None => None,
            };

            if let Some(hovering_endpoint) = hovering_endpoint {
                let world_pos = world
                    .get::<GlobalTransform>(hovering_endpoint.entity(world).unwrap())
                    .unwrap()
                    .translation();

                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, world_pos.xy());
                });

                PenTool::BuildingEdgeHoveringEndpoint(hovering_endpoint)
            } else {
                PenToolResource::resource_scope(world, |world, res| {
                    res.preview.set_endpoint_1_world_pos(world, *world_pos);
                });

                PenTool::BuildingEdge
            }
        }

        (
            PenTool::BuildingEdgeHoveringEndpoint(hovered_endpoint),
            InputMessage::PointerClick { world_pos, .. },
        ) => {
            let top = world
                .selectable_hits()
                .top()
                .filter(|v| matches!(v.ty, ObjectType::VectorEndpoint))
                .map(|v| {
                    world
                        .get::<ProxiedSelectable>(v.uid.entity(world).unwrap())
                        .unwrap()
                        .target()
                })
                .and_then(|uid| {
                    world
                        .get::<Endpoint>(uid.entity(world).unwrap())
                        .copied()
                        .map(|ep| (*uid, ep))
                });

            warn!("endpoint: {top:?}");

            let Some((target_endpoint, endpoint)) = top else {
                return PenTool::BuildingEdgeHoveringEndpoint(*hovered_endpoint);
            };

            let mut q_building_endpoint =
                world.query_filtered::<&Uid, With<PenToolBuildingFromEndpointTag>>();
            let from_endpoint = *q_building_endpoint.single(world);
            let mut q_building_vector_object =
                world.query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>();
            let vector_object = *q_building_vector_object.single(world);

            let mut builder = world.changeset();
            builder
                .entity(from_endpoint)
                .remove::<PenToolBuildingFromEndpointTag>();

            match (endpoint.prev_edge_entity(), endpoint.next_edge_entity()) {
                (Some(_), Some(_)) => {
                    let e1 = builder
                        .spawn((
                            Name::from("Endpoint"),
                            ObjectBundle::new(ObjectType::VectorEndpoint)
                                .with_position(world_pos.xy()),
                            ProxiedPosition::new(target_endpoint, ProxiedPositionStrategy::Local),
                            Endpoint::default(),
                            VectorEndpointVM,
                            InternalObject,
                            PenToolBuildingFromEndpointTag,
                        ))
                        .set_parent(vector_object)
                        .uid();

                    builder
                        .spawn_edge(EdgeVariant::Line, from_endpoint, e1)
                        .insert((
                            Name::from("Edge"),
                            ObjectBundle::new(ObjectType::VectorEdge),
                            InternalObject,
                        ))
                        // .insert(ObjectBundle::new(ObjectType::VectorSegment))
                        .set_parent(vector_object);

                    let changeset = builder.build();
                    UndoRedoApi::execute(world, changeset).unwrap();
                    PenTool::BuildingEdgeHoveringEndpoint(*hovered_endpoint)
                }
                (_, Some(_)) | (None, None) => {
                    builder
                        .spawn_edge(EdgeVariant::Line, from_endpoint, target_endpoint)
                        .insert((
                            Name::from("Edge"),
                            ObjectBundle::new(ObjectType::VectorEdge),
                            InternalObject,
                        ))
                        .set_parent(vector_object);

                    let changeset = builder.build();
                    UndoRedoApi::execute(world, changeset).unwrap();
                    PenTool::Default
                }
                (Some(_), _) => {
                    builder
                        .spawn_edge(EdgeVariant::Line, target_endpoint, from_endpoint)
                        .insert((
                            Name::from("Edge"),
                            ObjectBundle::new(ObjectType::VectorEdge),
                            InternalObject,
                        ))
                        .set_parent(vector_object);

                    let changeset = builder.build();
                    UndoRedoApi::execute(world, changeset).unwrap();
                    PenTool::BuildingEdgeHoveringEndpoint(*hovered_endpoint)
                }
            }
        }

        (PenTool::HoveringEdge(uid), InputMessage::PointerMove { .. }) => {
            let top = world.selectable_hits().top();
            let is_hovering_edge =
                top.map_or(false, |top| matches!(top.ty, ObjectType::VectorEdge));
            if !is_hovering_edge {
                PenTool::Default
            } else {
                PenTool::HoveringEdge(*uid)
            }
        }
        (PenTool::HoveringEdge(uid), InputMessage::PointerClick { .. }) => {
            let top = world.selectable_hits().top();
            if let Some(top) = top {
                let handle = world.get::<Mesh2dHandle>(top.entity).unwrap();
                let mesh = world
                    .resource::<Assets<Mesh>>()
                    .get(handle.0.clone_weak())
                    .unwrap();
                let result =
                    get_intersection_triangle_attribute_data(mesh, &top.data, ATTRIBUTE_EDGE_T.id);
                let edge_entity = world.get::<View<VectorEdgeVM>>(top.entity).unwrap();

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
