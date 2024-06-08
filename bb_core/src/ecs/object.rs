use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        reflect::{ReflectBundle, ReflectComponent},
    },
    math::Vec2,
    reflect::Reflect,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    transform::components::{GlobalTransform, Transform},
};
use bevy_mod_raycast::deferred::RaycastMesh;
use bevy_spts_uid::Uid;

use crate::plugins::selected::{Hovered, ProxiedHovered, ProxiedSelectable, ProxiedSelected, ProxiedVisibility, Selectable, Selected};

pub type ProxiedUid = ProxiedComponent<Uid, ()>;

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
    selectable: Selectable,
    rc_selectable: RaycastMesh<Selectable>,
    hovered: Hovered,

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
            selectable: Selectable::default(),
            rc_selectable: RaycastMesh::<Selectable>::default(),
            hovered: Hovered::default(),

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

    pub fn with_position(mut self, position: impl Into<Vec2>) -> Self {
        self.position = Position(position.into());
        self
    }

    pub fn with_z_position(mut self, z: f32) -> Self {
        self.transform.translation.z = z;
        self
    }
}

#[derive(Bundle, Reflect)]
#[reflect(Bundle)]
pub struct ProxiedObjectBundle {
    uid_proxy: ProxiedUid,
    position_proxy: ProxiedPosition,
    visibility_proxy: ProxiedVisibility,
    selected_proxy: ProxiedSelected,
    selectable_proxy: ProxiedSelectable,
    hovered_proxy: ProxiedHovered,
}

impl ProxiedObjectBundle {
    pub fn new(target: Uid) -> Self {
        Self {
            uid_proxy: ProxiedUid::new(target, ()),
            position_proxy: ProxiedPosition::new(target, ProxiedPositionStrategy::Viewport),
            visibility_proxy: ProxiedVisibility::new(target, ()),
            selected_proxy: ProxiedSelected::new(target, ()),
            selectable_proxy: ProxiedSelectable::new(target, ()),
            hovered_proxy: ProxiedHovered::new(target, ()),
        }
    }

    pub fn with_position_proxy_strategy(mut self, proxied_position_strategy: ProxiedPositionStrategy) -> Self {
        *self.position_proxy.state_mut() = proxied_position_strategy;
        self
    }
}

pub use definitions::ObjectType;

use super::{position::Position, ProxiedComponent, ProxiedPosition, ProxiedPositionStrategy};

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
        VectorEdge,
        VectorEndpoint,
        VectorCtrl,
    }
}
