use bevy::prelude::*;

use crate::messages::input::InputMessage;

use super::sys_tool_handle_messages;

#[derive(Event, Debug, Clone)]
pub enum SelectToolMsg {
    OnActivate,
    OnDeactivate,
    Input(InputMessage),
}

pub struct SelectToolPlugin;

impl Plugin for SelectToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectToolMsg>()
            .add_systems(PreUpdate, sys_tool_grab_msg_handler.after(sys_tool_handle_messages))
        ;
    }
}

pub fn sys_tool_grab_msg_handler(
    
) {

}
