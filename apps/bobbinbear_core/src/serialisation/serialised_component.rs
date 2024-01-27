//! serialised_component
//!
//! Contains a serlisable definition of a component.

use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_mod_raycast::prelude::RaycastMesh;

use crate::{plugins::{
    bounds_2d_plugin::GlobalBounds2D,
    selection_plugin::{Selectable, Selected},
    vector_graph_plugin::{Fill, Stroke, VectorGraph},
}, components::scene::{BBObject, VectorGraphDirty}};

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

/// ViewVisibilityDef
///
/// Serialisable representation of `ViewVisibility` component
///
/// Because this data is computed we'll just inject a default one into the scene and let bevy set
/// the correct values.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ViewVisibilityDef;
impl From<ViewVisibility> for ViewVisibilityDef {
    fn from(value: ViewVisibility) -> Self {
        ViewVisibilityDef
    }
}
impl From<ViewVisibilityDef> for ViewVisibility {
    fn from(value: ViewVisibilityDef) -> Self {
        ViewVisibility::default()
    }
}
/// InheritedVisibilityDef
///
/// Serialisable representation of `InheritedVisibility` component
///
/// Because this data is computed we'll just inject a default one into the scene and let bevy set
/// the correct values.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct InheritedVisibilityDef;
impl From<InheritedVisibility> for InheritedVisibilityDef {
    fn from(value: InheritedVisibility) -> Self {
        InheritedVisibilityDef
    }
}
impl From<InheritedVisibilityDef> for InheritedVisibility {
    fn from(value: InheritedVisibilityDef) -> Self {
        InheritedVisibility::default()
    }
}

/// Mesh2dHandleDef
///
/// Serialisable representation of `Mesh2dHandle` component
///
/// Because this data is computed we'll just inject a default one into the scene and let bevy set
/// the correct values.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Mesh2dHandleDef;
impl From<Mesh2dHandle> for Mesh2dHandleDef {
    fn from(value: Mesh2dHandle) -> Self {
        Mesh2dHandleDef
    }
}
impl From<Mesh2dHandleDef> for Mesh2dHandle {
    fn from(value: Mesh2dHandleDef) -> Self {
        Mesh2dHandle::default()
    }
}

/// ColorMaterialHandleDef
///
/// Serialisable representation of `ColorMaterialHandle` component
///
/// Because the component is a Handle<ColorMaterial> we'll need a from/to method that mutates the
/// world.
///
/// TODO: Same material detection for faster rendering.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ColorMaterialHandleDef {
    pub color: Color,
}
impl ColorMaterialHandleDef {
    pub fn to_world_and_handle(&self, world: &mut World) -> Handle<ColorMaterial> {
        let mut color_mat_assets = world.resource_mut::<Assets<ColorMaterial>>();
        let handle = color_mat_assets.add(ColorMaterial::from(self.color));
        handle
    }
    pub fn from_world_and_handle(world: &World, handle: Handle<ColorMaterial>) -> Self {
        let color_mat_assets = world.resource::<Assets<ColorMaterial>>();
        let material = color_mat_assets
            .get(&handle)
            .expect("from_world_and_handle(). No material {handle:?}");
        Self {
            color: material.color,
        }
    }
}

/// RaycastMeshDef
///
/// Serialisable representation of `RaycastMesh<Selectable>` component
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RaycastMeshSelectableDef;
impl From<RaycastMesh<Selectable>> for RaycastMeshSelectableDef {
    fn from(value: RaycastMesh<Selectable>) -> Self {
        RaycastMeshSelectableDef
    }
}
impl From<RaycastMeshSelectableDef> for RaycastMesh<Selectable> {
    fn from(value: RaycastMeshSelectableDef) -> Self {
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
    // Builtins
    Name(NameDef),
    Transform(TransformDef),
    GlobalTransform(GlobalTransformDef),
    Visibility(VisibilityDef),
    ViewVisibility(ViewVisibilityDef),
    InheritedVisibility(InheritedVisibilityDef),
    Mesh2dHandle(Mesh2dHandleDef),
    ColorMaterial(ColorMaterialHandleDef),

    // App State
    RaycastMeshSelectable(RaycastMeshSelectableDef),
    Selectable(SelectableDef),
    Selected(SelectedDef),
    GlobalBounds2D(GlobalBounds2DDef),

    // Object Types
    BBObject(BBObject),

    // bb_vector_network_related
    VectorGraph(VectorGraph),
    VectorGraphDirty(VectorGraphDirty),
    Fill(Fill),
    Stroke(Stroke),
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
impl From<ViewVisibility> for SerialisedComponent {
    fn from(value: ViewVisibility) -> Self {
        SerialisedComponent::ViewVisibility(value.into())
    }
}
impl From<InheritedVisibility> for SerialisedComponent {
    fn from(value: InheritedVisibility) -> Self {
        SerialisedComponent::InheritedVisibility(value.into())
    }
}

impl From<RaycastMesh<Selectable>> for SerialisedComponent {
    fn from(value: RaycastMesh<Selectable>) -> Self {
        SerialisedComponent::RaycastMeshSelectable(value.into())
    }
}

impl From<BBObject> for SerialisedComponent {
    fn from(value: BBObject) -> Self {
        SerialisedComponent::BBObject(value)
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

impl From<Fill> for SerialisedComponent {
    fn from(value: Fill) -> Self {
        SerialisedComponent::Fill(value)
    }
}
impl From<Stroke> for SerialisedComponent {
    fn from(value: Stroke) -> Self {
        SerialisedComponent::Stroke(value)
    }
}
impl From<VectorGraph> for SerialisedComponent {
    fn from(value: VectorGraph) -> Self {
        SerialisedComponent::VectorGraph(value)
    }
}
impl From<VectorGraphDirty> for SerialisedComponent {
    fn from(value: VectorGraphDirty) -> Self {
        SerialisedComponent::VectorGraphDirty(value)
    }
}

impl From<GlobalBounds2D> for SerialisedComponent {
    fn from(value: GlobalBounds2D) -> Self {
        SerialisedComponent::GlobalBounds2D(value.into())
    }
}
