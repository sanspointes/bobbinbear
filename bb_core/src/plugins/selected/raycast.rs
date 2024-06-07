use bevy::{
    ecs::{prelude::*, system::SystemState},
    math::Vec2,
    reflect::TypePath,
    render::camera::Camera,
    transform::components::GlobalTransform,
    utils::smallvec::SmallVec,
    window::{PrimaryWindow, Window},
};

use bevy_mod_raycast::{
    deferred::RaycastMesh,
    immediate::{Raycast, RaycastSettings, RaycastVisibility},
    primitives::{ray_from_screenspace, IntersectionData},
};
use bevy_spts_uid::{Uid, UidRegistry};

use crate::{
    ecs::{ObjectType, ProxiedUid},
    plugins::viewport::BobbinViewport,
};

use super::Selectable;

#[derive(Debug, Clone)]
pub struct SelectableHit(pub Entity, pub Uid, pub ObjectType, pub IntersectionData);

impl SelectableHit {
    pub fn new(entity: Entity, uid: Uid, object_type: ObjectType, data: IntersectionData) -> Self {
        Self(entity, uid, object_type, data)
    }
    pub fn entity(&self) -> Entity {
        self.0
    }
    pub fn uid(&self) -> &Uid {
        &self.1
    }
    pub fn object_type(&self) -> ObjectType {
        self.2
    }
    pub fn intersection_data(&self) -> &IntersectionData {
        &self.3
    }
}

#[derive(Debug, Clone)]
pub struct SelectableHits(SmallVec<[SelectableHit; 2]>);

impl FromIterator<SelectableHit> for SelectableHits {
    fn from_iter<T: IntoIterator<Item = SelectableHit>>(iter: T) -> Self {
        let data: SmallVec::<[SelectableHit; 2]> = SmallVec::from_iter(iter);
        SelectableHits(data)
    }
}

impl SelectableHits {
    pub fn top(&self) -> Option<&SelectableHit> {
        self.0.first()
    }

    pub fn top_if_object_type(&self, _object_type: ObjectType) -> Option<&SelectableHit> {
        self.top()
            .filter(|hit| matches!(hit.object_type(), _object_type))
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct SelectableRaycaster {
    pub hits: Vec<(Vec2, SmallVec<[SelectableHit; 2]>)>,
}

impl SelectableRaycaster {
    #[allow(dead_code)]
    /// Keeping this hear incase we need immediate mode raycasting.
    fn raycast_intersection_data<T: TypePath + Send + Sync + 'static>(
        world: &mut World,
        screen_pos: Vec2,
    ) -> SmallVec<[(Entity, IntersectionData); 2]> {
        let mut system_state = SystemState::<(
            Raycast,
            Query<(&Camera, &GlobalTransform), With<BobbinViewport>>,
            Query<&Window, With<PrimaryWindow>>,
            Query<&RaycastMesh<T>>,
            Query<&Selectable>,
        )>::new(world);
        let (mut rc, q_camera, q_window, q_filter, q_selectable) = system_state.get_mut(world);

        let (camera, camera_global_transform) = q_camera.single();
        let window = q_window.single();

        let screen_pos = Vec2::new(screen_pos.x, window.resolution.height() - screen_pos.y);
        let ray = ray_from_screenspace(screen_pos, camera, camera_global_transform, window);

        let Some(ray) = ray else {
            return SmallVec::new();
        };

        let filter = |entity| {
            q_filter.contains(entity)
                && q_selectable.get(entity).map_or(false, |selectable| {
                    matches!(selectable, Selectable::Default)
                })
        };
        let settings = RaycastSettings::default()
            .with_filter(&filter)
            .with_early_exit_test(&|_| true)
            .with_visibility(RaycastVisibility::MustBeVisible);
        SmallVec::from_iter(rc.cast_ray(ray, &settings).iter().cloned())
    }

    pub fn raycast_uncached<'w, T: TypePath + Send + Sync + 'static>(
        world: &'w mut World,
        screen_pos: Vec2,
    ) -> SelectableHits {
        // let selectable_hits: &'w SelectableHits = world.get_resource::<SelectableHits>().unwrap();
        // if let Some((_, hits)) = selectable_hits.hits.iter().find(|(pos, _)| *pos == screen_pos) {
        //     return hits;
        // }
        //
        let intersections = Self::raycast_intersection_data::<T>(world, screen_pos);

        let mut system_state = SystemState::<(
            Res<UidRegistry>,
            Query<(&Uid, Option<&ProxiedUid>)>,
            Query<&ObjectType>,
        )>::new(world);
        let (uid_registry, q_uid, q_object_type) = system_state.get(world);

        let hits: SelectableHits = intersections
            .into_iter()
            .map(|(entity, data)| {
                let mut curr_entity = Some(entity);
                let mut curr_uid = None;

                while let Some((uid, maybe_proxy)) = curr_entity.and_then(|e| q_uid.get(e).ok()) {
                    curr_uid = Some(uid);
                    if let Some(proxy) = maybe_proxy {
                        curr_entity = uid_registry.get_entity(*proxy.target()).ok();
                    } else {
                        curr_entity = None;
                    }
                }

                let ty = q_object_type.get(entity).unwrap();
                let uid = curr_uid.unwrap();
                SelectableHit(entity, *uid, *ty, data)
            })
            .collect();
        //
        // let mut selectable_hits = world.get_resource_mut::<SelectableHits>().unwrap();
        // selectable_hits.hits.push((screen_pos, hits));
        //
        // selectable_hits.hits[selectable_hits.hits.len() - 1].1
        hits
    }
}

pub trait SelectableHitsWorldExt {
    fn selectable_hits(&self) -> &SelectableRaycaster;
}

impl SelectableHitsWorldExt for World {
    fn selectable_hits(&self) -> &SelectableRaycaster {
        self.resource::<SelectableRaycaster>()
    }
}
