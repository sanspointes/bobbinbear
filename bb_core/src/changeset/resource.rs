use bevy::prelude::*;

use super::primitives::BBChange;
#[allow(unused)]
#[allow(dead_code)]

#[derive(Clone, Debug, Default, Resource)]
pub struct ChangesetResource {
    pub undo: Vec<BBChange>,
    pub redo: Vec<BBChange>,
}

pub struct ChangesetPlugin;

impl Plugin for ChangesetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChangesetResource::default());
    }
}

pub fn execute_change(world: &mut World, change: impl Into<BBChange>) -> Result<(), anyhow::Error> {
    let c: BBChange = change.into();
    let inverse = c.apply(world)?;
    let mut changeset = world.resource_mut::<ChangesetResource>();
    changeset.undo.push(inverse);
    Ok(())
}

pub fn undo_change(world: &mut World) -> Result<(), anyhow::Error> {
    let mut changeset = world.resource_mut::<ChangesetResource>();
    let change = changeset.undo.pop();
    let Some(change) = change else {
        return Ok(());
    };

    let inverse = change.apply(world)?;
    let mut changeset = world.resource_mut::<ChangesetResource>();
    changeset.redo.push(inverse);

    Ok(())
}

pub fn redo_change(world: &mut World) -> Result<(), anyhow::Error> {
    let mut changeset = world.resource_mut::<ChangesetResource>();
    let change = changeset.redo.pop();
    let Some(change) = change else {
        return Ok(());
    };

    let inverse = change.apply(world)?;
    let mut changeset = world.resource_mut::<ChangesetResource>();
    changeset.undo.push(inverse);

    Ok(())
}
