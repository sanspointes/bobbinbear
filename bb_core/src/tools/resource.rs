use bevy::ecs::system::Resource;

use super::types::BobbinTool;

#[derive(Default, Resource)]
pub struct ToolResource {
    base_tool: BobbinTool,
    tool_stack: Vec<BobbinTool>,
}
impl ToolResource {
    pub fn push_tool_stack(&mut self, tool: BobbinTool) {
        // Skip if already same tool
        if let Some(last) = self.tool_stack.last() {
            if *last == tool {
                return;
            }
        }
        self.tool_stack.push(tool);
    }

    pub fn pop_tool_stack(&mut self, tool: BobbinTool) {
        // Skip if different tool
        if let Some(last) = self.tool_stack.last() {
            if *last != tool {
                return;
            }
        }

        self.tool_stack.push(tool);
    }

    pub fn set_base_tool(&mut self, tool: BobbinTool) {
        self.base_tool = tool;
    }

    pub fn get_current_tool(&self) -> BobbinTool {
        self.tool_stack.last().copied().unwrap_or(self.base_tool)
    }
}
