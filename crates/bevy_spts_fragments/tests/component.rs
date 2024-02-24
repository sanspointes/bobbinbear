
use std::any::Any;

use bevy_app::App;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{Reflect, TypeRegistry};
use bevy_spts_fragments::prelude::ComponentFragment;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct TestComponent {
    pub field1: i32,
}

#[test]
pub fn test_from_component() {
    let mut app = App::new();
    let world = &mut app.world;

    let mut tr = TypeRegistry::new();
    tr.register::<TestComponent>();

    let component = TestComponent {
        field1: 0,
    };
    let cf = ComponentFragment::from_component(&component);

    let mut entity_mut = world.spawn_empty();
    cf.insert_to_entity_world_mut(&tr, &mut entity_mut).unwrap();
    let entity = entity_mut.id();

    let c = world.get::<TestComponent>(entity);
    assert!(c.is_some());
    assert_eq!(c.unwrap().get_represented_type_info().unwrap().type_path(), "component::TestComponent");
}

#[test]
pub fn test_from_entity_and_type_id() {
    let mut app = App::new();
    let world = &mut app.world;

    let mut tr = TypeRegistry::new();
    tr.register::<TestComponent>();

    // Spawn entity with component
    let component = TestComponent::default();
    let type_id = component.type_id();

    let entity = world.spawn(component).id();
    let entity_ref = world.entity(entity);

    // Store the component fragment for later
    let cf = ComponentFragment::from_type_id(&tr, &entity_ref, type_id).unwrap();

    // Check entity has component
    let c = world.get::<TestComponent>(entity);
    assert!(c.is_some());

    // Remove it and check entity doesn't have component
    world.entity_mut(entity).remove::<TestComponent>();
    let c = world.get::<TestComponent>(entity);
    assert!(c.is_none());

    // Re-add component via the ComponentFragment
    let mut entity_mut = world.entity_mut(entity);
    cf.insert_to_entity_world_mut(&tr, &mut entity_mut).unwrap();

    // Check entity has component again
    let c = world.get::<TestComponent>(entity);
    assert!(c.is_some());
    assert_eq!(c.unwrap().get_represented_type_info().unwrap().type_path(), "component::TestComponent");
}
