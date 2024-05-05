use bevy::{
    app::{App, Plugin},
    ecs::{system::Resource, world::World},
    log::{info, warn},
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
        }, undoredo::UndoRedoApi,
    },
};

use super::input::InputMessage;

pub struct SelectToolPlugin;

impl Plugin for SelectToolPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectTool::default());
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub enum SelectTool {
    #[default]
    Default,

    Hovering(Uid),

    PointerDownOnObject(Uid),
    PointerDownOnNothing,

    MovingSelectedObjects(Vec<(Uid, Position)>),
}

pub fn handle_select_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut [Effect],
) -> Result<(), anyhow::Error> {
    let mut state = world.resource::<SelectTool>().clone();

    for event in events {
        state = match (&state, event) {
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

                    if uid != prev_hovered {
                        SelectedApi::set_object_hovered(world, *prev_hovered, Hovered::Unhovered)?;
                    }
                    if !matches!(current_value, Hovered::Hovered) {
                        SelectedApi::set_object_hovered(world, target, Hovered::Hovered)?;
                    }
                    SelectTool::Hovering(target)
                } else {
                    SelectedApi::set_object_hovered(world, *prev_hovered, Hovered::Unhovered)?;
                    SelectTool::Default
                }
            }
            (SelectTool::Default, InputMessage::PointerDown { .. })
            | (SelectTool::Hovering(_), InputMessage::PointerDown { .. }) => {
                let top = world.selectable_hits().top();
                if let Some(SelectableHit { uid, .. }) = top {
                    let target = *uid;
                    SelectedApi::deselect_all_set_object_selected(
                        world,
                        target,
                        Selected::Selected,
                    )?;
                    SelectTool::PointerDownOnObject(target)
                } else {
                    SelectTool::PointerDownOnNothing
                }
            }
            (SelectTool::PointerDownOnObject(uid), InputMessage::PointerClick { .. }) => {
                SelectTool::Hovering(*uid)
            }
            (SelectTool::PointerDownOnNothing, InputMessage::PointerClick { .. }) => {
                let any_selected = world
                    .query::<&Selected>()
                    .iter(world)
                    .any(|s| matches!(s, Selected::Selected));
                if any_selected {
                    SelectedApi::deselect_all(world)?;
                }
                SelectTool::Default
            }

            (
                SelectTool::PointerDownOnObject(_),
                InputMessage::DragStart {
                    world_delta_pos, ..
                },
            ) => {
                let original_positions: Vec<_> = world
                    .query::<(&Uid, &Position, &Selected)>()
                    .iter(world)
                    .filter_map(|(uid, pos, selected)| {
                        if matches!(selected, Selected::Selected) {
                            Some((*uid, *pos))
                        } else {
                            None
                        }
                    })
                    .collect();

                let mut builder = world.changeset();

                for (target, position) in &original_positions {
                    let next_position = Position(position.0 + *world_delta_pos);
                    builder.entity(*target).apply(next_position);
                }

                let changeset = builder.build();

                UndoRedoApi::execute(world, changeset)?;

                SelectTool::MovingSelectedObjects(original_positions)
            }

            (
                SelectTool::MovingSelectedObjects(original_positions),
                InputMessage::DragMove {
                    world_delta_pos, ..
                },
            ) => {
                let mut builder = world.changeset();

                for (target, position) in original_positions {
                    let next_position = Position(position.0 + *world_delta_pos);
                    builder.entity(*target).apply(next_position);
                }

                let changeset = builder.build();

                UndoRedoApi::execute(world, changeset)?;

                SelectTool::MovingSelectedObjects(original_positions.to_vec())
            }
            (
                SelectTool::MovingSelectedObjects(original_positions),
                InputMessage::DragEnd {
                    world_delta_pos, ..
                },
            ) => {
                let mut builder = world.changeset();

                for (target, position) in original_positions {
                    let next_position = Position(position.0 + *world_delta_pos);
                    builder.entity(*target).apply(next_position);
                }

                let changeset = builder.build();

                UndoRedoApi::execute(world, changeset)?;

                SelectTool::Default
            }
            (state, ev) => {
                warn!("SelectTool: Unhandled state/ev\n\tstate: {state:?}\n\tev: {ev:?}");
                state.clone()
            },
        }
    }

    *world.resource_mut::<SelectTool>() = state;

    Ok(())
}
