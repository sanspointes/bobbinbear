use bevy::prelude::*;

pub fn dump_entity_components(world: &World, entity: Entity) {
    debug!("Inspecting entity {entity:?}");
    let cmps = world.inspect_entity(entity);
    for cmp in cmps {
        debug!("\t - {:?}", cmp.name());
    }
}
