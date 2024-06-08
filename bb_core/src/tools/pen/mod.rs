use core::panic;

use bevy::{
    app::{App, Plugin, Update},
    asset::{Assets, Handle},
    core::Name,
    ecs::{
        component::Component,
        query::With,
        reflect::ReflectComponent,
        schedule::IntoSystemConfigs,
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
use bevy_spts_changeset::{
    builder::{Changeset, ChangesetCommands},
    commands_ext::WorldChangesetExt,
};
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::{
    components::{Edge, EdgeVariant, Endpoint},
    prelude::VectorGraphicChangesetExt,
};

use crate::{
    api::scene::SceneApi,
    ecs::{InternalObject, ObjectBundle, ObjectType, ProxiedPosition, ProxiedPositionStrategy},
    plugins::{
        effect::{Effect, EffectQue},
        model_view::View,
        selected::{
            raycast::{SelectableHit, SelectableRaycaster},
            Selectable,
        },
        undoredo::UndoRedoApi,
    },
    tools::pen::utils::build_next_endpoint,
    utils::{
        mesh::{get_intersection_triangle_attribute_data, TriangleIntersectionAttributeData},
        safe_world_ext::BBSafeWorldExt,
    },
    views::{
        vector_edge::{VectorEdgeVM, ATTRIBUTE_EDGE_T},
        vector_endpoint::VectorEndpointVM,
    },
    PosSet,
};

mod resource;
mod utils;

use self::{
    resource::PenToolPreview,
    utils::{build_default_vector_graphic, get_position_of_edge_at_t_value, split_edge_at_t_value},
};
use self::{
    resource::PenToolResource,
    utils::{
        get_current_building_prev_endpoint, get_current_building_vector_object,
        get_new_vector_graphic_material,
    },
};

use super::{input::InputMessage, BobbinCursor};

pub struct PenToolPlugin;

impl Plugin for PenToolPlugin {
    fn build(&self, app: &mut App) {
        let res = PenToolResource::from_world(&mut app.world);
        app.insert_resource(res);
        app.insert_resource(PenTool::default());
        // .add_systems(
        //     Update,
        //     sys_update_pen_tool_preview.before(PosSet::Propagate),
        // );
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
    Deactive,
    Default,

    BuildingEdge,
}

fn handle_pen_tool_event(
    world: &mut World,
    _effects: &mut EffectQue,
    event: &InputMessage,
    state: PenTool,
) -> PenTool {
    match (&state, event) {
        (
            PenTool::Default,
            InputMessage::PointerMove {
                screen_pos,
                world_pos,
                ..
            },
        ) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top();

            let hit_world_pos = match top {
                Some(SelectableHit(e, _, ObjectType::VectorEndpoint, _)) => {
                    let hit_world_pos =
                        world.get::<GlobalTransform>(*e).unwrap().translation().xy();
                    _effects.push_effect(Effect::CursorChanged(BobbinCursor::PenSameEndpoint));
                    hit_world_pos
                }
                Some(SelectableHit(e, _, ObjectType::VectorEdge, data)) => {
                    let handle = world.bb_get::<Mesh2dHandle>(*e).unwrap().0.clone_weak();
                    let mesh = world.resource::<Assets<Mesh>>().get(handle).unwrap();
                    let result =
                        get_intersection_triangle_attribute_data(mesh, data, ATTRIBUTE_EDGE_T.id);

                    if let Ok(TriangleIntersectionAttributeData::Float32(t_value)) = result {
                        _effects.push_effect(Effect::CursorChanged(BobbinCursor::PenSplitEdge));
                        let edge = world.get::<Edge>(*e).unwrap();
                        let edge_variant = world.get::<EdgeVariant>(*e).unwrap();
                        get_position_of_edge_at_t_value(world, edge, edge_variant, t_value)
                    } else {
                        *world_pos
                    }
                }
                Some(_) | None => {
                    _effects.push_effect(Effect::CursorChanged(BobbinCursor::Pen));
                    *world_pos
                }
            };

            PenToolResource::resource_scope(world, |world, r| {
                r.preview.show_only_endpoint_0(world);
                r.preview.set_endpoint_0_world_pos(world, hit_world_pos);
            });

            PenTool::Default
        }

        (
            PenTool::Default,
            InputMessage::PointerClick {
                world_pos,
                screen_pos,
                ..
            },
        ) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top();

            match top {
                Some(SelectableHit(entity, _, ObjectType::VectorEdge, data)) => {
                    let handle = world.bb_get::<Mesh2dHandle>(*entity).unwrap();
                    let mesh = world
                        .resource::<Assets<Mesh>>()
                        .get(handle.0.clone_weak())
                        .unwrap();
                    let edge_entity = world.bb_get::<View<VectorEdgeVM>>(*entity).unwrap();
                    let result =
                        get_intersection_triangle_attribute_data(mesh, data, ATTRIBUTE_EDGE_T.id);
                    if let Ok(TriangleIntersectionAttributeData::Float32(t_value)) = result {
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
                    }
                    PenTool::BuildingEdge
                }
                Some(_) => PenTool::Default,
                None => {
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
                                ObjectBundle::new(ObjectType::VectorEndpoint)
                                    .with_position(world_pos.xy()),
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
                    PenTool::BuildingEdge
                }
            }
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
        (
            PenTool::BuildingEdge,
            InputMessage::PointerMove {
                screen_pos,
                world_pos,
                ..
            },
        ) => {
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top();

            let hit_world_pos = match top {
                Some(SelectableHit(e, _, ObjectType::VectorEndpoint, _)) => {
                    let hit_world_pos =
                        world.get::<GlobalTransform>(*e).unwrap().translation().xy();
                    _effects.push_effect(Effect::CursorChanged(BobbinCursor::PenSameEndpoint));
                    hit_world_pos
                }
                Some(SelectableHit(e, _, ObjectType::VectorEdge, data)) => {
                    let handle = world.bb_get::<Mesh2dHandle>(*e).unwrap().0.clone_weak();
                    let mesh = world.resource::<Assets<Mesh>>().get(handle).unwrap();
                    let result =
                        get_intersection_triangle_attribute_data(mesh, data, ATTRIBUTE_EDGE_T.id);

                    if let Ok(TriangleIntersectionAttributeData::Float32(t_value)) = result {
                        _effects.push_effect(Effect::CursorChanged(BobbinCursor::PenSplitEdge));
                        let edge = world.get::<Edge>(*e).unwrap();
                        let edge_variant = world.get::<EdgeVariant>(*e).unwrap();
                        get_position_of_edge_at_t_value(world, edge, edge_variant, t_value)
                    } else {
                        *world_pos
                    }
                }
                Some(_) | None => {
                    _effects.push_effect(Effect::CursorChanged(BobbinCursor::Pen));
                    *world_pos
                }
            };

            PenToolResource::resource_scope(world, |world, r| {
                r.preview.show_all(world);
                let building_from_global_transform = world
                    .query_filtered::<&GlobalTransform, With<PenToolBuildingFromEndpointTag>>()
                    .get_single(world);
                if let Ok(endpoint_0_global_transform) = building_from_global_transform {
                    r.preview.set_endpoint_0_world_pos(
                        world,
                        endpoint_0_global_transform.translation().xy(),
                    );
                }
                r.preview.set_endpoint_1_world_pos(world, hit_world_pos);
            });
            PenTool::BuildingEdge
        }

        (
            PenTool::BuildingEdge {},
            InputMessage::PointerClick {
                world_pos,
                screen_pos,
                ..
            },
        ) => {
            warn!("BuildingEdge + PointerClick");
            let Some(parent_vector_graphic) = get_current_building_vector_object(world) else {
                warn!("BuildingEdge + PointerClick: Couldn't get PenToolBuildingVectorObjectTag");
                return PenTool::BuildingEdge {};
            };
            let Some(from_endpoint_uid) = get_current_building_prev_endpoint(world) else {
                warn!("BuildingEdge + PointerClick: Couldn't get PenToolBuildingFromEndpointTag");
                return PenTool::BuildingEdge {};
            };

            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);

            let mut commands = world.changeset();
            commands
                .entity(from_endpoint_uid)
                .remove::<PenToolBuildingFromEndpointTag>();

            let to_endpoint_uid = match hits.top() {
                Some(SelectableHit(entity, uid, ObjectType::VectorEndpoint, _)) => {
                    let endpoint = world.get::<Endpoint>(*entity).unwrap();

                    let needs_new_endpoint = matches!(
                        (endpoint.prev_edge_entity(), endpoint.next_edge_entity()),
                        (Some(_), Some(_))
                    );

                    if needs_new_endpoint {
                        let new_endpoint_uid =
                            build_next_endpoint(&mut commands, *world_pos, parent_vector_graphic);
                        commands
                            .entity(new_endpoint_uid)
                            .insert(ProxiedPosition::new(*uid, ProxiedPositionStrategy::Local));
                        Some(new_endpoint_uid)
                    } else {
                        commands
                            .entity(*uid)
                            .insert(PenToolBuildingFromEndpointTag);
                        Some(*uid)
                    }
                }
                Some(SelectableHit(entity, _, ObjectType::VectorEdge, data)) => {
                    let handle = world
                        .bb_get::<Mesh2dHandle>(*entity)
                        .unwrap()
                        .0
                        .clone_weak();
                    let mesh = world.resource::<Assets<Mesh>>().get(handle).unwrap();

                    let result =
                        get_intersection_triangle_attribute_data(mesh, data, ATTRIBUTE_EDGE_T.id);
                    let edge_entity = world.bb_get::<View<VectorEdgeVM>>(*entity).unwrap();

                    result
                        .ok()
                        .and_then(|v| match v {
                            TriangleIntersectionAttributeData::Float32(v) => Some(v),
                            _ => None,
                        })
                        .and_then(|t_value| {
                            split_edge_at_t_value(
                                world,
                                &mut commands,
                                edge_entity.model().entity(),
                                t_value,
                            )
                            .ok()
                        })
                        .map(|(endpoint_uid, _, _)| endpoint_uid)
                }
                _ => {
                    let endpoint_uid =
                        build_next_endpoint(&mut commands, *world_pos, parent_vector_graphic);

                    Some(endpoint_uid)
                }
            };

            let Some(to_endpoint_uid) = to_endpoint_uid else {
                return PenTool::BuildingEdge;
            };

            let maybe_to_endpoint_data = to_endpoint_uid
                .get_entity(world)
                .ok()
                .and_then(|e| world.get::<Endpoint>(e));

            let mut edge_entity_commands = match maybe_to_endpoint_data {
                Some(endpoint) => {
                    match (endpoint.prev_edge_entity(), endpoint.next_edge_entity()) {
                        (Some(_), Some(_)) => panic!("Impossible.  If both slots are full a new endpoint should have been spawned above."),
                        (None, None) | (None, Some(_)) => commands.spawn_edge(EdgeVariant::Line, from_endpoint_uid, to_endpoint_uid),
                        // When prev slot is taken but next is empty, need to reverse direction of
                        // the edge.
                        (Some(_), None) => commands.spawn_edge(EdgeVariant::Line, to_endpoint_uid, from_endpoint_uid),
                    }
                }
                None => commands.spawn_edge(EdgeVariant::Line, from_endpoint_uid, to_endpoint_uid),
            };

            edge_entity_commands
                .insert((
                    Name::from("Edge"),
                    ObjectBundle::new(ObjectType::VectorEdge),
                    InternalObject,
                ))
                // .insert(ObjectBundle::new(ObjectType::VectorSegment))
                .set_parent(parent_vector_graphic);

            let changeset = commands.build();
            UndoRedoApi::execute(world, changeset).expect("Error building next edge changeset.");

            PenTool::BuildingEdge
        }

        (state, ev) => {
            warn!("PenTool: Unhandled state/ev\n\tstate: {state:?}\n\tev: {ev:?}");
            state.clone()
        }
    }
}

pub fn activate_pen_tool(
    world: &mut World,
    _commands: &mut ChangesetCommands,
    effects: &mut EffectQue,
) {
    let mut tool = world.resource_mut::<PenTool>();
    *tool = PenTool::Default;
    effects.push_effect(Effect::CursorChanged(super::BobbinCursor::Pen))
}

pub fn deactivate_pen_tool(
    world: &mut World,
    commands: &mut ChangesetCommands,
    _effects: &mut EffectQue,
) {
    let mut tool = world.resource_mut::<PenTool>();
    *tool = PenTool::Deactive;

    PenToolResource::resource_scope(world, |world, r| {
        r.preview.hide_all(world);
    });

    if let Ok(uid) = world
        .query_filtered::<&Uid, With<PenToolBuildingFromEndpointTag>>()
        .get_single(world)
        .copied()
    {
        commands
            .entity(uid)
            .remove::<PenToolBuildingFromEndpointTag>();
    }

    if let Ok(uid) = world
        .query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>()
        .get_single(world)
        .copied()
    {
        commands
            .entity(uid)
            .remove::<PenToolBuildingFromEndpointTag>();
    }
}

pub fn handle_pen_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut EffectQue,
) -> Result<(), anyhow::Error> {
    let mut state = world.resource::<PenTool>().clone();

    for event in events {
        state = handle_pen_tool_event(world, _effects, event, state);
    }

    *world.resource_mut::<PenTool>() = state;

    Ok(())
}

pub fn sys_update_pen_tool_preview(world: &mut World) {
    PenToolPreview::refresh(world);
}
