use bevy::ecs::{event::Events, world::World};
use bevy_wasm_api::bevy_wasm_api;

use wasm_bindgen::prelude::*;

use crate::plugins::effect::Effect;

use super::{resource::ToolResource, types::BobbinTool};

#[derive(Clone, Copy)]
pub struct ToolApi;

#[allow(dead_code)]
#[bevy_wasm_api]
impl ToolApi {
    fn deactivate_tool(world: &mut World, tool: BobbinTool) {
        match tool {
            BobbinTool::Noop => {},
            BobbinTool::Select => {},
            BobbinTool::Pen => {},
        }
    }

    fn activate_tool(world: &mut World, tool: BobbinTool) {
        match tool {
            BobbinTool::Noop => {},
            BobbinTool::Select => {},
            BobbinTool::Pen => {},
        }
    }

    pub fn set_base_tool(world: &mut World, tool: BobbinTool) {
        let current_tool = world.resource::<ToolResource>().get_current_tool();

        if tool == current_tool {
            return;
        }

        Self::deactivate_tool(world, current_tool);

        world.resource_mut::<ToolResource>().set_base_tool(tool);
        Self::activate_tool(world, tool);

        world.resource_mut::<Events<Effect>>().send(Effect::ToolChanged(tool));
    }
}
