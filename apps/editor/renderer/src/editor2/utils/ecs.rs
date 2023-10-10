use bevy::{
    ecs::{system::EntityCommands, world::EntityMut},
    prelude::*,
};
///
/// WORLD ACCESSOR,
///

/// LIFETIME: 'a represents the lifetime of the World or Commands object
/// Wraps Commands or World to provide the same API.
pub enum WorldAccessor<'a> {
    World(&'a mut World),
    Commands(&'a mut Commands<'a, 'a>),
}

impl<'a> WorldAccessor<'a> {
    pub fn from_world(world: &'a mut World) -> Self {
        WorldAccessor::World(world)
    }

    pub fn from_commands(commands: &'a mut Commands<'a, 'a>) -> Self {
        WorldAccessor::Commands(commands)
    }
}

impl<'s> WorldAccessor<'s> {
    pub fn spawn<'b, T>(&'b mut self, bundle: T) -> EntityBuilder<'b, 'b, 'b>
    where
        T: Bundle,
        'b: 's
    {
        match self {
            WorldAccessor::World(world) => {
                let entity = world.spawn(bundle);
                EntityBuilder::Entity(entity)
            },
            WorldAccessor::Commands(commands) => {
                let entity = commands.spawn(bundle);
                EntityBuilder::Commands(entity)
            },
        }
    }

    pub fn entity_mut<'b>(&'b mut self, entity: Entity) -> EntityBuilder<'b, 'b, 'b>
    where 'b : 's
    {
        match self {
            WorldAccessor::World(world) => {
                let entity = world.entity_mut(entity);
                EntityBuilder::Entity(entity)
            },
            WorldAccessor::Commands(commands) => {
                let entity = commands.entity(entity);
                EntityBuilder::Commands(entity)
            },

        }
    }
}

///
/// WORLD ACCESSOR,
///

/// Wraps EntityCommands or EntityMut (Provided by entity or world) to use the same API.
pub enum EntityBuilder<'a, 'b, 'c> {
    Entity(EntityMut<'a>),
    Commands(EntityCommands<'a, 'b, 'c>),
}

impl<'a> EntityBuilder<'a, 'a, 'a> {
    pub fn insert<'b, T: Bundle>(&'a mut self, component: T) -> &mut Self {
        match self {
            EntityBuilder::Entity(entity) => {
                entity.insert(component);
            },
            EntityBuilder::Commands(entity) => {
                entity.insert(component);
            },
        }
        self
    }

    pub fn id(&self) -> Entity {
        match self {
            EntityBuilder::Commands(commands) => {
                commands.id()
            }
            EntityBuilder::Entity(entity) => {
                entity.id()
            }
        }
    }

    pub fn push_children<'b>(&'a mut self, children: &[Entity]) -> &mut Self {
        match self {
            EntityBuilder::Commands(commands) => {
                commands.push_children(children);
            }
            EntityBuilder::Entity(entity) => {
                entity.push_children(children);
            }
        }
        self
    }
}
// pub enum EntityBuilder<'c> {
//     Commands(EntityCommands<'c, 'c, 'c>),
//     Entity(EntityMut<'c>),
// }
//
// impl<'c> EntityBuilder<'c> {
//     pub fn insert<T>(&'c mut self, bundle: T) -> &mut Self
//     where
//         T: Bundle,
//     {
//         match self {
//             EntityBuilder::Commands(commands) => {
//                 commands.insert(bundle);
//                 self
//             }
//             EntityBuilder::Entity(entity) => {
//                 entity.insert(bundle);
//                 self
//             }
//         }
//     }
//
//     pub fn id(&self) -> Entity {
//         match self {
//             EntityBuilder::Commands(commands) => {
//                 commands.id()
//             }
//             EntityBuilder::Entity(entity) => {
//                 entity.id()
//             }
//         }
//     } 
//
//     pub fn push_children(&mut self, children: &[Entity]) -> &mut Self {
//         match self {
//             EntityBuilder::Commands(commands) => {
//                 commands.push_children(children);
//                 self
//             }
//             EntityBuilder::Entity(entity) => {
//                 entity.push_children(children);
//                 self
//             }
//         }
//     }
// }
