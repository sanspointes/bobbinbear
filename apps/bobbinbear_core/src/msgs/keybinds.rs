use bevy::{input::ButtonState, prelude::*};

use crate::{plugins::input_plugin::InputMessage, types::BBTool};

use super::{ToolMessage, cmds::CmdMsg, MsgQue};

pub fn msg_handler_keybinds(
    mut _world: &mut World,
    message: &InputMessage,
    responder: &mut MsgQue,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("msg_handler_keybinds").entered();

    let mut should_pass_through = false;
    match message {
        InputMessage::Keyboard {
            pressed,
            key,
            modifiers,
        } => match (pressed, key, modifiers.command, modifiers.shift) {
            // Click to drag around viewport with space key pressed
            (ButtonState::Pressed, KeyCode::Space, _, _) => {
                responder.push_internal(ToolMessage::PushTool(BBTool::Grab));
            }
            (ButtonState::Released, KeyCode::Space, _, _) => {
                responder.push_internal(ToolMessage::ResetToRootTool);
            }
            (ButtonState::Released, KeyCode::Key1, _, _) => {
                responder.push_internal(ToolMessage::SwitchTool(BBTool::Select));
            }
            (ButtonState::Released, KeyCode::Key2, _, _) => {
                responder.push_internal(ToolMessage::SwitchTool(BBTool::Box));
            }
            (ButtonState::Released, KeyCode::Key0, _, _) => {
                responder.push_internal(ToolMessage::SwitchTool(BBTool::Noop));
            }
            (ButtonState::Released, KeyCode::Z, ButtonState::Pressed, ButtonState::Released) => {
                responder.push_internal(CmdMsg::Undo);
            }
            (ButtonState::Released, KeyCode::Z, ButtonState::Pressed, ButtonState::Pressed) => {
                responder.push_internal(CmdMsg::Redo);
            }
            (_, _, _, _) => {
                should_pass_through = true;
            }
        },
        _ => {
            should_pass_through = true;
        }
    }

    if should_pass_through {
        responder.push_internal(ToolMessage::Input(*message));
    }
}
