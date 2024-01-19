use crate::msgs::{effect::EffectMsg, MsgQue};

use super::{ToolHandler, ToolHandlerMessage};

pub struct NoopTool;

impl ToolHandler for NoopTool {
    fn setup(world: &mut bevy::prelude::World) {}
    fn handle_msg(
        world: &mut bevy::prelude::World,
        msg: &ToolHandlerMessage,
        responder: &mut MsgQue,
    ) {
    }
    fn handle_effects(world: &mut bevy::prelude::World, event: &EffectMsg) {}
}
