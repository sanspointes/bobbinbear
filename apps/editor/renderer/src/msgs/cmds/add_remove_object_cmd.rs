use std::fmt::{Debug, Display};

use bevy::{
    ecs::{entity::EntityMap, query::QueryEntityError, world::EntityMut},
    prelude::*,
    scene::SceneSpawnError,
};
use thiserror::Error;

use crate::{
    components::bbid::{BBId, BBIdUtils},
    utils::reflect_shims::{patch_world_subhierarchy_for_reflection, patch_world_subhierarchy_for_playback},
};

use super::{Cmd, CmdError};

#[derive(Error, Debug)]
pub enum AddRemoveObjectError {
    #[error("Cannot find entity {0:?}.")]
    CantFindEntity(Entity),
    #[error("Cannot find entity via bbid {0:?}.")]
    CantFindTarget(BBId),
    #[error("Cannot find entity parent via bbid {0:?}.")]
    CantFindTargetParent(BBId),
    #[error("Error spawning scene. {0:?}")]
    SpawnError(SceneSpawnError),
    #[error("Error querying entity. {0:?}")]
    QueryEntityError(QueryEntityError),
}
impl From<AddRemoveObjectError> for CmdError {
    fn from(value: AddRemoveObjectError) -> Self {
        CmdError::CustomError(Box::new(value))
    }
}

pub struct AddObjectCmd {
    entity_bbid: BBId,
    parent: Option<BBId>,
    scene: Option<DynamicScene>,
}

impl Display for AddObjectCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent_string = match self.parent {
            Some(parent) => parent.to_string(),
            None => String::from("Scene Root"),
        };
        write!(
            f,
            "Adding object with BBID {} to {}",
            self.entity_bbid, parent_string
        )
    }
}

impl Debug for AddObjectCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddObjectCmd")
            .field("entity_bbid", &self.entity_bbid)
            .field("parent", &self.parent)
            .field(
                "scene",
                &self.scene.as_ref().map(|_| String::from("Dynamic Scene")),
            )
            .finish()
    }
}

impl AddObjectCmd {
    pub fn from_builder<F: FnMut(&mut EntityMut<'_>)>(
        mut world: &mut World,
        parent: Option<BBId>,
        mut builder: F,
    ) -> Result<Self, AddRemoveObjectError> {
        let mut entity_mut = world.spawn_empty();
        builder(&mut entity_mut);

        let id = entity_mut.id();
        AddObjectCmd::from_entity(&mut world, id, parent)
    }

    pub fn from_entity(
        world: &mut World,
        entity: Entity,
        parent: Option<BBId>,
    ) -> Result<Self, AddRemoveObjectError> {
        let entity_bbid = world
            .get::<BBId>(entity)
            .cloned()
            .ok_or(AddRemoveObjectError::CantFindEntity(entity))?;

        let entities = patch_world_subhierarchy_for_reflection(world, entity)
            .map_err(|err| AddRemoveObjectError::QueryEntityError(err))?;

        for e in entities.iter() {
        }

        let mut builder = DynamicSceneBuilder::from_world(&world);
        builder.extract_entities(entities.into_iter());
        let dynamic_scene = builder.build();

        let type_registry = world.resource::<AppTypeRegistry>();
        println!("dynamic_scene as serialize_ron: {:?}", dynamic_scene.serialize_ron(&type_registry));

        world.entity_mut(entity).despawn_recursive();

        Ok(Self {
            entity_bbid,
            parent,
            scene: Some(dynamic_scene),
        })
    }
}

impl Cmd for AddObjectCmd {
    fn name(&self) -> &str {
        "AddObjectCmd"
    }

    fn execute(&mut self, world: &mut World) -> Result<(), CmdError> {
        // Write scene into world
        let type_registry = world.resource::<AppTypeRegistry>().clone();

        let mut entity_map = EntityMap::default();
        let scene = self.scene.take().ok_or(CmdError::DoubleExecute)?;
        scene
            .write_to_world_with(world, &mut entity_map, &type_registry)
            .map_err(|err| CmdError::from(AddRemoveObjectError::SpawnError(err)))?;

        // Get Target and parent entity to parent to correct object (if necessary)
        let target_entity: Entity =
            world
                .get_entity_id_by_bbid(self.entity_bbid)
                .ok_or(CmdError::from(AddRemoveObjectError::CantFindTarget(
                    self.entity_bbid,
                )))?;

        patch_world_subhierarchy_for_playback(world, target_entity)
            .map_err(|err| CmdError::CustomError(Box::new(err)))?;

        let maybe_parent = match self.parent {
            Some(parent) => match world.get_entity_id_by_bbid(parent) {
                Some(parent) => Some(parent),
                None => return Err(AddRemoveObjectError::CantFindTargetParent(parent).into()),
            },
            None => None,
        };

        // Parent the object if necessary
        if let Some(parent) = maybe_parent {
            world.entity_mut(target_entity).set_parent(parent);
        }

        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CmdError> {
        if self.scene.is_some() {
            return Err(CmdError::DoubleUndo);
        }

        // Get entity of app id.
        let target_entity = world
            .get_entity_id_by_bbid(self.entity_bbid)
            .ok_or(CmdError::from(AddRemoveObjectError::CantFindTarget(
                self.entity_bbid,
            )))?;

        let entities = patch_world_subhierarchy_for_reflection(world, target_entity)
            .map_err(|err| CmdError::CustomError(Box::new(err)))?;
        let mut builder = DynamicSceneBuilder::from_world(world);
        builder.extract_entities(entities.into_iter());
        let scene = builder.build();

        self.scene = Some(scene);

        world.entity_mut(target_entity).despawn_recursive();

        Ok(())
    }
}
