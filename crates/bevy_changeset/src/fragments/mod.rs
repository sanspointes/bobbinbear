use bevy_ecs::{component::Component, world::World};
use bevy_reflect::{erased_serde::Serialize, FromReflect, Reflect};

pub struct ReflectedComponent(u32);

impl ReflectedComponent {
    pub fn new<T: Component + Reflect + FromReflect + Serialize>(
        &self,
        world: &mut World,
        component: T,
    ) -> Self {
        let serializable = component.serializable().unwrap();
        todo!()
    }
}
