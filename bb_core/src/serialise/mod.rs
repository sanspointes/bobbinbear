mod components;
mod materials;

pub struct EncodeContext {
    materials: 
}

// use bevy::{ecs::world::EntityWorldMut, prelude::*};
//
// macro_rules! impl_enc_component_tag {
//     ($($var:ident),*) => {
//         #[derive(Clone, Debug)]
//         pub enum EncComponentTag { $(
//             $var,
//         )*}
//
//         #[derive(Clone, Debug)]
//         pub struct EncComponent(pub EncComponentTag, pub Vec<u8>);
//
//         impl EncComponent {
//             pub fn from_tag_and_entity_world_mut(tag: EncComponentTag, entity_mut: &mut EntityWorldMut) -> Self {
//                 let data = match tag {
//                     $(EncComponentTag::$var => {
//                         let component = entity_mut.get::<$var>().unwrap();
//                         rmp_serde::to_vec::<$var>(&component).unwrap()
//                     },)*
//                 };
//                 Self(tag, data)
//             }
//
//             pub fn remove_from_entity_world_mut(
//                 &self,
//                 entity_mut: &mut EntityWorldMut,
//             ) -> Result<(), rmp_serde::decode::Error> {
//                 match &self.0 {
//                     $(EncComponentTag::$var => {
//                         entity_mut.remove::<$var>();
//                     },)*
//                 }
//                 Ok(())
//             }
//
//             pub fn swap_with_entity_world_mut(
//                 &mut self,
//                 entity_mut: &mut EntityWorldMut,
//             ) -> Result<(), rmp_serde::decode::Error> {
//                 let data = match &self.0 {
//                     $(EncComponentTag::$var => {
//                         let component = entity_mut.get::<$var>().unwrap();
//                         rmp_serde::to_vec::<$var>(&component).unwrap()
//                     })*
//                 };
//                 self.try_insert_into_entity_world_mut(entity_mut)?;
//                 self.1 = data;
//                 Ok(())
//             }
//
//             pub fn try_insert_into_entity_world_mut(
//                 &self,
//                 entity_mut: &mut EntityWorldMut,
//             ) -> Result<(), rmp_serde::decode::Error> {
//                 match &self.0 {
//                     $(EncComponentTag::$var => {
//                         let component = rmp_serde::from_slice::<$var>(&self.1);
//                         component.map(|component| {
//                             entity_mut.insert(component);
//                         })
//                     },)*
//                 }
//             }
//         }
//     }
// }
//
//
// impl_enc_component_tag!{ Name, Transform, GlobalTransform, Visibility, InheritedVisibility, ViewVisibility }
//
// pub trait EncodableComponent {
//     fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error>;
//     fn encode(&self) -> EncComponent {
//         self.try_encode().unwrap()
//     }
// }
//
// macro_rules! impl_encodable_component {
//     ($value:ident) => {
//         impl EncodableComponent for $value {
//             fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
//                 let data = rmp_serde::to_vec(self);
//                 data.map(|data| EncComponent(EncComponentTag::$value, data))
//             }
//         }
//     };
// }
//
// impl_encodable_component!(Name);
// impl_encodable_component!(Mesh);
// impl_encodable_component!(Transform);
// impl_encodable_component!(Handle<ColorMaterial>);
// impl_encodable_component!(GlobalTransform);
// impl_encodable_component!(Visibility);
// impl_encodable_component!(ViewVisibility);
// impl_encodable_component!(InheritedVisibility);
