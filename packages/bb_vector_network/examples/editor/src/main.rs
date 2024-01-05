mod entities;
mod tesselation;
mod tools;
mod utils;

use bb_vector_network::prelude::*;
use comfy::*;
use entities::Node;
use tesselation::{tessellate_fill, tessellate_stroke};
use tools::{InputHelper, PenTool, SelectTool, Tool, ToolTrait, ToolUpdateResult};
use utils::{screen_bottom_left_world, screen_top_left_world, ERR_TEXT_PARAMS, TEXT_PARAMS, draw_bb_error};

simple_game!(
    "BB Vector Network :: Editor",
    GameState,
    config,
    setup,
    update
);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(1200, 800),
        min_resolution: ResolutionConfig::Physical(120 * 5, 80 * 5),
        ..config
    }
}

pub struct GameState {
    tool: Tool,
    input_helper: InputHelper,
    select_tool: SelectTool,
    pen_tool: PenTool,

    nodes: Vec<Node>,
    graph: BBGraph,

    show_graph_annotations: bool,
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
            input_helper: InputHelper::default(),
            select_tool: SelectTool::default(),
            pen_tool: PenTool::default(),

            show_graph_annotations: false,
        };

        gs.rebuild_game_nodes();

        gs
    }

    pub fn rebuild_game_nodes(&mut self) {
        self.nodes = self
            .graph
            .nodes
            .keys()
            .map(|node_idx| {
                let mut n = Node::new(*node_idx, 0.3);
                let _ = n.update_from_graph(&self.graph);
                n
            })
            .collect();
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

    // Keybinds
    if is_key_released(KeyCode::Num1) {
        state.tool = Tool::Select;
    } else if is_key_released(KeyCode::Num2) {
        state.tool = Tool::Pen;
    }

    draw_text_ex(
        "[d]: Toggle dbg annotations",
        screen_bottom_left_world() - vec2(-0.1, -0.1),
        comfy::TextAlign::TopLeft,
        TEXT_PARAMS.clone(),
    );
    if is_key_released(KeyCode::D) {
        state.show_graph_annotations = !state.show_graph_annotations;
    }

    // Perform Tool updates
    let mouse_events = state.input_helper.compute_mouse_events();
    let update_result = match state.tool {
        Tool::Select => SelectTool::update(state, &mouse_events),
        Tool::Pen => PenTool::update(state, &mouse_events),
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
        Err(reason) => draw_bb_error(&format!("TOOL: {:?}", state.tool), state, &reason),
    }

    if needs_update_regions {
        match state.graph.update_regions() {
            Ok(_) => (),
            Err(reason) => draw_bb_error(&format!("REGIONS:"), state, &reason),
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
        Err(reason) => draw_text(
            &format!("Fill failed: {reason}"),
            Vec2::ZERO,
            PINK,
            TextAlign::Center,
        ),
    }
    match tessellate_stroke(&state.graph) {
        Ok(mesh) => draw_mesh(mesh),
        Err(reason) => draw_text(
            &format!("Stroke failed: {reason}"),
            Vec2::ZERO,
            PINK,
            TextAlign::Center,
        ),
    }

    if state.show_graph_annotations {
        for node in state.nodes.iter() {
            draw_text_ex(
                &format!("{}", node.node_idx),
                node.position + vec2(0.1, 0.1),
                TextAlign::BottomLeft,
                TEXT_PARAMS.clone(),
            );
        }

        for (idx, edge) in state.graph.edges.iter() {
            draw_arrow(edge.start_pos(&state.graph), edge.end_pos(&state.graph), 0.1, BLUE, 100);
            draw_text_ex(
                &format!("{}", idx),
                edge.t_point(&state.graph, 0.5) + vec2(0.1, 0.1),
                TextAlign::BottomLeft,
                TEXT_PARAMS.clone(),
            );
        }
    }
}
