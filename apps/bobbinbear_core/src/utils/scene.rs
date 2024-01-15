use bevy::prelude::*;

pub fn get_all_children_recursive(
    entity: Entity, 
    children_query: &Query<Option<&Children>>, 
    entities: &mut Vec<Entity>
) {
    let Ok(children) = children_query.get(entity) else { return };

    entities.push(entity);
    if let Some(children) = children {
        for child in children {
            let child = *child;
            get_all_children_recursive(child, children_query, entities)
        }
    }
}
