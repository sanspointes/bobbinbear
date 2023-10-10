use std::{collections::VecDeque, sync::Arc};

use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    debug_log,
    editor2::{
        entities::{
            vector::{
                Ordered, PathSegment, TransientTag, VecNodeTag, VectorObjectSpawner,
                VectorObjectTag, VectorResource,
            },
            Bounded, MovableTag, SelectableTag, SelectedState,
        },
        utils::vector::reset_vector_object_origin,
        Message,
    },
};

use super::GenericCommand;

#[derive(Debug, Clone)]
pub enum ElementOperation {
    CreateVectorObject(Arc<VectorObjectSpawner>),
    ChangeSelection {
        to_select: Vec<Entity>,
        to_deselect: Vec<Entity>,
    },
    Delete(Vec<Entity>),
    TranslateObjects(Vec<(Entity, Vec2)>),
    InsertVecNode {
        parent: Entity,
        index: i32,
        position: Vec2,
        path_seg: PathSegment,
    },
    SetActiveVecNode {
        parent: Entity,
        new_active: usize,
    },
}
pub(crate) type ElementCommandMessage = GenericCommand<ElementOperation>;

// fn insert_vector_node_after(&mut world: World, parent: Entity, pos: Vec2, insert_after: Entity) {
//
// }

pub fn handle_element_command_message(
    world: &mut World,
    message: ElementCommandMessage,
    responses: &mut VecDeque<Message>,
) {
    match message.operation {
        ElementOperation::CreateVectorObject(spawner) => {
            let mut q_transient =
                world.query_filtered::<Entity, (With<TransientTag>, With<VectorObjectTag>)>();
            let entitys_to_delete: Vec<_> = q_transient.iter(world).collect();
            for entity in entitys_to_delete {
                world.entity_mut(entity).despawn_recursive();
            }
            spawner.clone().spawn_with_world(world);
        }
        ElementOperation::ChangeSelection {
            to_select,
            to_deselect,
        } => {
            let mut q_selectable =
                world.query_filtered::<&mut SelectedState, With<SelectableTag>>();

            for entity in to_select {
                if let Ok(mut selected) = q_selectable.get_mut(world, entity) {
                    selected.set_if_neq(SelectedState::Selected);
                }
            }
            for entity in to_deselect {
                if let Ok(mut selected) = q_selectable.get_mut(world, entity) {
                    selected.set_if_neq(SelectedState::Unselected);
                }
            }
            // TODO: Return inverse operation as an undo
        }
        ElementOperation::TranslateObjects(to_translate) => {
            let mut q_transformable =
                world.query_filtered::<(&mut Transform, &mut Bounded), With<MovableTag>>();
            for (ent, new_pos) in to_translate {
                if let Ok((mut transform, mut bounded)) = q_transformable.get_mut(world, ent) {
                    transform.translation =
                        Vec3::new(new_pos.x, new_pos.y, transform.translation.z);
                    *bounded = Bounded::NeedsCalculate;
                }
            }
        }
        ElementOperation::InsertVecNode {
            parent,
            index,
            position,
            path_seg,
        } => {
            let mut sys_state: SystemState<(
                Res<VectorResource>,
                // VectorObject
                Query<&Children, With<VectorObjectTag>>,
                // Vector nodes query
                Query<(&mut Ordered, &PathSegment), With<VecNodeTag>>,
            )> = SystemState::new(world);

            let (res, q_vector_objs, mut q_all_nodes) = sys_state.get_mut(world);

            // If there's already a node at the position, incrememnt nodes to make space for it.
            if let Ok(children) = q_vector_objs.get(parent) {
                let needs_incrememnt = q_all_nodes
                    .iter_many(children)
                    .find(|(order, _)| order.0 as i32 == index)
                    .is_some();
                if needs_incrememnt {
                    for entity in children {
                        if let Ok((mut order, path_seg)) = q_all_nodes.get_mut(*entity) {
                            // Increment all indices after the one we're about to insert
                            if order.0 as i32 >= index {
                                order.0 += 1;
                            }
                        }
                    }
                }
            }

            let to_insert = VectorObjectSpawner::new_vector_node(
                index.max(0) as usize,
                &position,
                path_seg,
                &res.cached_paths.endpoint_node,
                &res.cached_paths.control_node,
            );

            to_insert
                .with_parent(parent)
                .with_extra(move |entity| {
                    entity.insert((
                        path_seg,
                        Ordered(index.max(0) as usize),
                        VecNodeTag::default(),
                    ));
                })
                .spawn_with_world(world);

            match reset_vector_object_origin(world, parent) {
                Err(err) => {
                    debug_log!("InsertVecNode: Resetting origin error {:?}", err);
                }
                _ => {}
            }
            // TODO: Return undo action
        }
        ElementOperation::SetActiveVecNode { parent, new_active } => {
            let mut sys_state: SystemState<(
                // VectorObject
                Query<&Children, With<VectorObjectTag>>,
                // Vector nodes query
                Query<(&mut Ordered, &mut VecNodeTag), With<VecNodeTag>>,
            )> = SystemState::new(world);
            let (q_vector_objs, mut q_all_nodes) = sys_state.get_mut(world);

            if let Ok(children) = q_vector_objs.get(parent) {
                for child in children {
                    if let Ok((order, mut tag)) = q_all_nodes.get_mut(*child) {
                        tag.set_if_neq(VecNodeTag::Default);
                        if order.0 == new_active {
                            *tag = VecNodeTag::Active;
                        }
                    }
                }
            }
        }
        ElementOperation::Delete(to_delete) => {
            for entity in to_delete {
                if let Some(entity) = world.get_entity_mut(entity) {
                    entity.despawn_recursive();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
