use bevy::{ecs::{change_detection::DetectChangesMut, schedule::{NextState, State}, world::World}, log::warn};
use bevy_spts_changeset::builder::Changeset;
use bevy_wasm_api::bevy_wasm_api;

use wasm_bindgen::prelude::*;

use crate::plugins::{
    effect::{Effect, EffectQue},
    undoredo::UndoRedoApi,
};

use super::{
    pen::{activate_pen_tool, deactivate_pen_tool},
    resource::ToolResource,
    select::{activate_select_tool, deactivate_select_tool},
    types::BobbinTool,
};

#[derive(Clone, Copy)]
pub struct ToolApi;

#[allow(dead_code)]
#[bevy_wasm_api]
impl ToolApi {
    pub fn set_base_tool(world: &mut World, tool: BobbinTool) {

        let changeset = Changeset::scoped_commands(world, |world, commands| {
            world.resource_scope::<EffectQue, ()>(|world, mut effect_que| {
                let prev_tool = world.resource::<State<BobbinTool>>();
                warn!("Previous tool {prev_tool:?}");
                match prev_tool.get() {
                    BobbinTool::Noop => {}
                    BobbinTool::Select => deactivate_select_tool(world, commands, &mut effect_que),
                    BobbinTool::Pen => deactivate_pen_tool(world, commands, &mut effect_que),
                };

                warn!("Next tool {tool:?}");
                world.resource_mut::<ToolResource>().set_base_tool(tool);
                let mut v = world.resource_mut::<NextState<BobbinTool>>();
                v.set(tool);

                match tool {
                    BobbinTool::Noop => {}
                    BobbinTool::Select => activate_select_tool(world, commands, &mut effect_que),
                    BobbinTool::Pen => activate_pen_tool(world, commands, &mut effect_que),
                };

                effect_que.push_effect(Effect::ToolChanged(tool))
            })
        });

        UndoRedoApi::execute(world, changeset);
    }
}
