use bevy::{ecs::prelude::*, log::warn, window::CursorMoved};
use bevy_mod_raycast::{deferred::{RaycastMethod, RaycastSource}, primitives::IntersectionData};
use bevy_spts_uid::Uid;

use crate::{ecs::ObjectType, plugins::viewport::BobbinViewport};

use super::Selectable;


pub fn sys_setup_selection_raycast(
    mut commands: Commands,
    q_camera: Query<Entity, With<BobbinViewport>>,
) {
    let e_camera = q_camera.single();
    commands
        .get_entity(e_camera)
        .expect("sys_setup_selection_raycast: Cannot get camera")
        .insert(RaycastSource::<Selectable>::default());
}

pub fn sys_selection_raycast_update_ray(
    mut q_raycast_source: Query<&mut RaycastSource<Selectable>>,
    mut ev_cursor_moved: EventReader<CursorMoved>,
) {
    let mut source = q_raycast_source.single_mut();
    for ev in ev_cursor_moved.read() {
        source.cast_method = RaycastMethod::Screenspace(ev.position);
    }
}

pub fn sys_selection_raycast_update_helper(
    q_raycast_source: Query<&RaycastSource<Selectable>>,
    mut res: ResMut<SelectableHits>,
    q_objects: Query<(&Uid, &ObjectType)>,
) {
    let hits = q_raycast_source.single().intersections();

    let select_hits: Vec<_> = hits.iter().filter_map(|(e, data)| {
        if let Ok((uid, ty)) = q_objects.get(*e) {
            Some(SelectableHit {
                entity: *e,
                uid: *uid,
                ty: *ty,
                data: data.clone(),
            })
        } else {
            None
        }
    }).collect();

    warn!("Hits {hits:?}");

    res.hits = select_hits;
}

#[derive(Clone, Debug)]
pub struct SelectableHit {
    pub entity: Entity,
    pub uid: Uid,
    pub ty: ObjectType,
    pub data: IntersectionData,
}

#[derive(Resource, Clone, Default, Debug)]
pub struct SelectableHits {
    pub hits: Vec<SelectableHit>,
}

impl SelectableHits {
    pub fn top(&self) -> Option<&SelectableHit> {
        self.hits.first()
    }
}

pub trait SelectableHitsWorldExt {
    fn selectable_hits(&self) -> &SelectableHits;
}

impl SelectableHitsWorldExt for World {
    fn selectable_hits(&self) -> &SelectableHits {
        self.resource::<SelectableHits>()
    }
}
