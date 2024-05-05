use bevy::{
    app::{App, Plugin},
    ecs::{system::Resource, world::World},
    log::info,
};
use bevy_spts_uid::Uid;

use crate::plugins::{
    effect::Effect,
    selected::{raycast::{SelectableHit, SelectableHitsWorldExt}, Hovered, SelectedApi},
};

use super::input::InputMessage;

pub struct SelectToolPlugin;

impl Plugin for SelectToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectTool::default());
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub enum SelectTool {
    #[default]
    Default,
    Hovering(Uid),
    PointerDownOnSelected(Uid),
    PointerDownOnNothing,
}

pub fn handle_select_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut [Effect],
) -> Result<(), anyhow::Error>{
    let mut state = *world.resource::<SelectTool>();

    for event in events {
        state = match (state, event) {
            (SelectTool::Default, InputMessage::PointerDown { .. }) => state,
            (SelectTool::Default, InputMessage::PointerMove { .. }) => {
                let top = world.selectable_hits().top();
                let target = top.map(|hit| hit.uid);
                if let Some(target) = target {
                    SelectedApi::set_object_hovered(world, target, Hovered::Hovered)?;
                    SelectTool::Hovering(target)
                } else {
                    SelectTool::Default
                }
            }
            (SelectTool::Hovering(prev_hovered), InputMessage::PointerMove { .. }) => {
                let top = world.selectable_hits().top();
                if let Some(SelectableHit { entity, uid, .. }) = top {
                    let target = *uid;
                    let current_value = *world.get::<Hovered>(*entity).unwrap();

                    if *uid != prev_hovered {
                        SelectedApi::set_object_hovered(world, prev_hovered, Hovered::Unhovered)?;
                    }
                    if !matches!(current_value, Hovered::Hovered) {
                        SelectedApi::set_object_hovered(world, target, Hovered::Hovered)?;
                    }
                    SelectTool::Hovering(target)
                } else {
                    SelectedApi::set_object_hovered(world, prev_hovered, Hovered::Unhovered)?;
                    SelectTool::Default
                }
            }
            _ => state,
        }
    }

    *world.resource_mut::<SelectTool>() = state;

    Ok(())
}
