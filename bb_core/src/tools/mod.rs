use bevy::{
    app::{App, Plugin, Update},
    ecs::{event::Events, schedule::{IntoSystemConfigs, SystemSet}, world::World},
};

use crate::plugins::effect::{Effect, EffectQue};

use self::{input::{BobbinInputPlugin, InputMessage}, resource::ToolResource, select::{handle_select_tool_input, SelectToolPlugin}, types::BobbinTool};

mod api;
mod input;
mod resource;
mod types;

// Tools
mod select;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToolSet;

pub struct BobbinToolsPlugin;

impl Plugin for BobbinToolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((BobbinInputPlugin, SelectToolPlugin))
            .insert_resource(ToolResource::default())
            .add_systems(Update, sys_handle_tool_inputs.in_set(ToolSet));
    }
}

pub fn sys_handle_tool_inputs(world: &mut World) {
    let input_events: Vec<_> = world
        .get_resource_mut::<Events<InputMessage>>()
        .unwrap()
        .drain()
        .collect();

    let mut effects: Vec<Effect> = vec![];

    let curr_tool = world.resource::<ToolResource>().get_current_tool();
    match curr_tool {
        BobbinTool::Noop => {},
        BobbinTool::Select => {
            handle_select_tool_input(world, &input_events, &mut effects);
        }
    }

    let effect_que = world.resource_mut::<EffectQue>();
    for effect in effects {
        effect_que.push_effect(effect);
    }
}
