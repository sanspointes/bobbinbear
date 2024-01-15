use bevy::prelude::*;
use bevy_mod_raycast::RaycastMesh;
use serde::{Deserialize, Serialize};

use crate::plugins::{bounds_2d_plugin::GlobalBounds2D, selection_plugin::{Selectable, Selected}};

#[derive(Serialize, Deserialize)]
pub enum SerialisedComponent {
    Name(Name),
    Transform(Transform),
    GlobalTransform(GlobalTransform),
    Visibility(Visibility),
    ComputedVisibility(ComputedVisibility),

    RaycastMesh(RaycastMesh<Selectable>),
    Selectable(Selectable),
    Selected(Selected),

    GlobalBounds2D(GlobalBounds2D),
}

impl From<Name> for SerialisedComponent {
    fn from(value: Name) -> Self {
        SerialisedComponent::Name(value)
    }
}

impl From<Transform> for SerialisedComponent {
    fn from(value: Transform) -> Self {
        SerialisedComponent::Transform(value)
    }
}

impl From<GlobalBounds2D> for SerialisedComponent {
    fn from(value: GlobalBounds2D) -> Self {
        SerialisedComponent::GlobalBounds2D(GlobalBounds2D::NeedsCalculate)
    }
}

impl From<RaycastMesh<Selectable>> for SerialisedComponent {
    fn from(value: RaycastMesh<Selectable>) -> Self {
        SerialisedComponent::RaycastMesh(value)
    }
}

impl From<Selectable> for SerialisedComponent {
    fn from(value: Selectable) -> Self {
        SerialisedComponent::Selectable(value)
    }
}

impl From<Selected> for SerialisedComponent {
    fn from(value: Selected) -> Self {
        SerialisedComponent::Selected(value)
    }
}
