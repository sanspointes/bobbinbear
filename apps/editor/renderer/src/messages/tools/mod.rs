pub(super) mod tool_grab;
pub(super) mod tool_select;

use bevy::{prelude::*, input::ButtonState};

use crate::types::BBTool;

use self::{tool_grab::ToolGrabMsg, tool_select::ToolSelectMsg};

use super::{input::InputMessage, frontend::FrontendMsg};

#[derive(Event)]
pub enum ToolMsg {
    Input(InputMessage),

    SwitchTool(BBTool),
    PushTool(BBTool),
    ResetToRootTool,
}

/// Tool resource, stores state related to the current selected tool
#[derive(Resource, Default)]
struct ToolResource {
    base_tool: BBTool,
    tool_stack: Vec<BBTool>,
}
impl ToolResource {
    fn current_tool(&self) -> BBTool {
        *self.tool_stack.last().unwrap_or(&self.base_tool)
    }
    fn set_base_tool(&mut self, tool: BBTool) {
        self.base_tool = tool;
    }
    fn push_tool(&mut self, tool: BBTool) {
        self.tool_stack.push(tool);
    }
    fn clear(&mut self) {
        self.tool_stack.clear();
    }
}

pub struct ToolControllerPlugin;
impl Plugin for ToolControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<BBTool>()
            .add_event::<ToolMsg>()
            // .add_plugin(SelectToolPlugin)
            // .add_plugin(GrabToolPlugin)
            // .add_plugin(BoxToolPlugin)
            .insert_resource(ToolResource::default())
            // Setup BG Hit plane
            .add_systems(PreUpdate, sys_tool_preprocess_messages)
            // .add_system(tool_system.in_set(EditorSet::ToolProcess));
        ;
    }
}

/// Preprocess step before passing inputs to the tool system.
/// Implements some hotkey behaviour
pub fn sys_tool_preprocess_messages(
    current_tool: Res<State<BBTool>>,
    mut input_reader: EventReader<InputMessage>,
    mut tool_writer: EventWriter<ToolMsg>,
) {
    for msg in input_reader.iter() {
        let mut should_pass_through = false;
        let current_tool = current_tool.into_inner().get();

        match msg {
            InputMessage::Keyboard {
                pressed,
                key,
                modifiers,
            } => match (pressed, key, modifiers.command, modifiers.shift) {
                // Click to drag around viewport with space key pressed
                (ButtonState::Pressed, KeyCode::Space, _, _) => {
                    if *current_tool != BBTool::Grab {
                        tool_writer.send(ToolMsg::PushTool(BBTool::Grab));
                    }
                }
                (ButtonState::Released, KeyCode::Space, _, _) => {
                    if *current_tool == BBTool::Grab {
                        tool_writer.send(ToolMsg::ResetToRootTool);
                    }
                }
                (ButtonState::Released, KeyCode::Key1, _, _) => {
                    tool_writer.send(ToolMsg::SwitchTool(BBTool::Select));
                }
                (ButtonState::Released, KeyCode::Key2, _, _) => {
                    tool_writer.send(ToolMsg::SwitchTool(BBTool::Box));
                }
                // Command+N: New Documnet
                // (ButtonState::Released, KeyCode::N, ButtonState::Pressed, _) => document_writer
                //     .send(DocumentMessage::Create(CreateDocumentModel {
                //         size: Vec2::new(400., 400.),
                //         ..Default::default()
                //     })),
                (_, _, _, _) => {
                    should_pass_through = true;
                }
            },
            _ => {
                should_pass_through = true;
            }
        }

        if should_pass_through {
            tool_writer.send(ToolMsg::Input(*msg));
        }
    }
}

/// This handles general tool messages as well as passes data off to the tool message handler of
/// the currently active tool.
pub fn sys_tool_handle_messages(
    cur_tool: Res<State<BBTool>>,
    mut next_tool: ResMut<NextState<BBTool>>,
    mut tool_resource: ResMut<ToolResource>,
    mut tool_reader: EventReader<ToolMsg>,
    mut frontend_writer: EventWriter<FrontendMsg>,

    mut grab_writer: EventWriter<ToolGrabMsg>,
    mut select_writer: EventWriter<ToolSelectMsg>,
) {
    let current_tool = cur_tool.get();

    let mut handle_tool_switched = |old_tool: BBTool, new_tool: BBTool| {
        match old_tool {
            BBTool::Grab => grab_writer.send(ToolGrabMsg::OnDeactivate),
            BBTool::Select => select_writer.send(ToolSelectMsg::OnDeactivate),
        }
        match old_tool {
            BBTool::Grab => grab_writer.send(ToolGrabMsg::OnActivate),
            BBTool::Select => select_writer.send(ToolSelectMsg::OnActivate),
        }
    };

    for msg in tool_reader.iter() {
        match msg {
            ToolMsg::PushTool(tool) => {
                if *tool != tool_resource.current_tool() {
                    tool_resource.push_tool(*tool);
                    let new_tool = tool_resource.current_tool();

                    next_tool.set(new_tool);
                    frontend_writer.send(FrontendMsg::SetCurrentTool(new_tool));
                    handle_tool_switched(*current_tool, new_tool);
                }
            }
            ToolMsg::ResetToRootTool => {
                tool_resource.clear();
                if *current_tool != tool_resource.current_tool() {
                    let new_tool = tool_resource.current_tool();

                    next_tool.set(new_tool);
                    frontend_writer.send(FrontendMsg::SetCurrentTool(new_tool));
                    handle_tool_switched(*current_tool, new_tool);
                }
            }
            ToolMsg::SwitchTool(tool) => {
                if *tool != tool_resource.current_tool() {
                    tool_resource.set_base_tool(*tool);
                    let new_tool = tool_resource.current_tool();

                    next_tool.set(new_tool);
                    frontend_writer.send(FrontendMsg::SetCurrentTool(new_tool));
                    handle_tool_switched(*current_tool, new_tool);
                }
            }
            ToolMsg::Input(input_msg) => {
                // println!("ToolMsg::Input({:?})", input_msg);
                match current_tool {
                    BBTool::Grab => grab_writer.send(ToolGrabMsg::Input(*input_msg)),
                    BBTool::Select => select_writer.send(ToolSelectMsg::Input(*input_msg)),
                    _ => {},
                }
            }
        }
    }
}
