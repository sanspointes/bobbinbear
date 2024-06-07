use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::Events,
        schedule::{IntoSystemConfigs, SystemSet},
        world::World,
    },
};

use crate::{
    plugins::effect::{Effect, EffectQue},
    PosSet,
};

use self::{
    input::{BobbinInputPlugin, InputMessage},
    pen::{handle_pen_tool_input, sys_update_pen_tool_preview, PenToolPlugin},
    resource::ToolResource,
    select::{handle_select_tool_input, SelectToolPlugin},
};

pub use input::InputState;
pub use pen::{PenToolBuildingFromEndpointTag, PenToolBuildingVectorObjectTag};
pub use types::{ BobbinTool, BobbinCursor };

mod api;
mod input;
mod resource;
mod types;

// Tools
mod pen;
mod select;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToolSet;

pub struct BobbinToolsPlugin;

impl Plugin for BobbinToolsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ToolResource::default())
            .add_plugins((BobbinInputPlugin, SelectToolPlugin, PenToolPlugin))
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
        BobbinTool::Noop => {}
        BobbinTool::Select => {
            handle_select_tool_input(world, &input_events, &mut effects).unwrap();
        }
        BobbinTool::Pen => {
            handle_pen_tool_input(world, &input_events, &mut effects).unwrap();
        }
    }

    let effect_que = world.resource_mut::<EffectQue>();
    for effect in effects {
        effect_que.push_effect(effect);
    }
}
