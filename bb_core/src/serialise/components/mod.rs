use bevy::prelude::*;


#[derive(Clone, Debug)]
pub enum EncComponentTag {
    // Bevy Scene components
    Name,
    Transform,
    GlobalTransform,
    Visiblity,
    InheritedVisibility,
    // Bevy renderer components
    Mesh,
    HandleColorMaterial,
}

#[derive(Clone, Debug)]
pub struct EncComponent(pub EncComponentTag, pub Vec<u8>);


pub trait EncodableComponent {
    fn try_encode(&self, &mut E) -> Result<EncComponent, rmp_serde::encode::Error>;
    fn encode(&self) -> EncComponent {
        self.try_encode().unwrap()
    }
}

impl EncodableComponent for Name {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::Name, data))
    }
}
impl EncodableComponent for Transform {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::Transform, data))
    }
}
impl EncodableComponent for GlobalTransform {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::GlobalTransform, data))
    }
}
impl EncodableComponent for Visibility {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::Visibility, data))
    }
}
impl EncodableComponent for InheritedVisibility {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::InheritedVisibility, data))
    }
}
impl EncodableComponent for Mesh {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::Mesh, data))
    }
}
impl EncodableComponent for Handle<ColorMaterial> {
    fn try_encode(&self) -> Result<EncComponent, rmp_serde::encode::Error> {
        self::reflec
        let data = rmp_serde::to_vec(self)?;
        Ok(EncComponent(EncComponentTag::HandleColorMaterial, data))
    }
}
