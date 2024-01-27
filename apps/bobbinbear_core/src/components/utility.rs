// use std::sync::Arc;

// use bevy::prelude::*;

// type Callback = dyn Send + Sync + 'static + Fn(&mut World, Entity);
// type ArcedCallback = Arc<Callback>;
//
// #[derive(Component, Clone)]
// pub struct OnMoveCommand(ArcedCallback);
// /// Runs a one time function whenever the move command is executed on this entity.
// impl OnMoveCommand {
//     pub fn new(callback: impl Send + Sync + 'static + Fn(&mut World, Entity)) -> Self {
//         Self(Arc::new(callback))
//     }
//
//     pub fn run(&self, world: &mut World, entity: Entity) {
//         (self.0)(world, entity);
//     }
// }
