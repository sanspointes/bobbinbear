use bevy::prelude::*;

pub fn get_all_children_recursive(
    entity: Entity, 
    children_query: &Query<&Children>, 
    mut entities: &mut Vec<Entity>
) {
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            entities.push(child);
            get_all_children_recursive(child, children_query, entities);
        }
    }
}
