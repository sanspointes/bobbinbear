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

use self::position::Position;

pub mod core;
pub mod node;
pub mod position;

#[derive(Component, Reflect)]
#[reflect(Component)]
/// Marker component for an object that should not be visible in the frontend / editor.
pub struct InternalObject;

#[derive(Bundle, Reflect, Default)]
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
    position: Position,
    selected: Selected,

    transform: Transform,
    global_transform: GlobalTransform,

    visibility: Visibility,
    view_visibility: ViewVisibility,
    inherited_visibility: InheritedVisibility,
}

impl ObjectBundle {
    pub fn proxy_viewport(target: Uid) -> Self {
        Self {
            position: Position::ProxyViewport {
                target,
                target_world_position: Vec3::ZERO,
            },
            selected: Selected::Proxy { target, selected: false },
            ..Default::default()
        }
    }

    pub fn with_local_position(mut self, position: impl Into<Vec2>) -> Self {
        self.position = Position::Local(position.into());
        self
    }
}
