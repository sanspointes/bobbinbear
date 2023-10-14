use std::collections::VecDeque;

use bevy::{input::ButtonState, prelude::*};

use crate::{plugins::input_plugin::InputMessage, types::BBTool};

use super::{Message, ToolMessage, cmds::CmdMsg};

pub fn msg_handler_keybinds(
    mut _world: &mut World,
    message: &InputMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut should_pass_through = false;
    match message {
        InputMessage::Keyboard {
            pressed,
            key,
            modifiers,
        } => match (pressed, key, modifiers.command, modifiers.shift) {
            // Click to drag around viewport with space key pressed
            (ButtonState::Pressed, KeyCode::Space, _, _) => {
                responses.push_back(ToolMessage::PushTool(BBTool::Grab).into());
            }
            (ButtonState::Released, KeyCode::Space, _, _) => {
                responses.push_back(ToolMessage::ResetToRootTool.into());
            }
            (ButtonState::Released, KeyCode::Key1, _, _) => {
                responses.push_back(ToolMessage::SwitchTool(BBTool::Select).into());
            }
            (ButtonState::Released, KeyCode::Key2, _, _) => {
                responses.push_back(ToolMessage::SwitchTool(BBTool::Box).into());
            }
            (ButtonState::Released, KeyCode::Key3, _, _) => {
                // msg_writer.send(ToolMessage::SwitchTool(BBTool::Pen).into());
            }
            (ButtonState::Released, KeyCode::Z, ButtonState::Pressed, ButtonState::Released) => {
                responses.push_back(CmdMsg::UndoCmd.into());
            }
            (ButtonState::Released, KeyCode::Z, ButtonState::Pressed, ButtonState::Pressed) => {
                responses.push_back(CmdMsg::RedoCmd.into());
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
        responses.push_back(ToolMessage::Input(*message).into());
    }
}
