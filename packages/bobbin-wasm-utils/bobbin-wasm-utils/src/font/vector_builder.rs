
use crate::vector::{BBVectorCommand, BBVector};

/// Builder for owned_ttf_parser::OutlineBuilder
///
/// * `commands`: 
pub struct BBFaceVectorBuilder {
    commands: Vec<BBVectorCommand>,
}

impl BBFaceVectorBuilder {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn build(self) -> BBVector {
        BBVector {
            commands: self.commands,
        }
    }
}

impl owned_ttf_parser::OutlineBuilder for BBFaceVectorBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(BBVectorCommand::M { x, y })
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(BBVectorCommand::L { x, y })
    }
    fn quad_to(&mut self, c0x: f32, c0y: f32, x: f32, y: f32) {
        self.commands.push(BBVectorCommand::Q { c0x, c0y, x, y })
    }
    fn curve_to(&mut self, c0x: f32, c0y: f32, c1x: f32, c1y: f32, x: f32, y: f32) {
        self.commands.push(BBVectorCommand::C {
            c0x,
            c0y,
            c1x,
            c1y,
            x,
            y,
        })
    }
    fn close(&mut self) {
        self.commands.push(BBVectorCommand::Z);
    }


}
