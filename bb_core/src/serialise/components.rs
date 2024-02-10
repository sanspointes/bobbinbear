use anyhow::anyhow;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ecs::core::{DerivedMaterial, DerivedMesh};

#[derive(Clone, Debug)]
pub enum EncComponentTag {
    // Bevy Scene components
    Name,
    Transform,
    GlobalTransform,
    Visibility,
    InheritedVisibility,
    ViewVisibility,
    // These represent a mesh2d or a material 2d that is computed based on other components on the
    // entity.
    DerivedMaterial,
    DerivedMesh,
}

#[derive(Debug)]
pub enum EncDecodeError {
    SerdeError(rmp_serde::decode::Error),
    Unknown(anyhow::Error),
}
impl From<rmp_serde::decode::Error> for EncDecodeError {
    fn from(value: rmp_serde::decode::Error) -> Self {
        Self::SerdeError(value)
    }
}
impl From<anyhow::Error> for EncDecodeError {
    fn from(value: anyhow::Error) -> Self {
        Self::Unknown(value)
    }
}

#[derive(Clone, Debug)]
pub struct EncComponent(pub EncComponentTag, pub Vec<u8>);

macro_rules! impl_enc_component {
    ($($var:ident),*) => {
        impl EncComponent {
            pub fn from_tag_and_entity_world_mut(
                tag: EncComponentTag,
                entity_mut: &mut EntityWorldMut
            ) -> Result<Self, anyhow::Error> {
                let enc_component = match tag {
                    $(EncComponentTag::$var => {
                        let component = entity_mut.get::<$var>().unwrap();
                        component.try_encode()?
                    },)*
                };
                Ok(enc_component)
            }

            pub fn try_swap_with_entity_world_mut(
                &mut self,
                entity_mut: &mut EntityWorldMut,
            ) -> Result<(), anyhow::Error> {
                let data = match &self.0 {
                    $(EncComponentTag::$var => {
                        let component = entity_mut.get::<$var>().unwrap();
                        component.try_encode()
                    })*
                }?;
                self.try_insert_into_entity_world_mut(entity_mut)?;
                self.1 = data.1;
                Ok(())
            }

            pub fn try_insert_into_entity_world_mut(
                &self,
                entity_mut: &mut EntityWorldMut,
            ) -> Result<(), anyhow::Error> {
                match &self.0 {
                    $(EncComponentTag::$var => {
                        entity_mut.remove::<$var>();
                    },)*
                }
                Ok(())
            }

            pub fn try_remove_from_entity_world_mut(
                &self,
                entity_mut: &mut EntityWorldMut,
            ) -> Result<(), anyhow::Error> {
                match &self.0 {
                    $(EncComponentTag::$var => {
                        let component: $var = self.try_into().unwrap();
                        entity_mut.insert(component);
                    },)*
                }
                Ok(())
            }
        }
    }
}
impl_enc_component! { Name, Transform, GlobalTransform, Visibility, InheritedVisibility, ViewVisibility, DerivedMaterial, DerivedMesh }

pub trait EncodableComponent {
    fn try_encode(&self) -> Result<EncComponent, anyhow::Error>;
    fn encode(&self) -> EncComponent {
        self.try_encode().unwrap()
    }
}

macro_rules! impl_encodable_component {
    ($value:ident) => {
        impl EncodableComponent for $value {
            fn try_encode(&self) -> Result<EncComponent, anyhow::Error> {
                let data = rmp_serde::to_vec(self)?;
                Ok(EncComponent(EncComponentTag::$value, data))
            }
        }

        impl TryFrom<&EncComponent> for $value {
            type Error = anyhow::Error;
            fn try_from(value: &EncComponent) -> Result<Self, Self::Error> {
                match value.0 {
                    EncComponentTag::$value => {
                        let component = rmp_serde::from_slice::<$value>(&value.1);
                        component.map_err(|err| err.into())
                    }
                    _ => Err(anyhow!("Can't decode into $value")),
                }
            }
        }
    };
    ($value:ident,$proxy:ident) => {
        impl EncodableComponent for $value {
            fn try_encode(&self) -> Result<EncComponent, anyhow::Error> {
                let proxy: $proxy = self.clone().into();
                let data = rmp_serde::to_vec(&proxy)?;
                Ok(EncComponent(EncComponentTag::$value, data))
            }
        }

        impl TryFrom<&EncComponent> for $value {
            type Error = anyhow::Error;
            fn try_from(value: &EncComponent) -> Result<Self, Self::Error> {
                match value.0 {
                    EncComponentTag::$value => {
                        let proxy = rmp_serde::from_slice::<$proxy>(&value.1);
                        match proxy {
                            Ok(proxy) => Ok(proxy.into()),
                            Err(err) => Err(err.into()),
                        }
                    }
                    _ => Err(anyhow!("Can't decode into Name")),
                }
            }
        }
    };
}

impl_encodable_component!(Name);
impl_encodable_component!(Transform);
impl_encodable_component!(GlobalTransform);

#[derive(Serialize, Deserialize)]
pub enum VisibilityDef {
    Inherited,
    Hidden,
    Visible,
}
impl From<Visibility> for VisibilityDef {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Visible => VisibilityDef::Visible,
            Visibility::Hidden => VisibilityDef::Hidden,
            Visibility::Inherited => VisibilityDef::Inherited,
        }
    }
}
impl From<VisibilityDef> for Visibility {
    fn from(value: VisibilityDef) -> Self {
        match value {
            VisibilityDef::Visible => Visibility::Visible,
            VisibilityDef::Hidden => Visibility::Hidden,
            VisibilityDef::Inherited => Visibility::Inherited,
        }
    }
}
impl_encodable_component!(Visibility, VisibilityDef);

#[derive(Serialize, Deserialize)]
pub struct InheritedVisibilityDef(pub bool);

impl From<InheritedVisibility> for InheritedVisibilityDef {
    fn from(value: InheritedVisibility) -> Self {
        Self(value.get())
    }
}
impl From<InheritedVisibilityDef> for InheritedVisibility {
    fn from(value: InheritedVisibilityDef) -> Self {
        match value.0 {
            true => Self::VISIBLE,
            false => Self::HIDDEN,
        }
    }
}
impl_encodable_component!(InheritedVisibility, InheritedVisibilityDef);

#[derive(Serialize, Deserialize)]
pub struct ViewVisibilityDef(pub bool);

impl From<ViewVisibility> for ViewVisibilityDef {
    fn from(value: ViewVisibility) -> Self {
        Self(value.get())
    }
}
impl From<ViewVisibilityDef> for ViewVisibility {
    fn from(value: ViewVisibilityDef) -> Self {
        match value.0 {
            true => Self::HIDDEN, // TODO: Fix this, this should be positive.
            false => Self::HIDDEN,
        }
    }
}
impl_encodable_component!(ViewVisibility, ViewVisibilityDef);

impl_encodable_component!(DerivedMaterial);
impl_encodable_component!(DerivedMesh);
