use core::panic;

use bevy::{
    app::{App, Plugin},
    core::Name,
    ecs::{
        component::Component,
        query::With,
        reflect::ReflectComponent,
        system::Resource,
        world::{FromWorld, World},
    },
    input::{keyboard::KeyCode, ButtonState},
    log::warn,
    math::{Vec2Swizzles, Vec3Swizzles},
    reflect::Reflect,
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
        selected::{
            raycast::{SelectableHit, SelectableRaycaster},
            Selectable,
        },
        undoredo::UndoRedoApi,
    },
    tools::pen::{actions::PenToolActions, utils::spawn_child_endpoint},
    views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM},
};

mod actions;
mod resource;
mod utils;

use self::{
    resource::PenToolPreview,
    utils::{
        build_default_vector_graphic, get_position_of_edge_at_t_value, get_t_value_of_edge_hit,
        split_edge_at_t_value,
    },
};
use self::{resource::PenToolResource, utils::get_new_vector_graphic_material};

use super::{input::InputMessage, BobbinCursor};

pub struct PenToolPlugin;

impl Plugin for PenToolPlugin {
    fn build(&self, app: &mut App) {
        let res = PenToolResource::from_world(app.world_mut());
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
    warn!("Pen tool: {state:?}");
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
                    if let Ok(t_value) = get_t_value_of_edge_hit(world, *e, data) {
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
            let mut actions = PenToolActions::new(world);
            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            let top = hits.top();

            match top {
                Some(SelectableHit(_endpoint_e, endpoint_uid, ObjectType::VectorEndpoint, _)) => {
                    actions
                        .set_building_from_endpoint_tag(world, *endpoint_uid)
                        .unwrap();
                    actions.finish(world).unwrap();

                    PenTool::BuildingEdge
                }
                Some(SelectableHit(edge_e, edge_uid, ObjectType::VectorEdge, data)) => {
                    let t_value = get_t_value_of_edge_hit(world, *edge_e, data).unwrap();
                    let endpoint_uid = actions.split_edge(world, *edge_uid, t_value).unwrap();
                    actions
                        .changeset_scope(world, |_, commands| {
                            commands.entity(endpoint_uid).insert(VectorEndpointVM);
                        })
                        .unwrap();
                    actions
                        .set_building_from_endpoint_tag(world, endpoint_uid)
                        .unwrap();
                    actions.finish(world).unwrap();

                    PenTool::BuildingEdge
                }
                Some(_) => PenTool::Default,
                None => {
                    let vector_graphic = world
                        .query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>()
                        .get_single(world)
                        .copied();

                    let mut actions = PenToolActions::new(world);
                    let endpoint_uid = actions.changeset_scope(world, |world, commands| {
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
                                InternalObject,
                                VectorEndpointVM,
                            ))
                            .set_parent(vector_graphic)
                            .uid()
                    });
                    actions
                        .set_building_from_endpoint_tag(world, endpoint_uid.unwrap())
                        .unwrap();

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
                    if let Ok(t_value) = get_t_value_of_edge_hit(world, *e, data) {
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
            let mut actions = PenToolActions::new(world);

            let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
            match hits.top() {
                Some(SelectableHit(_, endpoint_uid, ObjectType::VectorEndpoint, _)) => {
                    let (edge_uid, maybe_new_endpoint_uid) = actions
                        .spawn_edge_to_endpoint(world, *endpoint_uid)
                        .unwrap();
                    actions
                        .set_building_from_endpoint_tag(world, *endpoint_uid)
                        .unwrap();
                    actions
                        .changeset_scope(world, |_, commands| {
                            commands.entity(edge_uid).insert(VectorEdgeVM);
                            if let Some(new_endpoint_uid) = maybe_new_endpoint_uid {
                                commands.entity(new_endpoint_uid).insert(VectorEndpointVM);
                            }
                        })
                        .unwrap();
                }
                Some(SelectableHit(edge_e, edge_uid, ObjectType::VectorEdge, data)) => {
                    let t_value = get_t_value_of_edge_hit(world, *edge_e, data).unwrap();
                    let endpoint_uid = actions.split_edge(world, *edge_uid, t_value).unwrap();
                    actions
                        .changeset_scope(world, |_, commands| {
                            commands.entity(endpoint_uid).insert(VectorEndpointVM);
                        })
                        .unwrap();
                    let (edge_uid, _) =
                        actions.spawn_edge_to_endpoint(world, endpoint_uid).unwrap();
                    actions
                        .set_building_from_endpoint_tag(world, endpoint_uid)
                        .unwrap();
                    actions
                        .changeset_scope(world, |_, commands| {
                            commands.entity(edge_uid).insert(VectorEdgeVM);
                        })
                        .unwrap();
                }
                _ => {
                    let endpoint_uid = actions.spawn_new_endpoint(world, *world_pos).unwrap();
                    actions
                        .changeset_scope(world, |_, commands| {
                            commands.entity(endpoint_uid).insert(VectorEndpointVM);
                        })
                        .unwrap();
                    let (edge_uid, _) =
                        actions.spawn_edge_to_endpoint(world, endpoint_uid).unwrap();
                    actions
                        .set_building_from_endpoint_tag(world, endpoint_uid)
                        .unwrap();
                    actions
                        .changeset_scope(world, |_, commands| {
                            commands.entity(edge_uid).insert(VectorEdgeVM);
                        })
                        .unwrap();
                }
            };

            actions.finish(world).unwrap();

            PenTool::BuildingEdge
        }
        (
            PenTool::BuildingEdge,
            InputMessage::Keyboard {
                pressed: ButtonState::Pressed,
                key: KeyCode::Escape,
                ..
            },
        ) => {
            let mut actions = PenToolActions::new(world);
            actions.clear_building_from_endpoint_tag(world).unwrap();
            actions.finish(world).unwrap();
            PenToolResource::resource_scope(world, |world, r| {
                r.preview.show_only_endpoint_0(world);
            });
            PenTool::Default
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
