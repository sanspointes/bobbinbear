mod tesselation;
mod entities;
mod tools;
mod utils;

use comfy::*;
use bb_vector_network::prelude::*;
use entities::Node;
use tesselation::{tessellate_fill, tessellate_stroke};
use tools::{Tool, SelectTool, ToolTrait, ToolUpdateResult};
use utils::{screen_top_left_world, TEXT_PARAMS};

simple_game!("BB Vector Network :: Editor", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(1200, 800),
        min_resolution: ResolutionConfig::Physical(120 * 5, 80 * 5),
        ..config
    }
}

pub struct GameState {
    tool: Tool,
    select_tool: SelectTool,

    nodes: Vec<Node>,
    graph: BBGraph,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        let mut g = BBGraph::new();
        let (_, first_edge) = g.line(Vec2::new(0., 0.), Vec2::new(5., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5., 5.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(0., 5.));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());
        let (_, _) = g.line_from(edge.start_idx(), Vec2::new(-5., 5.));

        let mut gs = Self {
            nodes: vec![],
            graph: g,
            tool: Tool::Select,
            select_tool: SelectTool::default(),
        };

        gs.rebuild_game_nodes();

        gs
    }

    pub fn rebuild_game_nodes(&mut self) {
        self.nodes = self.graph.nodes.keys().map(|node_idx| {
            let mut n = Node::new(*node_idx, 0.3);
            let _ = n.update_from_graph(&self.graph);
            n
        }).collect();
    }

    pub fn node(&self, node_idx: BBNodeIndex) -> Option<&Node> {
        self.nodes.iter().find(|n| n.node_idx == node_idx)
    }
    pub fn node_mut(&mut self, node_idx: BBNodeIndex) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.node_idx == node_idx)
    }

    pub fn intersect_nodes(&self, p: Vec2) -> Option<&Node> {
        self.nodes.iter().find(|n| n.intersects(&p))
    }
}

fn setup(state: &mut GameState, c: &mut EngineContext) {
    c.load_fonts_from_bytes(&[(
        "comfy-font",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/Roboto-Regular.ttf"
        )),
    )]);

    state.graph.update_regions().unwrap();
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
        draw_text_ex(
            "bb_vector_network: [1]: Select tool, [2]: Pen tool",
            screen_top_left_world() - vec2(-0.1, 0.1),
            comfy::TextAlign::TopLeft,
            TEXT_PARAMS.clone(),
        );


    let update_result = match state.tool {
        Tool::Select => {
            SelectTool::update(state)
        }
    };

    let mut needs_update_regions = false;
    match update_result {
        Ok(ToolUpdateResult::Noop) => (),
        Ok(ToolUpdateResult::RegenerateMesh) => {
            needs_update_regions = true;
        }
        Ok(ToolUpdateResult::RegenerateAll) => {
            state.rebuild_game_nodes();
            needs_update_regions = true;
        }
        Err(reason) => {
            draw_text(&format!("{:?} tool error: {:?}", state.tool, reason), Vec2::ZERO, RED, TextAlign::Center);
        }
    }

    if needs_update_regions {
        match state.graph.update_regions() {
            Ok(_) => (),
            Err(reason) => {
                draw_text(&format!("Error updating regions: {reason}"), Vec2::ZERO, RED, TextAlign::Center);
            }
        }
    }

    draw(state, _c);
}

fn draw(state: &GameState, _c: &mut EngineContext) {
    for node in state.nodes.iter() {
        node.draw(state, 50);
    }


    match tessellate_fill(&state.graph) {
        Ok(mesh) => draw_mesh(mesh),
        Err(reason) => {
            draw_text(&format!("Fill failed: {reason}"), Vec2::ZERO, PINK, TextAlign::Center)
        },
    }
    match tessellate_stroke(&state.graph) {
        Ok(mesh) => draw_mesh(mesh),
        Err(reason) => draw_text(&format!("Stroke failed: {reason}"), Vec2::ZERO, PINK, TextAlign::Center),
    }
}
