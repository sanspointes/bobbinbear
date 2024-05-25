use bevy::{
    app::{App, Plugin},
    ecs::{system::Resource, world::World},
    input::ButtonState,
    log::warn,
};
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_uid::Uid;

use crate::{
    ecs::Position,
    plugins::{
        effect::Effect,
        selected::{
            raycast::{SelectableHit, SelectableHitsWorldExt},
            Hovered, Selected, SelectedApi,
        },
        undoredo::UndoRedoApi,
    },
};

use super::input::InputMessage;

pub struct PenToolPlugin;

impl Plugin for PenToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PenTool::default());
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub enum PenTool {
    #[default]
    Default,

    HoveringEdge(Uid),
}

pub fn handle_pen_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut [Effect],
) -> Result<(), anyhow::Error> {
    let mut state = world.resource::<PenTool>().clone();

    for event in events {
        state = match (&state, event) {
            (PenTool::Default, InputMessage::PointerMove { .. }) => {
                let top = world.selectable_hits().top();
                if let Some(top) = top {
                    
                }
                todo!();
            }
            (state, ev) => {
                warn!("PenTool: Unhandled state/ev\n\tstate: {state:?}\n\tev: {ev:?}");
                state.clone()
            }
        }
    }

    *world.resource_mut::<PenTool>() = state;

    Ok(())
}
