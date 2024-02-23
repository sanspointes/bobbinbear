mod fragment;
mod uid;

pub mod prelude {
    pub use crate::fragment::{ComponentFragment, EntityFragment, HierarchyFragment};
    pub use crate::uid::Uid;
}

// use std::{any::TypeId, marker::PhantomData};
//
// use bevy_ecs::{
//     reflect::ReflectComponent,
//     system::Resource,
//     world::{EntityRef, Mut, World},
// };
// use bevy_reflect::{GetTypeRegistration, Reflect, TypeRegistry};
// use bevy_scene::{DynamicEntity, SceneFilter};
// use bevy_utils::HashSet;
// use fragment::EntityFragment;
// use uid::Uid;

// trait WorldExt {
//     fn with_fragment_resource<TTag: Send + Sync, TRet>(
//         &mut self,
//         handler: impl FnMut(&mut World, Mut<FragmentResource<TTag>>) -> TRet,
//     ) -> TRet;
// }
//
// impl WorldExt for World {
//     fn with_fragment_resource<TTag: Send + Sync, TRet>(
//         &mut self,
//         handler: impl FnMut(&mut World, Mut<FragmentResource<TTag>>) -> TRet,
//     ) -> TRet {
//         self.resource_scope::<FragmentResource<TTag>, TRet>(handler)
//     }
// }
//
// pub struct FragmentContext {
//     pub filter: SceneFilter,
// }
//
// impl<TTag> From<&FragmentResource<TTag>> for FragmentContext {
//     fn from(value: &FragmentResource<TTag>) -> Self {
//         let allowed_ids: HashSet<TypeId> = value
//             .type_registry
//             .iter()
//             .map(|reg| reg.type_id())
//             .collect();
//         let filter = SceneFilter::Allowlist(allowed_ids);
//
//         Self { filter }
//     }
// }
//
// #[derive(Resource)]
// /// Resource to serialize/deserialize entities.
// ///
// /// * `type_registry`:
// /// * `pd`:
// pub struct FragmentResource<TTag> {
//     type_registry: TypeRegistry,
//     pd: PhantomData<TTag>,
// }
//
// impl<TTag: Send + Sync> FragmentResource<TTag> {
//     /// Register a type to be serialized by this fragment serializer
//     pub fn register_type<TReg: Reflect + GetTypeRegistration>(&mut self) -> &mut Self {
//         self.type_registry.register::<TReg>();
//         self
//     }
//
//     pub fn try_entity_fragment_from_uid(world: &mut World, uid: Uid) -> EntityFragment {
//         world.with_fragment_resource::<TTag, EntityFragment>(|world, mut res| {
//             let entity = uid.entity(world).unwrap();
//             let entity_ref = world.entity(entity);
//             res.try_entity_fragment_from_entity_ref(world, &entity_ref)
//         })
//     }
//
//     pub fn try_entity_fragment_from_entity_ref(
//         &mut self,
//         world: &mut World,
//         entity_ref: &EntityRef,
//     ) -> EntityFragment {
//         let mut dynamic_entity = DynamicEntity {
//             entity: entity_ref.id(),
//             components: Vec::new(),
//         };
//
//         for comp_id in entity_ref.archetype().components() {
//             let reflected = world
//                 .components()
//                 .get_info(comp_id)
//                 .and_then(|c| c.type_id())
//                 .filter(|id| filter.is_allowed_by_id(*id))
//                 .and_then(|id| self.type_registry.get(id))
//                 .and_then(|reg| reg.data::<ReflectComponent>())
//                 .and_then(|reflect| reflect.reflect(entity_ref));
//
//             if let Some(reflect) = reflected {
//                 dynamic_entity.components.push(reflect.clone_value());
//             }
//         }
//
//         EntityFragment::new(uid, dynamic_entity)
//     }
//
//     pub fn try_entity_fragment_recursive_children(
//         &mut self,
//         world: &mut World,
//         uid: Uid,
//     ) -> EntityFragment {
//     }
// }
