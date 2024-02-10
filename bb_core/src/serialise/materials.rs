#[derive(Clone, Debug)]
pub enum EncMaterialTag {
    ColorMaterial,
}

#[derive(Clone, Debug)]
pub struct EncComponent(pub EncComponentTag, pub Vec<u8>);


