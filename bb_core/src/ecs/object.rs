use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        reflect::{ReflectBundle, ReflectComponent},
    },
    math::{Vec2, Vec3},
    reflect::Reflect,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    transform::components::{GlobalTransform, Transform},
};
use bevy_spts_uid::Uid;

use crate::plugins::selected::Selected;

#[derive(Component, Reflect)]
#[reflect(Component)]
/// Marker component for an object that should not be visible in the frontend / editor.
pub struct InternalObject;

#[derive(Bundle, Reflect)]
#[reflect(Bundle)]
/// Creates a scene object
///
/// * `position`:
/// * `selected`:
/// * `transform`:
/// * `global_transform`:
/// * `visibility`:
/// * `view_visibility`:
/// * `inherited_visibility`:
pub struct ObjectBundle {
    object_type: ObjectType,
    position: Position,
    selected: Selected,

    transform: Transform,
    global_transform: GlobalTransform,

    visibility: Visibility,
    view_visibility: ViewVisibility,
    inherited_visibility: InheritedVisibility,
}

#[allow(useless_deprecated)]
impl Default for ObjectBundle {
    #[deprecated = "Default only supplied for reflect.  Use new()."]
    fn default() -> Self {
        Self {
            object_type: ObjectType::default(),
            position: Position::default(),
            selected: Selected::default(),

            transform: Transform::default(),
            global_transform: GlobalTransform::default(),

            visibility: Visibility::default(),
            view_visibility: ViewVisibility::default(),
            inherited_visibility: InheritedVisibility::default(),
        }
    }
}

impl ObjectBundle {
    pub fn new(object_type: ObjectType) -> Self {
        Self {
            object_type,
            ..Default::default()
        }
    }

    pub fn proxy_viewport(target: Uid) -> Self {
        Self {
            position: Position::ProxyViewport {
                target,
                target_world_position: Vec3::ZERO,
            },
            selected: Selected::Proxy {
                target,
                selected: false,
            },
            ..Default::default()
        }
    }

    pub fn with_local_position(mut self, position: impl Into<Vec2>) -> Self {
        self.position = Position::Local(position.into());
        self
    }

    pub fn with_z_position(mut self, z: f32) -> Self {
        self.transform.translation.z = z;
        self
    }
}

pub use definitions::ObjectType;

use super::position::Position;

#[allow(non_snake_case, clippy::empty_docs)]
mod definitions {
    use bevy::{
        ecs::{component::Component, reflect::ReflectComponent},
        reflect::Reflect,
    };
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;

    #[derive(Debug, Component, Reflect, Default, Clone, Copy)]
    #[reflect(Component)]
    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    /// Enum representing all object types within the editor
    pub enum ObjectType {
        #[default]
        Unknown,
        Vector,
        VectorSegment,
        VectorEndpoint,
        VectorCtrl,
    }
}
