use bevy::{
    app::{App, Plugin},
    ecs::{query::With, system::Resource, world::World},
    input::ButtonState,
    log::warn,
};
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_uid::Uid;

use crate::{
    api::scene::SceneApi,
    ecs::{ObjectType, Position},
    plugins::{
        effect::Effect,
        inspecting::Inspected,
        selected::{raycast::SelectableRaycaster, Hovered, Selectable, Selected, SelectedApi},
        undoredo::UndoRedoApi,
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
    PointerDown,
    MovingSelectedObjects(Vec<(Uid, Position)>),

    SelectingBounds,
}

pub fn handle_select_tool_input(
    world: &mut World,
    events: &Vec<InputMessage>,
    _effects: &mut [Effect],
) -> Result<(), anyhow::Error> {
    let mut state = world.resource::<SelectTool>().clone();

    for event in events {
        state = match (&state, event) {
            (SelectTool::PointerDown, InputMessage::DoubleClick { screen_pos, .. }) => {
                let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
                let top_hit = hits.top().map(|hit| (*hit.uid(), hit.object_type()));

                match top_hit {
                    None => {
                        let inspected_uid = world
                            .query_filtered::<&Uid, With<Inspected>>()
                            .get_single(world)
                            .ok();
                        if inspected_uid.is_some() {
                            SceneApi::uninspect(world).unwrap();
                        }
                        SelectTool::Default
                    }
                    Some((uid, ObjectType::Vector)) => {
                        SceneApi::inspect(world, uid).unwrap();
                        SelectTool::Default
                    }
                    Some((_, _)) => {
                        // Ignore.. for now
                        SelectTool::Default
                    }
                }
            }

            (SelectTool::Default, InputMessage::PointerMove { screen_pos, .. }) => {
                SelectedApi::unhover_all(world).unwrap();
                let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
                let top = hits.top();
                if let Some(hit) = top {
                    SelectedApi::set_object_hovered(world, *hit.uid(), Hovered::Unhovered).unwrap();
                }

                SelectTool::Default
            }

            (
                SelectTool::Default,
                InputMessage::PointerDown {
                    modifiers,
                    screen_pos,
                    ..
                },
            ) => {
                let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
                let top = hits.top();

                if let Some(hit) = top {
                    let target = *hit.uid();
                    if matches!(modifiers.shift, ButtonState::Pressed) {
                        SelectedApi::set_object_selected(world, target, Selected::Selected)?;
                    } else {
                        SelectedApi::deselect_all_set_object_selected(
                            world,
                            target,
                            Selected::Selected,
                        )?;
                    }
                }
                SelectTool::PointerDown
            }

            (
                SelectTool::PointerDown,
                InputMessage::DragStart {
                    screen_start_pos, ..
                },
            ) => {
                let selected = SelectedApi::query_selected_uids(world);
                let hits =
                    SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_start_pos);
                let top_hit = hits.top().map(|hit| (hit.uid(), hit.object_type()));

                warn!("top_hit: {top_hit:?}");
                let dragging_selected = top_hit.map_or(false, |(hit_uid, _)| {
                    selected.iter().any(|uid| *uid == *hit_uid)
                });
                warn!("dragging_selected: {dragging_selected:?}\n{selected:?}");
                if dragging_selected {
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

                    SelectTool::MovingSelectedObjects(original_positions)
                } else {
                    SelectTool::SelectingBounds
                }
            }

            (SelectTool::SelectingBounds, InputMessage::DragEnd { .. }) => SelectTool::Default,

            (SelectTool::PointerDown, InputMessage::PointerClick { screen_pos, .. }) => {
                let hits = SelectableRaycaster::raycast_uncached::<Selectable>(world, *screen_pos);
                let hit_uid = hits.top().map(|hit| hit.uid());
                let mut q_selected = world.query::<&Selected>();

                let hit_selected = hit_uid.map_or(false, |hit_uid| {
                    q_selected
                        .get(world, hit_uid.entity(world).unwrap())
                        .ok()
                        .map_or(false, |s| matches!(s, Selected::Selected))
                });

                let any_selected = q_selected
                    .iter(world)
                    .any(|s| matches!(s, Selected::Selected));

                if !hit_selected && any_selected {
                    SelectedApi::deselect_all(world)?;
                }
                SelectTool::Default
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
            }
        }
    }

    *world.resource_mut::<SelectTool>() = state;

    Ok(())
}
