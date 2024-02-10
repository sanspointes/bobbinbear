use anyhow::Ok;
use bevy::prelude::*;

use crate::{index::{Idx, WorldIndex}, serialise::{EncComponent, EncComponentTag, EncodableComponent}};


#[derive(Clone, Debug)]
pub enum BBChange {
    MutateComponent(MutateComponent),
    AddComponent(AddComponent),
    RemoveComponent(RemoveComponent),
    SpawnEntity(SpawnEntity),
    DespawnEntity(DespawnEntity),
    ChangeParent(ChangeParent),
    ChangeMulti(ChangeMulti),
}
impl BBChange {
    pub fn apply(self, world: &mut World) -> Result<BBChange, anyhow::Error> {
        let result = match self {
            Self::MutateComponent(change) => Self::MutateComponent(change.apply(world)?),
            Self::AddComponent(change) => Self::RemoveComponent(change.apply(world)?),
            Self::RemoveComponent(change) => Self::AddComponent(change.apply(world)?),
            Self::SpawnEntity(change) => Self::DespawnEntity(change.apply(world)?),
            Self::DespawnEntity(change) => Self::SpawnEntity(change.apply(world)?),
            Self::ChangeParent(change) => Self::ChangeParent(change.apply(world)?),
            Self::ChangeMulti(change) => Self::ChangeMulti(change.apply(world)?),
        };
        Ok(result)
    }
}

/// Mutate component BBChange.  Mutates a component on an entity
///
/// * `old_value`:
/// * `new_value`:
/// * `target`:
#[derive(Clone, Debug)]
pub struct MutateComponent {
    value: EncComponent,
    target: Idx,
}
impl MutateComponent {
    pub fn apply(mut self, world: &mut World) -> Result<MutateComponent, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let mut entity_mut = world.entity_mut(entity);
        self.value.swap_with_entity_world_mut(&mut entity_mut)?;
        Ok(MutateComponent {
            target: self.target,
            value: self.value,
        })
    }
}

/// AddComponent BBChange.
/// Adds a component to an entity
///
/// * `to_add`:
/// * `target`:
#[derive(Clone, Debug)]
pub struct AddComponent {
    to_add: EncComponent,
    target: Idx,
}
impl AddComponent {
    pub fn apply(self, world: &mut World) -> Result<RemoveComponent, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let mut entity_mut = world.entity_mut(entity);
        self.to_add.try_insert_into_entity_world_mut(&mut entity_mut)?;
        Ok(RemoveComponent {
            target: self.target,
            to_remove: self.to_add,
        })
    }
}

/// Removes a component from an entity.
///
/// * `to_remove`:
/// * `target`:
#[derive(Clone, Debug)]
pub struct RemoveComponent {
    to_remove: EncComponent,
    target: Idx,
}
impl RemoveComponent {
    pub fn apply(self, world: &mut World) -> Result<AddComponent, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let mut entity_mut = world.entity_mut(entity);
        self.to_remove.remove_from_entity_world_mut(&mut entity_mut)?;
        Ok(AddComponent {
            target: self.target,
            to_add: self.to_remove,
        })
    }
}

/// Removes a component from an entity.
///
/// * `to_remove`:
/// * `target`:
#[derive(Clone, Debug)]
pub struct SpawnEntity {
    parent_target: Option<Idx>,
    components: Vec<EncComponent>,
}

impl SpawnEntity {
    pub fn new(parent: Option<Idx>, components: Vec<EncComponent>) -> Self {
        Self {
            parent_target: parent,
            components,
        }
    }

    pub fn new_empty(parent: Option<Idx>) -> Self {
        Self {
            parent_target: parent,
            components: vec![],
        }
    }

    pub fn with_component<C: Component + EncodableComponent>(&mut self, component: C) -> Result<&mut Self, anyhow::Error> {
        let encoded = component.try_encode()?;
        self.components.push(encoded);
        Ok(self)
    }

    pub fn apply(self, world: &mut World) -> Result<DespawnEntity, anyhow::Error> {
        let idx = Idx::new();
        let mut entity_mut = world.spawn(idx);

        for comp in &self.components {
            comp.try_insert_into_entity_world_mut(&mut entity_mut)?;
        }
        let tags: Vec<_> = self.components.into_iter().map(|comp| comp.0).collect();

        Ok(DespawnEntity { target: idx, component_tags: tags, })
    }
}

/// Despawn Entity
///
/// * `to_remove`:
/// * `target`:
#[derive(Clone, Debug)]
pub struct DespawnEntity {
    target: Idx,
    component_tags: Vec<EncComponentTag>
}
impl DespawnEntity {
    pub fn apply(self, world: &mut World) -> Result<SpawnEntity, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let parent_target = world.get::<Parent>(entity).and_then(|p| world.get::<Idx>(**p).copied());

        let mut entity_mut = world.entity_mut(entity);
        let components: Vec<_> = self.component_tags.into_iter().map(|tag| {
            EncComponent::from_tag_and_entity_world_mut(tag, &mut entity_mut)
        }).collect();

        Ok(SpawnEntity { parent_target, components })
    }
}

/// Parent an entity to another or none
///
/// * `target`: The scene element we want to change
/// * `parent_target`: Id of the entity that we want `target` to be parented to
#[derive(Clone, Debug)]
pub struct ChangeParent {
    target: Idx,
    parent_target: Option<Idx>
}
impl ChangeParent {
    pub fn set_parent(target: Idx, parent_target: Idx) -> Self {
        Self {
            target,
            parent_target: Some(parent_target),
        }
    }
    pub fn remove_parent(target: Idx) -> Self {
        Self {
            target,
            parent_target: None,
        }
    }
    pub fn apply(self, world: &mut World) -> Result<ChangeParent, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let parent_target_entity = self.parent_target.and_then(|idx| world.idx(idx));
        let old_parent_target = world.get::<Parent>(entity).and_then(|p| world.get::<Idx>(**p).copied());

        let mut entity_mut = world.entity_mut(entity);
        if let Some(parent_target_entity) = parent_target_entity {
            entity_mut.set_parent(parent_target_entity);
        } else {
            entity_mut.remove_parent();
        }

        Ok(ChangeParent { target: self.target, parent_target: old_parent_target})
    }
}

/// 
///
/// * `changes`: 
#[derive(Clone, Debug)]
pub struct ChangeMulti {
    changes: Vec<BBChange>,
}
impl ChangeMulti {
    pub fn new(changes: Vec<BBChange>) -> Self {
        Self {
            changes,
        }
    }
    pub fn push(&mut self, change: impl Into<BBChange>) -> &mut Self {
        self.changes.push(change.into());
        self
    }
    pub fn extend(&mut self, changes: Vec<BBChange>) -> &mut Self {
        self.changes.extend(changes);
        self
    }
}
impl ChangeMulti {
    pub fn apply(self, world: &mut World) -> Result<ChangeMulti, anyhow::Error> {
        let inverse_changes: Result<Vec<_>, anyhow::Error> = self.changes.into_iter().map(|change| {
            change.apply(world)
        }).collect();

        Ok(ChangeMulti { changes: inverse_changes? })
    }
}
