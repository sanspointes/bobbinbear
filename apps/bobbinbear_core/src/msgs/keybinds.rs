use bevy::{input::ButtonState, prelude::*};

use crate::{plugins::input_plugin::InputMessage, types::BBTool};

use super::{ToolMessage, cmds::CmdMsg, MsgResponder};

pub fn msg_handler_keybinds(
    mut _world: &mut World,
    message: &InputMessage,
    responder: &mut MsgResponder,
) {
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
                responder.respond(ToolMessage::PushTool(BBTool::Grab));
            }
            (ButtonState::Released, KeyCode::Space, _, _) => {
                responder.respond(ToolMessage::ResetToRootTool);
            }
            (ButtonState::Released, KeyCode::Key1, _, _) => {
                responder.respond(ToolMessage::SwitchTool(BBTool::Select));
            }
            (ButtonState::Released, KeyCode::Key2, _, _) => {
                responder.respond(ToolMessage::SwitchTool(BBTool::Box));
            }
            (ButtonState::Released, KeyCode::Key3, _, _) => {
                // msg_writer.send(ToolMessage::SwitchTool(BBTool::Pen).into());
            }
            (ButtonState::Released, KeyCode::Z, ButtonState::Pressed, ButtonState::Released) => {
                responder.respond(CmdMsg::Undo);
            }
            (ButtonState::Released, KeyCode::Z, ButtonState::Pressed, ButtonState::Pressed) => {
                responder.respond(CmdMsg::Redo);
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
        responder.respond(ToolMessage::Input(*message));
    }
}
