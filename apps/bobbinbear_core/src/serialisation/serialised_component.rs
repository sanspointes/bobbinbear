//! serialised_component
//!
//! Contains a serlisable definition of a component.

use bevy::{prelude::*, math::Affine3A};
use bevy_mod_raycast::RaycastMesh;

use crate::plugins::{bounds_2d_plugin::GlobalBounds2D, selection_plugin::{Selectable, Selected}};

/// NameDef
///
/// Serialisable representation of `Name` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct NameDef(pub String);
impl From<Name> for NameDef {
    fn from(value: Name) -> Self {
        NameDef(value.to_string())
    }
}
impl From<NameDef> for Name {
    fn from(value: NameDef) -> Self {
        Name::from(value.0.to_string())
    }
}

/// TransformDef
///
/// Serialisable representation of `Transform` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct TransformDef {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
impl From<Transform> for TransformDef {
    fn from(value: Transform) -> Self {
        TransformDef {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}
impl From<TransformDef> for Transform {
    fn from(value: TransformDef) -> Self {
        Transform {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}


/// GlobalTransformDef
///
/// Serialisable representation of `GlobalTransform` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalTransformDef(pub Mat4);
impl From<GlobalTransform> for GlobalTransformDef {
    fn from(value: GlobalTransform) -> Self {
        GlobalTransformDef(value.compute_matrix())
    }
}
impl From<GlobalTransformDef> for GlobalTransform {
    fn from(value: GlobalTransformDef) -> Self {
        GlobalTransform::from(value.0)
    }
}

/// VisibilityDef
///
/// Serialisable representation of `Visibility` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum VisibilityDef {
    Inherited,
    Hidden,
    Visible,
}
impl From<Visibility> for VisibilityDef {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Inherited => VisibilityDef::Inherited,
            Visibility::Hidden => VisibilityDef::Hidden,
            Visibility::Visible => VisibilityDef::Visible,
        }
    }
}
impl From<VisibilityDef> for Visibility {
    fn from(value: VisibilityDef) -> Self {
        match value {
            VisibilityDef::Inherited => Visibility::Inherited,
            VisibilityDef::Hidden => Visibility::Hidden,
            VisibilityDef::Visible => Visibility::Visible,
        }
    }
}

/// ComputedVisibilityDef
///
/// Serialisable representation of `ComputedVisibility` component
///
/// Because this data is computed we'll just inject a default one into the scene and let bevy set
/// the correct values.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ComputedVisibilityDef;
impl From<ComputedVisibility> for ComputedVisibilityDef {
    fn from(value: ComputedVisibility) -> Self {
        ComputedVisibilityDef
    }
}
impl From<ComputedVisibilityDef> for ComputedVisibility {
    fn from(value: ComputedVisibilityDef) -> Self {
        ComputedVisibility::default()
    }
}

/// RaycastMeshDef
///
/// Serialisable representation of `RaycastMesh<Selectable>` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RaycastMeshDef;
impl From<RaycastMesh<Selectable>> for RaycastMeshDef {
    fn from(value: RaycastMesh<Selectable>) -> Self {
        RaycastMeshDef
    }
}
impl From<RaycastMeshDef> for RaycastMesh<Selectable> {
    fn from(value: RaycastMeshDef) -> Self {
        RaycastMesh::<Selectable>::default()
    }
}

/// SelectableDef
///
/// Serialisable representation of `Selectable` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum SelectableDef {
    Locked,
    Default,
}
impl From<Selectable> for SelectableDef {
    fn from(value: Selectable) -> Self {
        match value {
            Selectable::Locked => SelectableDef::Locked,
            Selectable::Default => SelectableDef::Default,
        }
    }
}
impl From<SelectableDef> for Selectable {
    fn from(value: SelectableDef) -> Self {
        match value {
            SelectableDef::Locked => Selectable::Locked,
            SelectableDef::Default => Selectable::Default,
        }
    }
}

/// SelectedDef
///
/// Serialisable representation of `Selected` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum SelectedDef {
    No,
    Yes,
}
impl From<Selected> for SelectedDef {
    fn from(value: Selected) -> Self {
        match value {
            Selected::No => SelectedDef::No,
            Selected::Yes => SelectedDef::Yes,
        }
    }
}
impl From<SelectedDef> for Selected {
    fn from(value: SelectedDef) -> Self {
        match value {
            SelectedDef::No => Selected::No,
            SelectedDef::Yes => Selected::Yes,
        }
    }
}

/// GlobalBounds2DDef
///
/// Serialisable representation of `GlobalBounds2D` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalBounds2DDef;
impl From<GlobalBounds2D> for GlobalBounds2DDef {
    fn from(value: GlobalBounds2D) -> Self {
        GlobalBounds2DDef
    }
}
impl From<GlobalBounds2DDef> for GlobalBounds2D {
    fn from(value: GlobalBounds2DDef) -> Self {
        GlobalBounds2D::NeedsCalculate
    }
}


/// SerialisedComponent
/// 
/// Generic Serialisable representation of a component.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum SerialisedComponent {
    Name(NameDef),
    Transform(TransformDef),
    GlobalTransform(GlobalTransformDef),
    Visibility(VisibilityDef),
    ComputedVisibility(ComputedVisibilityDef),

    RaycastMesh(RaycastMeshDef),
    Selectable(SelectableDef),
    Selected(SelectedDef),

    GlobalBounds2D(GlobalBounds2DDef),
}

impl From<Name> for SerialisedComponent {
    fn from(value: Name) -> Self {
        SerialisedComponent::Name(value.into())
    }
}

impl From<Transform> for SerialisedComponent {
    fn from(value: Transform) -> Self {
        SerialisedComponent::Transform(value.into())
    }
}
impl From<GlobalTransform> for SerialisedComponent {
    fn from(value: GlobalTransform) -> Self {
        SerialisedComponent::GlobalTransform(value.into())
    }
}

impl From<Visibility> for SerialisedComponent {
    fn from(value: Visibility) -> Self {
        SerialisedComponent::Visibility(value.into())
    }
}
impl From<ComputedVisibility> for SerialisedComponent {
    fn from(value: ComputedVisibility) -> Self {
        SerialisedComponent::ComputedVisibility(value.into())
    }
}

impl From<RaycastMesh<Selectable>> for SerialisedComponent {
    fn from(value: RaycastMesh<Selectable>) -> Self {
        SerialisedComponent::RaycastMesh(value.into())
    }
}

impl From<Selectable> for SerialisedComponent {
    fn from(value: Selectable) -> Self {
        SerialisedComponent::Selectable(value.into())
    }
}

impl From<Selected> for SerialisedComponent {
    fn from(value: Selected) -> Self {
        SerialisedComponent::Selected(value.into())
    }
}

impl From<GlobalBounds2D> for SerialisedComponent {
    fn from(value: GlobalBounds2D) -> Self {
        SerialisedComponent::GlobalBounds2D(value.into())
    }
}
